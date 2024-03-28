use std::{fmt::Write, io::prelude::*, io::Write as IOWrite, io::BufWriter, io::BufRead, io::BufReader};

const USER: &str = "nixi";
const PASSWORD: &str = "test";

fn main() -> std::io::Result<()> {
    println!("Starting tonk client");

    let stream = std::net::TcpStream::connect("192.168.209.237:1312")?;
    let mut reader = std::io::BufReader::new(stream.try_clone()?);

    /*let mut server = Server {
        stream,
    };*/
    let mut gamestate = GameState::new();

    loop {
        let mut response = String::new();
        reader.read_line(&mut response)?;
        if response.contains('\n') {
            let response = response.trim();
            for line in response.lines() {
                gamestate.process_response(line, &stream);
            }
        }
    }
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

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Fire(x, y) => write!(f, "FIRE {} {}", x, y),
            Command::Move(dir) => write!(f, "MOVE {}", dir.to_string()),
            Command::Login(user, pass) => write!(f, "LOGIN {} {}", user, pass),
        }
    }
}

trait CommandSink {
    fn send_command(&mut self, command: Command) -> std::io::Result<()>;
}

impl<T:IOWrite> CommandSink for T {
    fn send_command(&mut self, command: Command) -> std::io::Result<()> {
        println!("<- {}", command.to_string());
        writeln!(self, "{command}");
        self.flush()
    }
}

struct GameState {
    id: u32,
    map: Map,
    players: Vec<Player>,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            id: 0,
            map: Map::new(10, 10),
            players: vec![],
        }
    }
    fn process_response(&mut self, response: &str, mut server:  impl IOWrite) {
        println!("-> {}", response);
        match response.split_once(' ').unwrap_or((response, "")) {
            ("MOTD", msg) => {
                println!("Message of the day: {}", msg);
                server.send_command(Command::Login(USER.to_string(), PASSWORD.to_string())).expect("Failed to send login command");
            },
            ("HELLO", "") => (),
            ("HELLO", id) => {
                    println!("Aaron ist Böse :(");
            },
            ("START", args) => {
                let dimensions: Vec<_> = args
                    .split(' ')
                    .collect();
                self.map = Map::new(dimensions[0].parse().expect("Could not parse height"), dimensions[1].parse().expect("Could not parse height"));
                self.id = dimensions[2].parse().expect("Could not parse ID");
            }
            ("DIE", "") => (),
            ("WIN", "") => (),
            ("PLAYER", players) => {
                self.players = parse_players(players).expect("Failed to parse players")
            }
            ("MAP", data) => {
                self.map = Map::parse(self.map.width, self.map.height, data.to_string())
                    .expect("Failed to parse map")
            }
            ("END", "") => (),
            ("TICK", "") => {
                let command = self.generate_response();
                server.send_command(command).expect("Failed to send command");
            },
            _ => panic!("Unknown response: {}", response),
        }
    }

    fn generate_response(&self) -> Command {
        Command::Move(Direction::Up)
    }
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

    fn parse(width: u16, height: u16, data: String) -> Result<Map, ()> {
        let data: Vec<_> = data.chars().map(Tile::from_char).collect();
        if data.len() != (width * height) as usize {
            return Err(());
        }
        Ok(Map {
            width,
            height,
            data,
        })
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
