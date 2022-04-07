// extern crate env_logger;
// use crossbeam::channel;
// use futures::task::Context;
// use futures::task::{self, ArcWake};
// use log::*;
// use std::future::Future;
// use std::pin::Pin;
// use std::sync::{Arc, Mutex};
use crate::configuration::ChainSetting;
use crate::errors::BlockchainError;
use crate::types::MessageReceived;
use async_trait::async_trait;
use tokio::sync::mpsc;

#[async_trait]
pub trait Blockchain {
  async fn spawn(
    sender: mpsc::Sender<MessageReceived>,
    settings: &ChainSetting,
  ) -> Result<Self, BlockchainError>
  where
    Self: std::marker::Sized;
  async fn watch(&mut self) -> Result<Option<MessageReceived>, BlockchainError>;
}
