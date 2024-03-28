use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub enum GameCommand {
    Foo,
    #[default]
    Nop,
}

#[derive(Debug, Clone)]
pub struct LoginCommand {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub enum ServerMessage {
    Motd { msg: String },
}

impl std::fmt::Display for ServerMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerMessage::Motd { msg } => write!(f, "MOTD {msg}"),
        }
    }
}

impl FromStr for LoginCommand {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.splitn(3, " ");

        match std::array::from_fn(|_| iter.next()) {
            [Some("LOGIN"), Some(username), Some(password)] => Ok(Self {
                username: username.to_owned(),
                password: password.to_owned(),
            }),
            _ => Err(ServerError::InvalidFormat),
        }
    }
}

impl FromStr for GameCommand {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prot, payload) = s.split_once(" ").ok_or(ServerError::InvalidFormat)?;

        match (prot.to_ascii_uppercase().as_str(), payload) {
            ("NOP", "") => Ok(GameCommand::Nop),
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
