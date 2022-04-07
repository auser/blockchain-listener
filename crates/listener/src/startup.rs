use crossbeam::channel;
use listener_avalanche::Avalanche;
use listener_core::blockchain::blockchain::Blockchain;
use listener_core::blockchain::chain::Chain;
use listener_core::configuration::Settings;
use listener_core::errors::ChainError;
use listener_core::types::Command;
use listener_core::types::MessageReceived;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

pub struct Listener {
  // TODO: change to blockchain
  pub chains: Vec<Avalanche>,
  receiver: mpsc::Receiver<MessageReceived>,
  sender: mpsc::Sender<MessageReceived>,
}

impl Listener {
  pub fn spawn<F>(&self) {
    println!("YAY");
    // for chain in self.chains {
    // chain.spawn(async {
    //   println!("Spawn in listener");
    // })
    // }
  }

  pub async fn build(settings: &Settings) -> Self {
    let (sender, receiver) = mpsc::channel::<MessageReceived>(32);

    let mut chain_list = vec![];
    for key in settings.blockchains.keys() {
      println!("Trying to do key thing: {}", key);
      match Chain::try_from(String::from(key)) {
        Ok(t) => match t {
          Chain::Avalanche => {
            let config = settings.blockchains[key.into()].clone();
            println!("Found avalanche chain: {:?}", config);
            let sender = sender.clone();
            match Avalanche::spawn(sender, &config).await {
              Ok(chain) => {
                chain_list.push(chain);
              }
              Err(_) => {}
            }
          }
          _other => {} //return Err(ChainError::UnsupportedChain(String::from(key))),
        },
        _ => {} //return Err(ChainError::UnableToInitialize(String::from(key))),
      };
    }
    // self.chains = chain_list;
    Self {
      chains: chain_list,
      sender,
      receiver,
    }
  }

  pub async fn run_until_stopped(mut self) -> Result<(), std::io::Error> {
    for chain in self.chains {
      // let blk = chain as Box<dyn Blockchain>;
      tokio::spawn(async {
        let mut chain = chain;
        let _ = chain.watch().await;
      });
    }

    loop {
      tokio::select! {
        msg = self.receiver.recv() => {
          println!("Got a message on receiver: {:?}", msg);
        }
      }
    }

    // self.manager.run();
    // Ok(())
  }
}
