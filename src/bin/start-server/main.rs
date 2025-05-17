use anyhow::{Result, bail};
use clap::Parser;
use minecraft_server::protocol::{ClientConnection, ClientState};
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
        let (stream, addr) = listener.accept().await?;
        println!("Connection with: {}:{}", addr.ip(), addr.port());
        let mut conn = ClientConnection::new(stream)?;

        handle_handshake_request(&mut conn).await?;

        loop {
            match &conn.state {
                ClientState::Status => {
                    handle_status_request(&mut conn).await?;
                    break;
                }
                ClientState::Login => handle_login_request(&mut conn).await?,
                ClientState::Configuration => handle_configuration_request(&mut conn).await?,
                ClientState::Play => bail!("Play is not implemented"),
            }
            println!(
                "###############################################################################"
            );
        }
        println!("Close connection with: {}:{}", addr.ip(), addr.port());
        println!("-------------------------------------------------------------------------------");
    }
}

async fn handle_handshake_request(conn: &mut ClientConnection) -> Result<()> {
    let packet_id = conn.reader.packet_id().await?;

    if packet_id == 0x00 {
        let protocol_ver = conn.reader.read_varint().await?;
        println!("Protocol version: {}", protocol_ver);

        let server_addr = conn.reader.read_string().await?;
        println!("Server address: {}", server_addr);

        let port = conn.reader.read_u16().await?;
        println!("Server port: {}", port);

        let state = ClientState::from(conn.reader.read_varint().await?)?;
        println!("Next state: {}", state);
        conn.state = state;
    } else {
        bail!("Unexpected packet ID 0x{:02X}", packet_id);
    }

    Ok(())
}

async fn handle_status_request(conn: &mut ClientConnection) -> Result<()> {
    loop {
        let packet_id = conn.reader.packet_id().await?;

        match packet_id {
            0x00 => {
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
                conn.writer.write_string(&response)?;
                conn.writer.send_packet(packet_id).await?;
            }
            0x01 => {
                let timestamp = conn.reader.read_i64().await?;
                conn.writer.write_i64(timestamp)?;
                conn.writer.send_packet(packet_id).await?;

                return Ok(());
            }
            _ => bail!("Unexpected packet ID 0x{:02X}", packet_id),
        }
    }
}

async fn handle_login_request(conn: &mut ClientConnection) -> Result<()> {
    let username = conn.reader.read_string().await?;
    println!("Username: {}", username);
    let uuid = conn.reader.read_uuid().await?;
    println!("UUID: {}", uuid);

    if username != "wristylotus" {
        conn.writer
            .write_string(r#"{text: "I don't know you, fuck off!", color: "red"}"#)?;
        conn.writer.send_packet(0x00).await?;
    }

    println!("Login Success");
    conn.writer.write_uuid(uuid)?;
    conn.writer.write_string(&username)?;
    conn.writer.write_varint(0)?;
    conn.writer.send_packet(0x02).await?;

    if conn.reader.packet_id().await? == 0x03 {
        println!("Login Acknowledged")
    }
    conn.state = ClientState::Configuration;

    Ok(())
}

async fn handle_configuration_request(conn: &mut ClientConnection) -> Result<()> {
    let packet_id = conn.reader.packet_id().await?;

    match packet_id {
        0x00 => {
            // Client Information (configuration)
            println!("Locale: {}", conn.reader.read_string().await?);
            println!("View Distance: {}", conn.reader.read_i8().await?);
            println!("Chat mode: {}", conn.reader.read_varint().await?);
            println!("Chat colors: {}", conn.reader.read_bool().await?);
            println!("Displayed Skin Parts: {:#b}", conn.reader.read_u8().await?);
            println!("Main Hand : {}", conn.reader.read_varint().await?);
            println!("Enable text filtering: {}", conn.reader.read_bool().await?);
            println!("Allow server listings: {}", conn.reader.read_bool().await?);
            println!("Particle Status: {}", conn.reader.read_varint().await?);

            // conn.writer.send_packet(0x03).await?;

            // TODO
            conn.writer
                .write_nbt_string("Disconnect: This section is not implemented")?;
            conn.writer.send_packet(0x02).await?;
            bail!("This section is not implemented");
        }
        0x02 => {
            // Server bound Plugin Message (configuration)
            println!("Channel: {}", conn.reader.read_string().await?);
            println!("Data: {}", conn.reader.read_string().await?);
        }
        _ => bail!("Unexpected packet ID 0x{:02X}", packet_id),
    }

    Ok(())
}
