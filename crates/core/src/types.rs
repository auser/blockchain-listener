use crate::blockchain::chain::Chain;
use serde::Deserialize;

// Types of messages we send
#[derive(Debug, Deserialize)]
pub enum MessageReceived {
  NullMessage,
  ErrorMessageReceived {
    chain: Chain,
    error: Option<String>,
  },
  NewTransactionReceived {
    chain: Chain,
    result: Option<String>,
  },
}

#[derive(Debug)]
pub enum Command {}
