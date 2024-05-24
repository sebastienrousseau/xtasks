#[cfg(test)]
mod tests {
    use clap::Arg;
    use clap::ArgAction;
    use clap::ArgMatches;
    use clap::Command;
    use rlg::log_level::LogLevel;
    use xtasks::tasks::bloat::deps;
    use xtasks::tasks::bloat::format_analysis_results;
    use xtasks::tasks::bloat::handle_dry_run;
    use xtasks::tasks::bloat::log_and_return_error;
    use xtasks::tasks::bloat::time;

    fn get_matches(args: Vec<&str>) -> ArgMatches {
        Command::new("test")
            .arg(
                Arg::new("dry-run")
                    .short('n')
                    .long("dry-run")
                    .action(ArgAction::SetTrue)
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

    #[test]
    fn test_log_and_return_error() {
        let err = log_and_return_error(
            &LogLevel::ERROR,
            "Test Context",
            "Test error message".to_string(),
            "Additional context".to_string(),
        );
        assert_eq!(err.to_string(), "Additional context");
    }

    #[test]
    fn test_handle_dry_run() {
        let matches = get_matches(vec![
            "test",
            "--dry-run",
            "--package",
            "testpackage",
        ]);
        let result = handle_dry_run(&matches, "Test Context");
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_handle_dry_run_disabled() {
        let matches =
            get_matches(vec!["test", "--package", "testpackage"]);
        let result = handle_dry_run(&matches, "Test Context");
        assert!(result.is_none());
    }

    #[test]
    fn test_deps_package_name_empty() {
        let matches = get_matches(vec!["test", "--package", ""]);
        let result = deps("", &matches);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed due to empty package name"
        );
    }

    #[test]
    fn test_deps_invalid_package_name() {
        let matches =
            get_matches(vec!["test", "--package", "invalid*name"]);
        let result = deps("invalid*name", &matches);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed due to invalid characters in package name"
        );
    }

    #[test]
    fn test_time_dry_run() {
        let matches = get_matches(vec![
            "test",
            "--dry-run",
            "--package",
            "mypackage",
        ]);
        let result = time("mypackage", &matches);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_analysis_results() {
        let raw_output = "crate1, 10KB, 1s\ncrate2, 20KB, 2s\n";
        let result = format_analysis_results(raw_output).unwrap();
        assert_eq!(
            result,
            "Dependency Analysis Results:\ncrate1: Size = 10KB, Time = 1s\ncrate2: Size = 20KB, Time = 2s\n"
        );
    }

    #[test]
    fn test_format_analysis_results_invalid_format() {
        let raw_output = "crate1, 10KB\ncrate2, 20KB, 2s\n";
        let result = format_analysis_results(raw_output);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid line format: crate1, 10KB"
        );
    }
}
