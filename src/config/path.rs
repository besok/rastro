use std::fs;
use std::path::PathBuf;
use crate::config::{Configuration, ConfigurationError};


pub fn rastro_config_path() -> Result<PathBuf, ConfigurationError> {
    dirs::home_dir().map(|p| p.join(".rastro").join("config.toml"))
        .ok_or(ConfigurationError("Could not find home directory".to_owned()))
}


#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use std::fs;
    use crate::config::Configuration;
    use crate::config::path::{rastro_config_path};

    #[test]
    fn manual_test_path() {
        let location = rastro_config_path();
        assert!(location.is_ok());
        let location = location.unwrap();

        let path = Configuration::default().to_toml_file(&location);
        assert!(path.is_ok());
    }

    #[test]
    fn test_path_default() {
        let location = current_dir()
            .unwrap()
            .join("src")
            .join("config")
            .join("config.toml");
        println!("{:?}", location);
        let path = Configuration::default().to_toml_file(&location);
        assert!(path.is_ok());

        let file = Configuration::from_toml_file(&location);
        assert!(file.is_ok());
        let file = file.unwrap();
        assert_eq!(file.to_toml_str(), Configuration::default().to_toml_str());

        fs::remove_file(location).unwrap();
    }
}
