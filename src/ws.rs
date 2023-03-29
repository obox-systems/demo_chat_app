use std::sync::Arc;

use futures::{future::select, stream::SplitSink};
use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use tokio::{
  net::{TcpListener, TcpStream},
  sync::{
    broadcast::{Receiver, Sender},
    Mutex,
  },
};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

type Tx = SplitSink<WebSocketStream<TcpStream>, Message>;
type PeerList = Arc<Mutex<Vec<Tx>>>;

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum WsTypes {
  Create,
  Update,
  Delete,
}

#[derive(Serialize)]
pub struct WsMessage<T> {
  r#type: WsTypes,
  data: T,
}

#[derive(Serialize)]
pub struct MessageWithId {
  id: i64,
  #[serde(flatten)]
  message: crate::data::Message,
}

pub fn ws_new_message(id: i64, message: crate::data::Message) -> WsMessage<MessageWithId> {
  WsMessage {
    r#type: WsTypes::Create,
    data: MessageWithId { id, message },
  }
}

pub fn ws_update_message(id: i64, message: crate::data::Message) -> WsMessage<MessageWithId> {
  WsMessage {
    r#type: WsTypes::Update,
    data: MessageWithId { id, message },
  }
}

pub fn ws_delete_message(id: i64) -> WsMessage<i64> {
  WsMessage {
    r#type: WsTypes::Delete,
    data: id,
  }
}

pub struct Websocket {
  tx: Arc<Sender<String>>,
}

impl Websocket {
  pub fn send<T: Serialize>(&self, data: WsMessage<T>) -> anyhow::Result<usize> {
    if self.tx.receiver_count() == 0 {
      Ok(0)
    } else {
      Ok(self.tx.send(serde_json::to_string(&data)?)?)
    }
  }
}

pub async fn new_ws_server(addr: &str) -> anyhow::Result<Websocket> {
  let listener = TcpListener::bind(addr).await?;
  let (tx, _) = tokio::sync::broadcast::channel::<String>(1);
  let tx = Arc::new(tx);
  let tx_inner = tx.clone();
  tokio::spawn(async move {
    let peers: PeerList = Arc::new(Mutex::new(vec![]));
    while let Ok((stream, _)) = listener.accept().await {
      tokio::spawn(handle_connection(
        peers.clone(),
        stream,
        tx_inner.clone().subscribe(),
      ));
    }
  });
  Ok(Websocket { tx })
}

async fn handle_connection(peers: PeerList, raw_stream: TcpStream, mut rx_out: Receiver<String>) {
  if let Ok(ws) = accept_async(raw_stream).await {
    let (outgoing, incoming) = ws.split();

    let incoming = incoming.for_each_concurrent(None, |_| async {}); // ignore incomming

    peers.lock().await.push(outgoing);
    let peers_inner = peers.clone();
    let outgoing = tokio::spawn(async move {
      while let Ok(msg) = rx_out.recv().await {
        let mut peers = peers_inner.lock().await;

        let mut to_remove = vec![];
        for (index, client) in peers.iter_mut().enumerate() {
          if let Err(_) = client.send(Message::Text(msg.clone())).await {
            to_remove.push(index);
          }
        }
        to_remove.reverse();
        for index in to_remove.into_iter() {
          _ = peers.remove(index);
        }
      }
    });

    select(outgoing, incoming).await;
  }
}
