use std::collections::HashMap;
use std::sync::Arc;

use game::{Game, JoinRequest};
use protocol::{ClientMessage, ServerError};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::{io::BufStream, net::TcpListener};

mod ext;
mod game;
mod protocol;

struct Server {
    game: Game,
    /// Username -> Password map (Unencrypted lol)
    auth: HashMap<String, String>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (sender, receiver) = std::sync::mpsc::sync_channel::<JoinRequest>(64);
    let game = Game::new(32, receiver);
    let usermap = Arc::new(Mutex::new(HashMap::<String, String>::new()));
    let listener = TcpListener::bind("0.0.0.0:1312").await?;

    tokio::spawn(async move {
        let mut game = game;

        loop {
            game.tick().await;
            game.handle_connection_requests().await;
        }
    });

    loop {
        let connection = listener.accept().await;
        if let Err(err) = &connection {
            println!("ERROR: Could not accept connection {err}");
        }

        let send = sender.clone();
        let usermap = usermap.clone();

        tokio::spawn(async move {
            let (stream, addr) = connection.unwrap();
            let mut stream = BufStream::new(stream);

            println!("New connection req from {addr}");

            match connect_handler(&mut stream, &usermap).await {
                Ok(()) => {
                    if let Err(err) = send.try_send(JoinRequest { stream, addr }) {
                        println!("SEND ERROR: {err}")
                    };
                }
                Err(err) => {
                    println!("<- ERROR {err:?}");
                    let _ = stream.write(format!("ERROR {err:?}\n").as_bytes()).await;
                    return;
                }
            }
        });
    }
}

async fn connect_handler(
    stream: &mut BufStream<TcpStream>,
    user_map: &Arc<Mutex<HashMap<String, String>>>,
) -> Result<(), ServerError> {
    println!("<- MOTD hey :)");
    stream.write(b"MOTD hey :)\n").await?;
    stream.flush().await?;

    let mut buffer = String::new();
    stream.read_line(&mut buffer).await?;

    buffer.pop();
    println!("-> {buffer}");

    let client_req: ClientMessage = buffer.parse()?;

    let ClientMessage::Login { username, password } = client_req else {
        return Err(ServerError::UnexpectedCommand);
    };

    if username.is_empty() || password.is_empty() {
        return Err(ServerError::InvalidFormat);
    }

    {
        let mut user_map = user_map.lock().await;
        let entry = user_map.entry(username).or_insert(password.clone());

        if *entry != password {
            return Err(ServerError::InvalidCredentials);
        }
    }

    stream.write(b"HELLO 99\n").await?;
    stream.flush().await?;
    println!("<- HELLO");

    Ok(())
}
