//! Configuration for Rastro
#![allow(dead_code)]

use std::fmt::{Display, Formatter};
use toml::{Value};
use crate::config::values::*;

mod macros;
pub mod path;
pub mod values;

#[derive(Debug)]
pub struct ConfigurationError(String);

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Configuration error: {}", self.0)
    }
}

impl std::error::Error for ConfigurationError {}

#[derive(Debug, Default)]
struct Configuration {
    pub logger: Logger,
    pub console: Console,
    pub visualization: Visualization,
    pub iers: Iers,
    pub data: Data,
}

impl Configuration {
    pub fn to_toml_file(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.logger.to_toml_section());
        s.push_str(&self.console.to_toml_section());
        s.push_str(&self.visualization.to_toml_section());
        s.push_str(&self.iers.to_toml_section());
        s.push_str(&self.data.to_toml_section());
        s
    }

    pub fn from_toml_str(toml_str: String) -> Result<Configuration, ConfigurationError> {
        let res: Value = toml::from_str(&toml_str).map_err(|e| ConfigurationError(format!("{:?}",e)))?;
        if let Value::Table(tbl) = res {
            let logger = Logger::from_toml_table(&tbl);
            let console = Console::from_toml_table(&tbl);
            let visualization = Visualization::from_toml_table(&tbl);
            let iers = Iers::from_toml_table(&tbl);
            let data = Data::from_toml_table(&tbl);
            Ok(Configuration {
                logger,
                console,
                visualization,
                iers,
                data,
            })
        } else {
            Err(ConfigurationError("Invalid TOML file".to_owned()))
        }

        Ok(Configuration::default())
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
        let toml = cfg.to_toml_file();
        println!("{}", toml);
    }
}