use std::io::prelude::*;

const USER: &str = "nixi";
const PASSWORD: &str = "test";

fn main() -> std::io::Result<()> {
    println!("Starting tonk client");

    let mut stream = std::net::TcpStream::connect("192.168.209.237:1312")?;
    let mut output = String::new();
    stream.read_to_string(&mut output)?;

    let mut server = Server { stream, game: GameState {} };
    server.send_command(Command::Login(USER.to_string(), PASSWORD.to_string()))?;

    Ok(())
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Up => "UP".to_string(),
            Direction::Down => "DOWN".to_string(),
            Direction::Left => "LEFT".to_string(),
            Direction::Right => "RIGHT".to_string(),
        }
    }
}

enum Command {
    Fire (u16, u32),
    Move (Direction),
    Login (String, String),
}

impl Command {
    fn to_string(&self) -> String {
        match self {
            Command::Fire (x, y) => format!("FIRE {} {}", x, y),
            Command::Move (dir) => format!("MOVE {}", dir.to_string()),
            Command::Login (user, pass) => format!("LOGIN {} {}", user, pass),
        }
    }
}

struct Server {
    stream: std::net::TcpStream,
    game: GameState
}

impl Server {
    fn send_command(&mut self, command: Command) -> std::io::Result<()> {
        let command_str = command.to_string();
        self.stream.write(command_str.as_bytes())?;
        Ok(())
    }
}

struct GameState {
}
