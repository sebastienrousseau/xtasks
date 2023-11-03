#[cfg(test)]
mod tests {
    use xtasks::tasks::{dev_with_command, install, main_with_args};

    #[test]
    fn test_dev() {
        let result = dev_with_command("echo");
        assert!(result.is_ok());
    }

    // #[test]
    // fn test_install() {
    //     let result = install();
    //     assert!(result.is_ok());
    // }

    #[test]
    fn test_main_with_vars_command() {
        let args = vec!["xtask", "vars"];
        let result = main_with_args(
            &args
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>(),
        );
        assert!(result.is_ok());
    }
}
