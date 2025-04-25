use anyhow::{Result, bail};
use clap::Parser;
use minecraft_server::protocol::{ProtocolReader, ProtocolWriter};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use tokio::net::TcpListener;

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
        let (stream, _) = listener.accept().await?;
        let stream = Rc::new(RefCell::new(stream));

        let reader = ProtocolReader::from_stream(stream.clone()).await?;
        let mut writer = ProtocolWriter::from_stream(stream.clone())?;

        if reader.packet_id()? == 0x00 {
            let protocol_ver = reader.read_varint().await?;
            println!("Protocol version: {}", protocol_ver);

            let server_addr = reader.read_string().await?;
            println!("Server address: {}", server_addr);

            let port = reader.read_u16().await?;
            println!("Server port: {}", port);

            let state = HandshakeState::from(reader.read_varint().await?)?;
            println!("Next state: {}", state);
            println!("-----------------------------------");

            match state {
                HandshakeState::Status => handle_status_request(&reader, &mut writer).await?,
                HandshakeState::Login => handle_login(&reader).await?,
                _ => bail!("Invalid state"),
            }
        } else {
            bail!("Unexpected packet ID 0x{:02X}", reader.packet_id()?);
        }
    }
}

async fn handle_status_request(reader: &ProtocolReader, writer: &mut ProtocolWriter) -> Result<()> {
    // 0x01 for PING
    if reader.packet_id()? == 0x00 {
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

        writer.write_string(&response)?;

        writer.send_packet(reader.packet_id()?).await?;

        Ok(())
    } else {
        bail!("Unexpected packet ID 0x{:02X}", reader.packet_id()?);
    }
}

async fn handle_login(reader: &ProtocolReader) -> Result<()> {
    let username = reader.read_string().await?;
    println!("Username: {}", username);
    let uuid = reader.read_uuid().await?;
    println!("UUID: {}", uuid);

    Ok(())
}

#[derive(Debug)]
pub enum HandshakeState {
    Status = 1,
    Login = 2,
    Transfer = 3,
}

impl Display for HandshakeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl HandshakeState {
    pub fn from(num: i32) -> anyhow::Result<HandshakeState> {
        match num {
            1 => Ok(HandshakeState::Status),
            2 => Ok(HandshakeState::Login),
            3 => Ok(HandshakeState::Transfer),
            _ => bail!("Unknown handshake state"),
        }
    }
}
