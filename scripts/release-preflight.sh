#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Release preflight: every version-bearing file agrees with Cargo.toml.
#
#   scripts/release-preflight.sh            # on any branch
#   scripts/release-preflight.sh v0.0.7     # before tagging v0.0.7
#
# Checks Cargo.toml's version against the Cargo.lock entries of the
# workspace's own packages, every `<crate> = "x.y.z"` or
# `<crate> = { version = "x.y.z" ... }` install snippet in the
# Markdown docs (CHANGELOG and release notes are history, not
# snippets), CITATION.cff and Tauri configs when present, and that
# CHANGELOG.md has a `## [x.y.z]` section. With a tag argument it also
# requires the tag to be v<version> and that section to be dated.
#
# Written after tags shipped while Cargo.toml still named the previous
# version (sitemap-gen v0.0.6, staticweaver v0.0.7, wiserone v0.0.13,
# xtasks v0.0.3, raidhos v0.0.4).
set -euo pipefail
cd "$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

fail=0
bad() { echo "FAIL: $*"; fail=1; }
ok() { echo "ok:   $*"; }
same() { if [ "$1" = "$2" ]; then ok "$3"; else bad "$4"; fi; }

version=$(sed -n 's/^version = "\([^"]*\)".*/\1/p' Cargo.toml | head -1)
[ -n "$version" ] || { echo "FAIL: no version in Cargo.toml"; exit 1; }

# The workspace's own packages: every manifest that declares a version
# or inherits the workspace one.
mapfile -t names < <(git ls-files '*Cargo.toml' | grep -v '^fuzz/' | while read -r m; do
  awk '/^\[package\]/{p=1; next} /^\[/{p=0} p && /^name *=/{gsub(/[" ]/,""); sub(/^name=/,""); print; exit}' "$m"
done | sort -u)

if [ -f Cargo.lock ]; then
  for n in "${names[@]}"; do
    v=$(awk -v n="$n" '$0=="name = \"" n "\""{getline; gsub(/version = |"/,""); print; exit}' Cargo.lock)
    if [ -z "$v" ]; then continue; fi
    pkgver=$(git ls-files '*Cargo.toml' | while read -r m; do
      awk -v n="$n" '/^\[package\]/{p=1; next} /^\[/{p=0} p && $0 ~ "^name *= *\"" n "\""{f=1} p && f && /^version *=/{print; exit}' "$m"; done | head -1)
    case "$pkgver" in
      *workspace*|"") want=$version ;;
      *) want=$(sed -n 's/^version *= *"\([^"]*\)".*/\1/p' <<<"$pkgver") ;;
    esac
    same "$v" "$want" "Cargo.lock $n $v" "Cargo.lock has $n $v, its manifest says $want (run cargo metadata)"
  done
fi

main=${names[0]}
for n in "${names[@]}"; do grep -q "^name = \"$n\"" Cargo.toml && main=$n; done
pattern="^[[:space:]]*\`?${main}[[:space:]]*=[[:space:]]*(\\{[[:space:]]*version[[:space:]]*=[[:space:]]*)?\"=?[0-9]+\\.[0-9]+\\.[0-9]+\""
while IFS= read -r hit; do
  [ -n "$hit" ] || continue
  got=$(sed -E 's/.*"=?([0-9]+\.[0-9]+\.[0-9]+)".*/\1/' <<<"${hit#*:*:}")
  same "$got" "$version" "${hit%%:*} snippet $got" "${hit%%:*}: install snippet names $got, Cargo.toml says $version"
done < <(git ls-files '*.md' | grep -vE '^(CHANGELOG\.md|RELEASE|docs/releases/)' | xargs grep -nE "$pattern" /dev/null || true)

if [ -f CITATION.cff ]; then
  c=$(sed -n 's/^version: *//p' CITATION.cff)
  same "$c" "$version" "CITATION.cff $c" "CITATION.cff says $c, Cargo.toml says $version"
fi
while IFS= read -r f; do
  t=$(sed -n 's/^ *"version": *"\([^"]*\)".*/\1/p' "$f" | head -1)
  same "$t" "$version" "$f $t" "$f says $t, Cargo.toml says $version"
done < <(git ls-files '*tauri.conf.json')

heading=$(grep -m1 -E "^## \[v?${version//./\\.}\]" CHANGELOG.md 2>/dev/null || true)
if [ -n "$heading" ]; then ok "CHANGELOG.md: $heading"; else bad "CHANGELOG.md has no ## [$version] section"; fi

if [ -n "${1:-}" ]; then
  same "$1" "v$version" "tag $1" "tag $1 does not match Cargo.toml version $version"
  if grep -qE "^## \[v?${version//./\\.}\] - [0-9]{4}-[0-9]{2}-[0-9]{2}" CHANGELOG.md 2>/dev/null; then
    ok "CHANGELOG.md section is dated"
  else
    bad "CHANGELOG.md ## [$version] is not dated yet"
  fi
fi

if [ "$fail" != 0 ]; then exit 1; fi
echo "release preflight: every version-bearing file names $version"
