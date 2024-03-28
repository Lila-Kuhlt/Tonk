use std::{collections::HashMap, sync::Arc};
use tokio::io::AsyncWriteExt;

use protocol::ServerMessage;
use tokio::{
    io::BufStream,
    net::{TcpListener, TcpStream},
    sync::RwLock,
};

use ext::AsyncMap;

mod ext;
mod game;
mod protocol;

type ArcState = Arc<RwLock<Controler>>;

#[derive(Default)]
pub struct PeerMap {
    pub peers: HashMap<u32, BufStream<TcpStream>>,
}

impl PeerMap {
    async fn send(&mut self, to: u32, message: ServerMessage) {
        if let Some(item) = self.peers.get_mut(&to) {
            item.write_all(&format!("{message}").as_bytes());
        }
    }

    async fn broadcast(&mut self, message: ServerMessage) {
        self.peers.keys().map(|key| self.send(*key, message));
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let socket_addr = "[::1]:1312";
    let tcp_listener = TcpListener::bind(socket_addr).await?;
    let peer_list = ArcState::default();

    // connection thread
    tokio::spawn(async move {});

    Ok(())
}
