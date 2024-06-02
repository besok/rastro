//! Configuration for Rastro
#![allow(dead_code)]

use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;
use toml::{Value};
use crate::config::values::*;

mod macros;
mod cast;
pub mod path;
pub mod values;

#[derive(Debug)]
pub struct ConfigurationError(String);

impl ConfigurationError {
    pub fn add_prefix(&mut self, prefix: String) {
        self.0 = format!("{}: {}", prefix, self.0);
    }
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Configuration error: {}", self.0)
    }
}


impl std::error::Error for ConfigurationError {}

/// Configuration for Rastro
/// This struct contains all the configuration options for Rastro.
///
/// # Example
///
/// ## How to work with the fields of the config
/// ```
/// use rastro::config::Configuration;///
/// let toml = r#"
/// [logger]
/// log_level = "DEBUG"
/// log_warnings = false
/// "#;
/// let cfg = Configuration::from_toml_str(toml.to_string());
/// assert!(cfg.is_ok());
/// let mut cfg = cfg.unwrap();
/// assert_eq!(cfg.logger.log_level.get(), "DEBUG");
///
/// cfg.logger.log_level.set("INFO".to_owned());
/// assert_eq!(cfg.logger.log_level.get(), "INFO");
/// ```
///
/// ## Where to store the configuration.
/// The default location is `~/.rastro/config.toml`.
/// To create the config file, you can use the `create_rastro_config` function
/// or provide the path to the file.
/// ```
/// use rastro::config::path::{rastro_config_path};
/// use rastro::config::Configuration;
/// let location = rastro_config_path();
/// assert!(location.is_ok());
/// let location = location.unwrap();
/// let path = Configuration::default().to_toml_file(&location);
/// assert!(path.is_ok());
/// ```
///
/// ```no_run
/// use std::env::current_dir;
/// use rastro::config::path::{rastro_config_path};
/// use rastro::config::Configuration;
/// let location = current_dir()
///             .unwrap()
///             .join("src")
///             .join("config")
///             .join("config.toml");
///         println!("{:?}", location);
/// let path = Configuration::default().to_toml_file(&location);
/// assert!(path.is_ok());
///
#[derive(Debug, Default)]
pub struct Configuration {
    pub logger: Logger,
    pub console: Console,
    pub visualization: Visualization,
    pub iers: Iers,
    pub data: Data,
}

impl Configuration {
    pub fn to_toml_str(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.logger.to_toml_section());
        s.push_str(&self.console.to_toml_section());
        s.push_str(&self.visualization.to_toml_section());
        s.push_str(&self.iers.to_toml_section());
        s.push_str(&self.data.to_toml_section());
        s
    }

    pub fn from_toml_str(toml_str: String) -> Result<Configuration, ConfigurationError> {
        let res: Value = toml::from_str(&toml_str).map_err(|e| ConfigurationError(format!("{:?}", e)))?;
        if let Value::Table(tbl) = res {
            let logger = Logger::from_toml_section(&tbl)?;
            let console = Console::from_toml_section(&tbl)?;
            let visualization = Visualization::from_toml_section(&tbl)?;
            let iers = Iers::from_toml_section(&tbl)?;
            let data = Data::from_toml_section(&tbl)?;
            Ok(Configuration {
                logger,
                console,
                visualization,
                iers,
                data,
            })
        } else {
            Err(ConfigurationError("Invalid TOML file. The toml file should have the sections".to_owned()))
        }
    }
    pub fn from_toml_file(file_path: &PathBuf) -> Result<Configuration, ConfigurationError> {
        let toml_str = std::fs::read_to_string(file_path)
            .map_err(|e| ConfigurationError(format!("{:?}", e)))?;
        Configuration::from_toml_str(toml_str)
    }

    pub fn to_toml_file(&self, file_path: &PathBuf) -> Result<(), ConfigurationError> {
        if !file_path.exists() {
            if let Some(parent) = file_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)
                        .map_err(|e| ConfigurationError(format!("{:?}", e)))?;
                }
            } else {
                Err(ConfigurationError("Could not find parent directory".to_owned()))?;
            }
        }
        let cfg_as_str = self.to_toml_str();
        fs::write(file_path, cfg_as_str.as_bytes())
            .map_err(|e| ConfigurationError(format!("{:?}", e)))?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::config::Configuration;

    #[test]
    fn smoke() {
        let mut cfg = Configuration::default();
        assert_eq!(cfg.logger.log_level.get(), "INFO");
        cfg.logger.log_level.set("DEBUG".to_owned());
        assert_eq!(cfg.logger.log_level.get(), "DEBUG");
    }

    #[test]
    fn to_toml() {
        let cfg = Configuration::default();
        let toml = cfg.to_toml_str();
        println!("{}", toml);
    }

    #[test]
    fn from_toml_smoke() {
        let toml = r#"
[logger]
log_level = "DEBUG"
log_warnings = false
"#;

        let cfg = Configuration::from_toml_str(toml.to_string());
        assert!(cfg.is_ok());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.logger.log_level.get(), "DEBUG");
        assert_eq!(cfg.logger.log_warnings.get(), &false);
    }

    #[test]
    fn from_toml_wrong_section_format() {
        let toml = r#"
                        logger = 1
                        "#;

        let cfg = Configuration::from_toml_str(toml.to_string());
        assert!(cfg.is_err());
    }

    #[test]
    fn from_toml_wrong_type() {
        let toml = r#"
                        [logger]
                        log_level = 1
                        "#;

        assert!(Configuration::from_toml_str(toml.to_string()).is_err());
    }
}