use xtasks::tasks::main_with_args;

#[test]
fn test_main_with_args() {
    let test_cases = vec![
        (
            vec!["xtasks", "install"],
            "Main with args should succeed for install",
        ),
        (
            vec!["xtasks", "lint"],
            "Main with args should succeed for lint",
        ),
        (
            vec!["xtasks", "format"],
            "Main with args should succeed for format",
        ),
        (
            vec!["xtasks", "release"],
            "Main with args should succeed for release",
        ),
        (
            vec!["xtasks", "benchmark"],
            "Main with args should succeed for benchmark",
        ),
        (
            vec!["xtasks", "security"],
            "Main with args should succeed for security",
        ),
        (
            vec!["xtasks", "config"],
            "Main with args should succeed for config",
        ),
        (
            vec!["xtasks", "update"],
            "Main with args should succeed for update",
        ),
        (
            vec!["xtasks", "clean"],
            "Main with args should succeed for clean",
        ),
        (
            vec!["xtasks", "init"],
            "Main with args should succeed for init",
        ),
        (
            vec!["xtasks", "vars"],
            "Main with args should succeed for vars",
        ),
        (vec!["xtasks", "ci"], "Main with args should succeed for ci"),
        (
            vec!["xtasks", "coverage"],
            "Main with args should succeed for coverage",
        ),
        (
            vec!["xtasks", "docs"],
            "Main with args should succeed for docs",
        ),
        (
            vec!["xtasks", "powerset"],
            "Main with args should succeed for powerset",
        ),
        (
            vec!["xtasks", "bloat-deps", "-p", "package_name"],
            "Main with args should succeed for bloat-deps",
        ),
        (
            vec!["xtasks", "bloat-time", "-p", "package_name"],
            "Main with args should succeed for bloat-time",
        ),
    ];

    for (args, msg) in test_cases {
        let args =
            args.iter().map(|&s| s.to_string()).collect::<Vec<_>>();
        let result = main_with_args(&args);
        assert!(result.is_ok(), "{}", msg);
    }
}

#[test]
fn test_main_with_args_dry_run() {
    let test_cases = vec![
        (
            vec!["xtasks", "install", "--dry-run"],
            "Main with args should succeed for install dry run",
        ),
        (
            vec!["xtasks", "lint", "--dry-run"],
            "Main with args should succeed for lint dry run",
        ),
        (
            vec!["xtasks", "format", "--dry-run"],
            "Main with args should succeed for format dry run",
        ),
        (
            vec!["xtasks", "release", "--dry-run"],
            "Main with args should succeed for release dry run",
        ),
        (
            vec!["xtasks", "benchmark", "--dry-run"],
            "Main with args should succeed for benchmark dry run",
        ),
        (
            vec!["xtasks", "security", "--dry-run"],
            "Main with args should succeed for security dry run",
        ),
        (
            vec!["xtasks", "config", "--dry-run"],
            "Main with args should succeed for config dry run",
        ),
        (
            vec!["xtasks", "update", "--dry-run"],
            "Main with args should succeed for update dry run",
        ),
        (
            vec!["xtasks", "clean", "--dry-run"],
            "Main with args should succeed for clean dry run",
        ),
        (
            vec!["xtasks", "init", "--dry-run"],
            "Main with args should succeed for init dry run",
        ),
        (
            vec!["xtasks", "ci", "--dry-run"],
            "Main with args should succeed for ci dry run",
        ),
        (
            vec!["xtasks", "coverage", "--dry-run"],
            "Main with args should succeed for coverage dry run",
        ),
        (
            vec!["xtasks", "docs", "--dry-run"],
            "Main with args should succeed for docs dry run",
        ),
        (
            vec!["xtasks", "powerset", "--dry-run"],
            "Main with args should succeed for powerset dry run",
        ),
        (
            vec![
                "xtasks",
                "bloat-deps",
                "-p",
                "package_name",
                "--dry-run",
            ],
            "Main with args should succeed for bloat-deps dry run",
        ),
        (
            vec![
                "xtasks",
                "bloat-time",
                "-p",
                "package_name",
                "--dry-run",
            ],
            "Main with args should succeed for bloat-time dry run",
        ),
    ];

    for (args, msg) in test_cases {
        let args =
            args.iter().map(|&s| s.to_string()).collect::<Vec<_>>();
        let result = main_with_args(&args);
        assert!(result.is_ok(), "{}", msg);
    }
}
