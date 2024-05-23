#[cfg(test)]
mod tests {
    use xtasks::tasks::powerset::{Powerset, PowersetBuilder};

    /// Tests the creation of a `Powerset` instance with default values.
    ///
    /// # Steps
    /// 1. Create a `Powerset` instance using the default constructor.
    /// 2. Assert that the `depth` field is set to its default value of 2.
    ///
    /// # Expected Outcome
    /// The `Powerset` instance is created successfully with the `depth` field set to 2.
    #[test]
    fn test_default_powerset() {
        let powerset = PowersetBuilder::default().build().unwrap();
        assert_eq!(powerset.depth, 2);
        assert!(!powerset.exclude_no_default_features);
    }

    /// Tests the creation of a `Powerset` instance with a custom `depth` value.
    ///
    /// # Steps
    /// 1. Create a `Powerset` instance using the builder pattern, setting the `depth` to 3.
    /// 2. Assert that the `depth` field is set to 3.
    ///
    /// # Expected Outcome
    /// The `Powerset` instance is created successfully with the `depth` field set to 3.
    #[test]
    fn test_custom_powerset() {
        let powerset =
            PowersetBuilder::default().depth(3).build().unwrap();
        assert_eq!(powerset.depth, 3);
    }

    /// Tests specific functionality of the `Powerset` struct.
    ///
    /// # Steps
    /// 1. Use the `PowersetBuilder` to create a new `Powerset` instance, setting the `depth` to 3.
    /// 2. Assert that the `depth` field of the created `Powerset` instance is correctly set to 3.
    ///
    /// # Expected Outcome
    /// The `Powerset` instance is created successfully with the specified `depth` value,
    /// and the `depth` field is set correctly.
    #[test]
    fn test_powerset_functionality() {
        let powerset =
            PowersetBuilder::default().depth(3).build().unwrap();
        assert_eq!(powerset.depth, 3);
    }

    /// Tests the serialization and deserialization of the `Powerset` struct.
    ///
    /// # Steps
    /// 1. Create a `Powerset` instance and serialize it to a JSON string.
    /// 2. Deserialize the JSON string back into a `Powerset` instance.
    /// 3. Assert that the original and deserialized instances are equal.
    ///
    /// # Expected Outcome
    /// The serialization and deserialization processes complete successfully,
    /// and the original and deserialized `Powerset` instances are equal.
    #[test]
    fn test_serialization() {
        let powerset =
            PowersetBuilder::default().depth(3).build().unwrap();
        let serialized = serde_json::to_string(&powerset).unwrap();
        let deserialized: Powerset =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(powerset, deserialized);
    }
}
