use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub enum Chain {
  Avalanche,
  Bitcoin,
  Ethereum,
}

impl Chain {
  pub fn as_str(&self) -> &'static str {
    match self {
      Chain::Avalanche => "avalanche",
      Chain::Bitcoin => "bitcoin",
      Chain::Ethereum => "ethereum",
    }
  }
}

impl TryFrom<String> for Chain {
  type Error = String;

  fn try_from(s: String) -> Result<Self, Self::Error> {
    match s.to_lowercase().as_str() {
      "avalanche" => Ok(Self::Avalanche),
      "bitcoin" => Ok(Self::Bitcoin),
      "ethereum" => Ok(Self::Ethereum),
      other => Err(format!("{} is not a supported environment", other)),
    }
  }
}
