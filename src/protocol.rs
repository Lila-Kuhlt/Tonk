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
}

#[derive(Debug, Clone)]
pub enum ServerMessage {
    Hello(String),
}

pub enum ServerError {
    UnknownCommand,
}

pub type ServerResponse = Result<ServerMessage, ServerError>;
