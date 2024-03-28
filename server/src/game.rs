use tokio::io::{AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;

use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::mpsc::{self, Receiver},
    time::Duration,
};

use tokio::io::BufStream;

use crate::protocol::ServerResponse;

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
    id: u32,
    health: u32,
    position: Position,
    addr: SocketAddr,
    stream: BufStream<TcpStream>,
}

pub struct JoinRequest {
    pub stream: BufStream<TcpStream>,
    pub addr: SocketAddr,
}

pub struct GameSettings {
    player_health: u32,
    tick_speed: Duration,
}

pub struct Game {
    fields: Vec<Tile>,
    next_client_id: u32,
    players: HashMap<u32, Player>,
    join_requests: mpsc::Receiver<JoinRequest>,
    settings: GameSettings,
}

impl Game {
    pub fn new(size: usize, receiver: Receiver<JoinRequest>) -> Self {
        Self {
            fields: vec![Tile::Air; size * size],
            next_client_id: 0,
            players: HashMap::new(),
            join_requests: receiver,
            settings: GameSettings {
                player_health: 100,
                tick_speed: Duration::ZERO,
            },
        }
    }

    pub async fn send_client(&mut self, target: u32, msg: ServerResponse) -> std::io::Result<()> {
        if let Some(client) = self.players.get_mut(&target) {
            let msg = match msg {
                Ok(msg) => format!("{msg}\n"),
                Err(msg) => format!("ERROR {msg:?}\n"),
            };

            client.stream.write(msg.as_bytes()).await?;
        }

        // TODO: don't return Ok, when client does not exist
        Ok(())
    }

    pub async fn handle_connection_requests(&mut self) {
        for JoinRequest { stream, addr } in self.join_requests.iter() {
            let player = Player {
                id: self.next_client_id,
                health: self.settings.player_health,
                position: Position::Index(0), // Fix this lol
                addr,
                stream,
            };

            self.next_client_id += 1;
            self.players.insert(player.id, player);
        }
    }

    pub async fn tick(&mut self) {

    }
}
