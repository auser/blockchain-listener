use anyhow::{Context, Result};
use async_trait::async_trait;
use crossbeam::channel;
use futures::Future;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use listener_core::blockchain::blockchain::Blockchain;
use listener_core::blockchain::chain::Chain;
use listener_core::configuration::ChainSetting;
use listener_core::errors::BlockchainError;
use listener_core::types::Command;
use listener_core::types::MessageReceived;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

use tokio_tungstenite::{
    tungstenite::protocol::WebSocketConfig, tungstenite::Message as WebsocketMessage,
    tungstenite::Message, MaybeTlsStream, WebSocketStream,
};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EthSubscriptionMessageParams {
    pub subscription: String,
    pub result: Option<String>,
}

// Types of messages we can get back
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum EthControlMessage {
    Ping,
    NullMessage {
        jsonrpc: String,
        id: u64,
        result: Option<String>,
    },
    SubscriptionMessage {
        jsonrpc: String,
        id: Option<String>,
        // method: Option<String>,
        params: EthSubscriptionMessageParams,
    },
    NewPendingTransaction {
        jsonrpc: String,
        id: Option<u64>,
        // method: Option<String>,
        result: serde_json::Value,
    },
}

// type WebSocketStream = tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct Avalanche {
    pub write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    pub read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,

    message_sender: mpsc::Sender<MessageReceived>, // executor: channel::Sender<Arc<dyn Blockchain>>
}

#[async_trait]
impl Blockchain for Avalanche {
    async fn spawn(
        sender: mpsc::Sender<MessageReceived>,
        settings: &ChainSetting,
    ) -> Result<Self, BlockchainError>
    where
        Self: std::marker::Sized,
    {
        let ws_config: WebSocketConfig = WebSocketConfig::default();
        let url = format!(
            "{}{}:{}{}",
            settings.protocol, settings.host, settings.port, settings.endpoint
        );

        let ws = match tokio_tungstenite::connect_async_with_config(url, Some(ws_config)).await {
            Ok((ws, _)) => ws,
            Err(e) => return Err(BlockchainError::Other(Box::new(e))),
        };

        let (write, read) = ws.split();

        let inst = Self {
            read,
            write,
            message_sender: sender,
        };

        // let chain = Arc::new(Mutex::from(inst));

        Ok(inst)
    }

    async fn watch(&mut self) -> Result<Option<MessageReceived>, BlockchainError> {
        let _ = self.subscribe_to_changes().await.map_err(|_err| {
            println!("Error subscribing to changes");
            return BlockchainError::ConnectionError("Error subscribing to changes".into());
        });
        loop {
            while let Some(msg) = self.read.next().await {
                match msg {
                    Ok(tokio_tungstenite::tungstenite::Message::Text(s)) => {
                        match serde_json::from_str(&s) {
                            Ok::<EthControlMessage, _>(mr) => {
                                // Convert intermediate message into listener message
                                match mr {
                                    EthControlMessage::SubscriptionMessage { params, .. } => {
                                        // We have a new transaction hash
                                        match params.result {
                                            Some(hsh) => {
                                                let msg = MessageReceived::NewTransactionReceived {
                                                    chain: Chain::Avalanche,
                                                    result: Some(hsh),
                                                };
                                                let _ = self.message_sender.send(msg).await;
                                            }
                                            None => {}
                                        };
                                    }
                                    _other => {}
                                }
                            }
                            Err(e) => {
                                println!("Error converting into message received: {:?}", e);
                            }
                        }
                    }
                    Ok(Message::Binary(b)) => {
                        println!("Got binary back: {:?}", b)
                    }
                    Ok(_) => {
                        println!("Got something else back");
                    }
                    Err(_) => {
                        println!("Got some error back");
                    }
                }
            }
        }
    }
}

impl Avalanche {
    async fn subscribe_to_changes(&mut self) -> Result<()> {
        self.send_message(
            "eth_subscribe",
            serde_json::json!(["newPendingTransactions"]),
        )
        .await
        .with_context(|| "Unable to send subscribe")?;
        Ok(())
    }

    async fn send_message(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<(), BlockchainError> {
        let json: &serde_json::Value = &serde_json::json!({
          "jsonrpc":"2.0",
          "method": method,
          "params": params,
          "id": 1
        });
        log::debug!("Sending message: {:?}", json);
        // let mut ws = self.ws;
        self.write
            .send(WebsocketMessage::Text(json.to_string()))
            .await
            .map_err(|err| BlockchainError::SendMessageError(err.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
