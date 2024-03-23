use memflow::prelude::v1::*;
use serde::{Deserialize, Serialize};

// see https://github.com/serde-rs/serde/issues/368
fn default_string_info() -> String {
    "info".to_string()
}
fn default_bool_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub connector: String,
    #[serde(default)]
    pub args: String,

    #[serde(default = "default_string_info")]
    pub log_level: String,

    // TODO: expose caching options (lifetimes, etc)
    #[serde(default = "default_bool_true")]
    pub parse_sections: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            connector: String::new(),
            args: String::new(),

            log_level: "info".to_string(),

            parse_sections: false,
        }
    }
}

pub struct Settings {
    config: Config,
}

impl Settings {
    /// Loads the current config from the ~/.config/memflow/reclass.toml file.
    pub fn new() -> Self {
        // load config file
        let home_dir = home::home_dir().expect("unable to get home dir");
        let memflow_config_dir = home_dir.join(".config").join("memflow");
        let config = if let Ok(configstr) =
            std::fs::read_to_string(memflow_config_dir.join("reclass.toml"))
        {
            toml::from_str::<Config>(&configstr).unwrap_or_default()
        } else {
            Config::default()
        };

        Self { config }
    }

    /// Saves the current configuration to the ~/.config/memflow/reclass.toml file.
    pub fn persist(&self) -> Result<()> {
        let home_dir = home::home_dir().expect("unable to get home dir");
        let memflow_config_dir = home_dir.join(".config").join("memflow");
        let configstr = toml::to_string_pretty(&self.config).map_err(|_| {
            Error(ErrorOrigin::Other, ErrorKind::Configuration)
                .log_error("unable to serialize config")
        })?;
        std::fs::write(memflow_config_dir.join("reclass.toml"), configstr).map_err(|_| {
            Error(ErrorOrigin::Other, ErrorKind::NotFound).log_error("unable to write config file")
        })?;
        Ok(())
    }

    /// Retrieves the current config
    pub fn config(&self) -> Config {
        self.config.clone()
    }

    /// Displays the configuration UI to the user and updates the config
    /// This function blocks until the user clicks the "Ok" button.
    pub fn configure(&mut self) {
        let inventory = Inventory::scan();
        let connectors: Vec<String> = inventory
            .available_connectors()
            .iter()
            .map(|c|c.to_owned())
            .collect::<Vec<_>>();

        let mut connector_idx = connectors
            .iter()
            .enumerate()
            .find(|(_, c)| c.as_str() == self.config.connector)
            .map(|(i, _)| i as i32)
            .unwrap_or_default();
        let mut connector_args = self.config.args.clone();
        let mut log_level_idx = match self.config.log_level.to_lowercase().as_ref() {
            "off" => 0,
            "error" => 1,
            "warn" => 2,
            "info" => 3,
            "debug" => 4,
            "trace" => 5,
            _ => 0,
        };
        let mut parse_sections = self.config.parse_sections;
    }
}
