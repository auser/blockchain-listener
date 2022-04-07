use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
  pub blockchains: HashMap<String, ChainSetting>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ChainSetting {
  pub host: String,
  pub endpoint: String,
  pub port: u16,
  pub protocol: String,
}

pub fn get_configuration(base_path: Option<PathBuf>) -> Result<Settings, config::ConfigError> {
  let base_path = match base_path {
    None => std::env::current_dir().expect("Failed to determine the current directory"),
    Some(path) => path,
  };
  let configuration_directory = base_path.join("configuration");
  let environment: Environment = std::env::var("APP_ENVIRONMENT")
    .unwrap_or_else(|_| "local".to_string())
    .try_into()
    .expect("Unable to parse environment");

  let settings = config::Config::builder()
    .add_source(config::File::from(configuration_directory.join("base")).required(true))
    .add_source(
      config::File::from(configuration_directory.join(environment.as_str())).required(false),
    )
    .build()
    .unwrap();

  settings.try_deserialize::<Settings>()
}

pub enum Environment {
  Local,
  Production,
}

impl Environment {
  pub fn as_str(&self) -> &'static str {
    match self {
      Environment::Local => "local",
      Environment::Production => "production",
    }
  }
}

impl TryFrom<String> for Environment {
  type Error = String;

  fn try_from(s: String) -> Result<Self, Self::Error> {
    match s.to_lowercase().as_str() {
      "local" => Ok(Self::Local),
      "production" => Ok(Self::Production),
      other => Err(format!("{} is not a supported environment", other)),
    }
  }
}
