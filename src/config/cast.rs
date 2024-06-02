use toml::{Value};
use crate::config::ConfigurationError;

/// Trait to convert a toml value to a specific type that The configuration expects.
/// This trait is implemented for the types that are expected in the configuration.
/// The trait is used in the `from_toml_value` method of the `var!` macro.
pub trait FromTomlValue {
    fn from_toml_value(toml_value: &Value) -> Result<Self, ConfigurationError> where Self: Sized;
}

impl FromTomlValue for String {
    fn from_toml_value(toml_value: &Value) -> Result<Self, ConfigurationError> {
        toml_value
            .as_str()
            .map(|s| s.to_owned())
            .ok_or(ConfigurationError("Invalid string".to_owned()))
    }
}

impl FromTomlValue for i64 {
    fn from_toml_value(toml_value: &Value) -> Result<Self, ConfigurationError> {
        toml_value
            .as_integer()
            .ok_or(ConfigurationError("Invalid integer".to_owned()))
    }
}

impl FromTomlValue for f64 {
    fn from_toml_value(toml_value: &Value) -> Result<Self, ConfigurationError> {
        toml_value
            .as_float()
            .ok_or(ConfigurationError("Invalid float".to_owned()))
    }
}

impl FromTomlValue for i32 {
    fn from_toml_value(toml_value: &Value) -> Result<Self, ConfigurationError> {
        toml_value
            .as_integer()
            .map(|i| i as i32)
            .ok_or(ConfigurationError("Invalid unsigned integer".to_owned()))
    }
}

impl FromTomlValue for bool {
    fn from_toml_value(toml_value: &Value) -> Result<Self, ConfigurationError> {
        toml_value
            .as_bool()
            .ok_or(ConfigurationError("Invalid boolean".to_owned()))
    }
}