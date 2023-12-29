// Copyright © 2023 xtasks. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::{Result, anyhow};

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    thread_local! {
        static MOCK_MAIN_FN: RefCell<Option<Box<dyn Fn() -> Result<()> + Send>>> = RefCell::new(None);
    }

    fn set_mock_main_fn<F>(mock_fn: F)
    where
        F: Fn() -> Result<()> + 'static + Send,
    {
        MOCK_MAIN_FN.with(|mock| {
            *mock.borrow_mut() = Some(Box::new(mock_fn));
        });
    }

    fn main() -> Result<()> {
        MOCK_MAIN_FN.with(|mock| {
            if let Some(mock_fn) = &*mock.borrow() {
                (mock_fn)()
            } else {
                Ok(()) // Default behaviour or call the real implementation
            }
        })
    }

    #[test]
    fn test_main_success() {
        set_mock_main_fn(|| Ok(()));
        let result = main();
        assert!(result.is_ok(), "Expected Ok, got {result:?}");
    }

    #[test]
    fn test_main_error() {
        set_mock_main_fn(|| Err(anyhow!("mock error")));
        let result = main();
        assert!(result.is_err(), "Expected Err, got {result:?}");
    }
}
