use std::io::prelude::*;

const USER: &str = "nixi";
const PASSWORD: &str = "test";

fn main() -> std::io::Result<()> {
    println!("Starting tonk client");

    let mut stream = std::net::TcpStream::connect("192.168.209.237:1312")?;
    let mut output = String::new();
    stream.read_to_string(&mut output)?;

    let mut server = Server {
        stream,
        game: GameState {
            map: Map::new(10, 10),
        },
    };
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
    Fire(u16, u32),
    Move(Direction),
    Login(String, String),
}

impl Command {
    fn to_string(&self) -> String {
        match self {
            Command::Fire(x, y) => format!("FIRE {} {}", x, y),
            Command::Move(dir) => format!("MOVE {}", dir.to_string()),
            Command::Login(user, pass) => format!("LOGIN {} {}", user, pass),
        }
    }
}

struct Server {
    stream: std::net::TcpStream,
    game: GameState,
}

impl Server {
    fn send_command(&mut self, command: Command) -> std::io::Result<()> {
        let command_str = command.to_string();
        self.stream.write(command_str.as_bytes())?;
        Ok(())
    }
}

struct GameState {
    map: Map,
}

struct Player {
    x: u32,
    y: u32,
    health: u32,
    id: u32,
}

impl TryFrom<&str> for Player {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let numbers: Result<Vec<_>, _> = value.split(',').map(|x| x.parse()).collect();
        let Ok(&[x, y, health, id]) = numbers.as_deref() else {
            return Err(());
        };
        Ok(Player { x, y, health, id })
    }
}

fn parse_players(command: &str) -> Result<Vec<Player>, ()> {
    let players = command.split_whitespace();
    players.into_iter().map(Player::try_from).collect()
}

struct Map {
    width: u16,
    height: u16,
    data: Vec<Tile>,
}

impl Map {
    fn new(width: u16, height: u16) -> Map {
        Map {
            width,
            height,
            data: vec![Tile::Empty; (width * height) as usize],
        }
    }

    fn get(&self, x: u16, y: u16) -> Tile {
        self.data[(y * self.width + x) as usize]
    }

    fn set(&mut self, x: u16, y: u16, value: Tile) {
        self.data[(y * self.width + x) as usize] = value;
    }

    fn parse(&mut self, data: String) {
        self.data = data.chars().map(Tile::from_char).collect();
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Player,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            ' ' => Tile::Empty,
            '#' => Tile::Wall,
            'P' => Tile::Player,
            _ => panic!("Unknown tile type: {}", c),
        }
    }
}
