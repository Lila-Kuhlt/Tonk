#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub enum ClientMessage {
    Login { username: String, password: String },
    Move { direction: Direction },
    Turn { deg: u32 },
}

#[derive(Debug, Clone)]
pub enum ServerMessage {
    Hello(String),
    GameStart { parameters: GameParameters },
}

impl std::fmt::Display for ServerMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerMessage::Hello(motd) => write!(f, "hello:{motd}"),
        }
    }
}

pub enum ServerError {
    UnknownCommand,
}

pub type ServerResponse = Result<ServerMessage, ServerError>;
