use tokio::io::{AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;

use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::mpsc::{self, Receiver},
    time::Duration,
};

use tokio::io::BufStream;

use crate::protocol::{ServerMessage, ServerResponse};

#[derive(Copy, Clone)]
pub enum Position {
    Index(usize),
    XY(u32, u32),
}

#[derive(Copy, Clone)]
pub enum Tile {
    Air,
    Wall,
    Player(u32),
}

pub struct Player {
    health: u32,
    position: Position,
    addr: SocketAddr,
    stream: BufStream<TcpStream>,
}

pub struct JoinRequest {
    pub stream: BufStream<TcpStream>,
    pub addr: SocketAddr,
}

pub struct Game {
    fields: Vec<Tile>,
    next_client_id: u32,
    players: HashMap<u32, Player>,
    tick_speed: Duration,
    join_requests: mpsc::Receiver<JoinRequest>,
}

impl Game {
    pub fn new(size: usize, receiver: Receiver<JoinRequest>) -> Self {
        Self {
            fields: vec![Tile::Air; size * size],
            next_client_id: 0,
            players: HashMap::new(),
            tick_speed: Duration::ZERO,
            join_requests: receiver,
        }
    }

    pub async fn send_client(&mut self, target: u32, msg: ServerResponse) -> std::io::Result<()> {
        if let Some(client) = self.players.get_mut(&target) {
            let msg = match msg {
                Ok(msg) => format!("{msg}\n\0"),
                Err(msg) => format!("ERROR {msg:?}\n\0"),
            };

            client.stream.write(msg.as_bytes()).await?;
        }

        // TODO: don't return Ok, when client does not exist
        Ok(())
    }

    pub async fn handle_connection_requests(&mut self) {}

    pub async fn tick(&mut self) {}
}
