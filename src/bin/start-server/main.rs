use anyhow::{bail, Result};
use clap::Parser;
use minecraft_server::connection::request::{ReadRequest, Request};
use minecraft_server::connection::response::{Response, SendResponse};
use minecraft_server::connection::{ClientConnection, ClientState};
use std::net::SocketAddr;
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
        let (stream, addr) = listener.accept().await?;

        tokio::spawn(async move {
            process(stream, addr).await.expect("process failed");
        });
    }
}

async fn process(mut stream: TcpStream, addr: SocketAddr) -> Result<()> {
    println!("Connection with: {}", addr);
    let mut conn = ClientConnection::new(&mut stream)?;

    let handshake = conn.handshake().await?;
    println!("{:?}", handshake);

    loop {
        match &conn.state {
            ClientState::Status => {
                handle_status_request(&mut conn).await?;
                break;
            }
            ClientState::Login => handle_login_request(&mut conn).await?,
            ClientState::Configuration => handle_configuration_request(&mut conn).await?,
            ClientState::Play => handle_play_request(&mut conn).await?,
        }
        println!("###############################################################################");
    }

    println!("Close connection with: {}:{}", addr.ip(), addr.port());
    println!("-------------------------------------------------------------------------------");

    Ok(())
}

async fn handle_status_request(conn: &mut ClientConnection<'_>) -> Result<()> {
    'end_status: loop {
        match conn.read_request().await {
            Ok(Request::Status { .. }) => {
                let response = Response::Status {
                    cluster_info: String::from(
                        r#"{
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
                }"#,
                    ),
                };
                conn.send_response(response).await?;
            }
            Ok(Request::Ping { timestamp, .. }) => {
                conn.send_response(Response::LoginPong { timestamp }).await?;
                break 'end_status;
            }
            Ok(req) => bail!("Request '{:?}' not expected in Status state", req),
            Err(err) => bail!(err),
        }
    }

    Ok(())
}

async fn handle_login_request(conn: &mut ClientConnection<'_>) -> Result<()> {
    match conn.read_request().await {
        Ok(Request::LoginStart { username, uuid, .. }) => {
            println!("Username: {}", username);
            println!("UUID: {}", uuid);

            if username != "wristylotus" {
                conn.send_response(Response::LoginDisconnect {
                    message: String::from(r#"{text: "I don't know you, fuck off!", color: "red"}"#),
                })
                .await?;
            } else {
                conn.send_response(Response::LoginSuccess {
                    uuid,
                    username: String::from(username),
                })
                .await?;
            }
        }
        Ok(Request::LoginAcknowledged { .. }) => {
            println!("Login Acknowledged");
            conn.state = ClientState::Configuration;
        }
        Ok(req) => bail!("Request '{:?}' not expected in Login state", req),
        Err(err) => bail!(err),
    }

    Ok(())
}

async fn handle_configuration_request(conn: &mut ClientConnection<'_>) -> Result<()> {
    match conn.read_request().await {
        Ok(req @ Request::ClientConfiguration { .. }) => {
            println!("{:?}", req);
            conn.send_response(Response::ConfigurationFinish).await?;
        }
        Ok(req @ Request::PluginMessage { .. }) => {
            println!("{:?}", req);
        }
        Ok(Request::AcknowledgeFinishConfiguration { .. }) => {
            conn.state = ClientState::Play;
        }
        Ok(req) => bail!("Request '{:?}' not expected in Configuration state", req),
        Err(err) => bail!(err),
    }

    Ok(())
}

async fn handle_play_request(conn: &mut ClientConnection<'_>) -> Result<()> {
    //TODO Generate world
    match conn.read_request().await {
        Ok(req) => bail!("Request '{:?}' not expected in Configuration state", req),
        Err(err) => bail!(err),
    }
}
