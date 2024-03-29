pub use config::ConfigError;
use config::{Config, Value};
use serde::Deserialize;
use std::{collections::HashMap, fs};

pub trait Configurator {
    fn get<'de, T: Deserialize<'de>>(&'de self, key: &'de str) -> Result<T, ConfigError>;
    fn get_str(&self, key: &str) -> Result<String, ConfigError>;
    fn get_int(&self, key: &str) -> Result<i64, ConfigError>;
    fn get_float(&self, key: &str) -> Result<f64, ConfigError>;
    fn get_bool(&self, key: &str) -> Result<bool, ConfigError>;
    fn get_table(&self, key: &str) -> Result<HashMap<String, Value>, ConfigError>;
    fn get_array(&self, key: &str) -> Result<Vec<Value>, ConfigError>;
    fn try_into<'de, T: Deserialize<'de>>(self) -> Result<T, ConfigError>;
}

pub fn new(path: &str, prefix: &str) -> Result<impl Configurator, ConfigError> {
    let mut config_builder = Config::builder();
    let paths = fs::read_dir(path)
        .map_err(|_| ConfigError::Message(format!("Could not read directory at {}", path)))?
        .filter_map(|result| result.ok())
        .map(|de| de.path());
    for path in paths {
        config_builder = match path.as_path().to_str() {
            Some(s) => {
                if s.ends_with(".yaml") {
                    config_builder.add_source(config::File::with_name(s))
                } else {
                    config_builder
                }
            }
            None => config_builder,
        };
    }
    config_builder
        .add_source(config::Environment::with_prefix(prefix).separator("_"))
        .build()
}

impl Configurator for Config {
    fn get<'de, T: Deserialize<'de>>(&'de self, key: &'de str) -> Result<T, ConfigError> {
        Config::get(self, key)
    }
    fn get_str(&self, key: &str) -> Result<String, ConfigError> {
        Config::get_string(self, key)
    }
    fn get_int(&self, key: &str) -> Result<i64, ConfigError> {
        Config::get_int(self, key)
    }
    fn get_float(&self, key: &str) -> Result<f64, ConfigError> {
        Config::get_float(self, key)
    }
    fn get_bool(&self, key: &str) -> Result<bool, ConfigError> {
        Config::get_bool(self, key)
    }
    fn get_table(&self, key: &str) -> Result<HashMap<String, Value>, ConfigError> {
        Config::get_table(self, key)
    }
    fn get_array(&self, key: &str) -> Result<Vec<Value>, ConfigError> {
        Config::get_array(self, key)
    }
    fn try_into<'de, T: Deserialize<'de>>(self) -> Result<T, ConfigError> {
        Config::try_deserialize(self)
    }
}
