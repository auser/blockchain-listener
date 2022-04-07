use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockchainError {
  #[error("Error connecting to {0}")]
  ConnectionError(String),
  #[error("Send message error")]
  SendMessageError(String),
  #[error("Uninitialized")]
  Uninitialized,
  #[error(transparent)]
  Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Error, Debug)]
pub enum ChainError {
  #[error("Chain {0} not supported")]
  UnsupportedChain(String),
  #[error("Unable to initialize chain {0}")]
  UnableToInitialize(String),
}
