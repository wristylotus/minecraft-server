use anyhow::{Result, bail};
use bytes::BytesMut;
use clap::Parser;
use minecraft_server::types::{HandshakeState, MCString, Packet, U16, VarInt};
use tokio::net::{TcpListener, TcpStream};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
    #[arg(long, default_value_t = 25565)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let (host, port) = (args.host, args.port);

    let listener = TcpListener::bind(format!("{host}:{port}")).await?;

    loop {
        let (mut stream, _) = listener.accept().await?;

        let packet = Packet::read(&mut stream).await?;
        println!("Packet length: {}", packet.length);
        println!("Packet ID: {}", packet.id);

        if packet.id == 0x00 {
            let (protocol_ver, data) = VarInt::read(packet.data)?;
            println!("Protocol version: {}", protocol_ver);

            let (server_addr, data) = MCString::read(data)?;
            println!("Server address: {}", server_addr);

            let (port, data) = U16::read(data)?;
            println!("Server port: {}", port);

            let (state, _data) = HandshakeState::read(data)?;
            println!("Next state: {}", state);
            println!("-----------------------------------");

            match state {
                HandshakeState::Status => handle_status_request(&mut stream).await?,
                HandshakeState::Login => handle_login(&mut stream).await?,
                _ => bail!("Invalid state"),
            }
        } else {
            bail!("Unexpected packet ID 0x{:02X}", packet.id);
        }
    }
}

async fn handle_status_request(stream: &mut TcpStream) -> Result<()> {
    let _packet_len = VarInt::read_stream(stream).await?;
    let packet_id = VarInt::read_stream(stream).await?;

    if packet_id == 0x00 {
        let response = r#"{
            "version": {
                "name": "1.21.5",
                "protocol": 770
            },
            "players": {
                "max": 2,
                "online": 0,
                "sample": []
            },
            "description": {
                "text": "Rust Minecraft Server"
            }
        }"#;

        let mut response_data = BytesMut::new();
        MCString::write(&response, &mut response_data)?;

        let packet = Packet::new(packet_id, response_data.into());

        packet.send(stream).await?;

        Ok(())
    } else {
        bail!("Unexpected packet ID 0x{:02X}", packet_id)
    }
}

async fn handle_login(stream: &mut TcpStream) -> Result<()> {
    let packet = Packet::read(stream).await?;

    Ok(())
}
