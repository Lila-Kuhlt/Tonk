use std::str::FromStr;

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
    Foo,
}

#[derive(Debug, Clone)]
pub enum ServerMessage {
    Motd { msg: String },
    Hello { id: u32 },
}

impl std::fmt::Display for ServerMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerMessage::Motd { msg } => write!(f, "MOTD {msg}"),
            ServerMessage::Hello { id } => write!(f, "HELLO {id}"),
        }
    }
}

impl FromStr for ClientMessage {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prot, payload) = s.split_once(" ").ok_or(ServerError::InvalidFormat)?;

        match prot.to_ascii_uppercase().as_str() {
            "LOGIN" => {
                let (username, password) =
                    payload.split_once(" ").ok_or(ServerError::InvalidFormat)?;

                Ok(ClientMessage::Login {
                    username: username.into(),
                    password: password.into(),
                })
            }

            _ => Err(ServerError::UnknownCommand),
        }
    }
}

#[derive(Debug)]
pub enum ServerError {
    /// This command is not known
    UnknownCommand,
    /// This command is known, but is not expected (eg. MOVE before login)
    UnexpectedCommand,
    /// This command is in the wrong format (eg. LOGIN without password)
    InvalidFormat,
    InvalidCredentials,
    IOError(std::io::Error),
}

impl From<std::io::Error> for ServerError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

pub type ServerResponse = Result<ServerMessage, ServerError>;
