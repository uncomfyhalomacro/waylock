use serde::Deserialize;
use xdg::BaseDirectories;

use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::result::Result;

#[derive(Debug)]
pub enum ConfigError {
    NotFound,
    Io(io::Error),
    Toml(toml::de::Error),
}

// Special case for NotFound since if there is no config file we simply use the defaults
impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        if err.kind() == io::ErrorKind::NotFound {
            Self::NotFound
        } else {
            Self::Io(err)
        }
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        Self::Toml(err)
    }
}

impl error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::NotFound => None,
            Self::Io(err) => err.source(),
            Self::Toml(err) => err.source(),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "Couldn't find the config file"),
            Self::Io(err) => write!(f, "I/O error reading the config file: {}", err),
            Self::Toml(err) => write!(f, "TOML error reading the config file: {}", err),
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub colors: Colors,
}

#[derive(Deserialize)]
pub struct Colors {
    pub init_color: Option<u32>,
    pub input_color: Option<u32>,
    pub fail_color: Option<u32>,
}

impl Config {
    /// Find and read the config file if it exists. If $XDG_CONFIG_HOME is not set, the xdg crate will
    /// properly default to $HOME/.config
    pub fn new(path_override: Option<&str>) -> Result<Self, ConfigError> {
        if let Some(path) = path_override {
            let config = fs::read_to_string(Path::new(path))?;
            Ok(toml::from_str(&config)?)
        } else if let Some(config_file) = BaseDirectories::with_prefix("waylock")
            .ok()
            .and_then(|base_dirs| base_dirs.find_config_file("waylock.toml"))
        {
            let config = fs::read_to_string(config_file)?;
            Ok(toml::from_str(&config)?)
        } else {
            Err(ConfigError::NotFound)
        }
    }
}