use anyhow::{bail, Result};
use clap::Parser;
use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::Runtime;
use deadpool_redis::{Config as RedisConfig, Pool};
use log::{error, info};
use minecraft_server::connection::request::{ReadRequest, Request};
use minecraft_server::connection::response::{Response, SendResponse};
use minecraft_server::connection::{ClientConnection, ClientState};
use tokio::net::{TcpListener, TcpStream};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
    #[arg(long, default_value_t = 25565)]
    port: u16,
    #[arg(long, default_value = "redis://localhost:6379")]
    redis_url: String,
    #[arg(long, default_value_t = 10)]
    redis_pool_size: usize,
}

const SERVER_INFO: &str = r#"{
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

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();
    let (host, port) = (args.host, args.port);

    let redis_pool = RedisConfig::from_url(args.redis_url)
        .builder()?
        .max_size(args.redis_pool_size)
        .runtime(Runtime::Tokio1)
        .build()?;

    let listener = TcpListener::bind(format!("{host}:{port}")).await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        let redis_pool = redis_pool.clone();

        tokio::spawn(async move {
            info!("Connection with: {}", addr);

            let _: () = redis_pool.get().await.unwrap().set("test", "test").await.unwrap();

            if let Err(e) = handle_connection(stream, redis_pool).await {
                error!("Client: {}. Connection error: {}", addr, e);
            } else {
                info!("Close connection with client: {}", addr);
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream, redis_pool: Pool) -> Result<()> {
    let mut conn = ClientConnection::new(&mut stream)?;

    let handshake = conn.handshake().await?;
    info!("{:?}", handshake);

    {
        let mut conn = redis_pool.get().await?;
        let _: () = conn.set("test", "test").await?;
    }

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
    }

    Ok(())
}

async fn handle_status_request(conn: &mut ClientConnection<'_>) -> Result<()> {
    'end_status: loop {
        match conn.read_request().await {
            Ok(Request::Status { .. }) => {
                let response = Response::Status {
                    cluster_info: SERVER_INFO.into(),
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
            info!("Username: {}, UUID: {}", username, uuid);

            if username != "wristylotus" {
                conn.send_response(Response::LoginDisconnect {
                    message: r#"{text: "I don't know you, fuck off!", color: "red"}"#.into(),
                })
                .await?;
            } else {
                conn.send_response(Response::LoginSuccess { uuid, username }).await?;
            }
        }
        Ok(Request::LoginAcknowledged { .. }) => {
            info!("Login Acknowledged");
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
            info!("{:?}", req);
            conn.send_response(Response::ConfigurationFinish).await?;
        }
        Ok(req @ Request::PluginMessage { .. }) => {
            info!("{:?}", req);
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
