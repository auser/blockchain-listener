use crate::blockchain::blockchain::Blockchain;
use crate::blockchain::chain::Chain;
use crate::configuration::Settings;
use crate::errors::ChainError;
use crossbeam::channel;
use log::*;
use std::future::Future;
use std::sync::{Arc, Mutex};

// TODO: Rename and rework as blockchain manager
pub struct BlockchainManager {
  receiver: channel::Receiver<Arc<dyn Blockchain>>,
  sender: channel::Sender<Arc<dyn Blockchain>>,
}

impl BlockchainManager {
  pub fn new() -> Self {
    let (sender, receiver) = channel::unbounded();
    trace!("Created a new blockchain manager");
    Self { sender, receiver }
  }

  pub fn run(&self) {
    while let Ok(task) = self.receiver.recv() {
      trace!("Received a new task inside the blockchain manager");
      // task.poll();
    }
  }
}
