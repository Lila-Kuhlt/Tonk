mod ext;
mod game;
mod protocol;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    // connection thread
    tokio::spawn(async move {});

    Ok(())
}
