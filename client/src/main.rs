use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Starting tonk client");

    let mut stream = std::net::TcpStream::connect("192.168.209.237:1312")?;

    let mut output = String::new();
    stream.read_to_string(&mut output)?;

    stream.write(b"LOGIN nixi test")?;
    Ok(())
}
