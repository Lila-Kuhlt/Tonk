use tokio::io::{AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;

mod map;
pub use map::*;

use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::mpsc::{self, Receiver},
    time::Duration,
};

use tokio::io::BufStream;

use crate::protocol::{GameCommand, ServerResponse};

pub struct Player {
    id: u32,
    health: u32,
    position: Position,
    addr: SocketAddr,
    stream: BufStream<TcpStream>,
    command: GameCommand,
}

pub struct JoinRequest {
    pub stream: BufStream<TcpStream>,
    pub addr: SocketAddr,
}

pub struct GameSettings {
    player_health: u32,
    tick_speed: Duration,
    connection_limit: u32,
}

pub struct Game {
    next_client_id: u32,
    players: HashMap<u32, Player>,
    join_requests: mpsc::Receiver<JoinRequest>,
    settings: GameSettings,
    map: Map,
}

impl Game {
    pub fn new(width: usize, height: usize, receiver: Receiver<JoinRequest>) -> Self {
        Self {
            map: Map::new(width, height),
            next_client_id: 0,
            players: HashMap::new(),
            join_requests: receiver,
            settings: GameSettings {
                connection_limit: 1000,
                player_health: 100,
                tick_speed: Duration::from_secs(1),
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
            if self.players.len() > self.settings.connection_limit as usize {
                continue;
            }

            let player = Player {
                id: self.next_client_id,
                health: self.settings.player_health,
                position: Position::Index(0), // Fix this lol
                command: GameCommand::Nop,
                addr,
                stream,
            };

            self.next_client_id += 1;
            self.players.insert(player.id, player);
        }
    }

    pub async fn tick(&mut self) {
        for player in self.players.values_mut() {
            let command = std::mem::take(&mut player.command);
            if command == GameCommand::Nop {
                continue;
            }
        }
    }
}
