use std::{collections::HashMap, net::{SocketAddr, TcpStream}, time::Duration};

use tokio::io::BufStream;

#[derive(Copy, Clone)]
pub enum Position {
    Index(usize),
    XY(u32, u32),
}

#[derive(Copy, Clone)]
pub enum Field {
    Air,
    Obstacle,
    Player(u32),
}

pub struct Player {
    health: u32,
    position: Position,
    addr: SocketAddr,
    stream: BufStream<TcpStream>,
}

pub struct Game {
    fields: Vec<Field>,
    next_client_id: u32,
    map: HashMap<u32, Player>
    tick_speed: Duration,
}

impl Game {
    pub fn new(size: usize) -> Self {
        Self {
            fields: vec![Field::Air; size * size],
            next_client_id: 0,
            map: HashMap::new(),
            tick_speed: Duration::ZERO,
        }
    }
}
