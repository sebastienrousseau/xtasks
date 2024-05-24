//! This crate provides tasks for analysing the bloat of a package.

use clap::{Arg, ArgAction, ArgMatches, Command};
use xtasks::tasks::bloat::deps;

fn get_matches(args: Vec<&str>) -> ArgMatches {
    Command::new("test")
        .arg(
            Arg::new("dry-run")
                .short('n')
                .long("dry-run")
                .action(ArgAction::SetTrue)
                .id("dry-run")
                .help("Show what would be done without doing it"),
        )
        .arg(
            Arg::new("package")
                .short('p')
                .long("package")
                .value_name("PACKAGE")
                .help("The package to analyze for bloat")
                .required(true),
        )
        .get_matches_from(args)
}

// #[allow(clippy::redundant_pub_crate)]
pub(crate) fn main() {
    let matches = get_matches(vec!["test", "--package", ""]);
    let result = deps("", &matches);
    if let Err(e) = result {
        eprintln!("Error analysing package dependencies: {e}");
    }
}
