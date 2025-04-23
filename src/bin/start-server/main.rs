use clap::Parser;
use minecraft_server::types::VarInt;
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
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let (host, port) = (args.host, args.port);

    let listener = TcpListener::bind(format!("{host}:{port}")).await?;

    loop {
        let (mut stream, _) = listener.accept().await?;

        let package_len = VarInt::read(&mut stream).await?;
        println!("Package length: {}", package_len);

        let package_id = VarInt::read(&mut stream).await?;
        println!("Package ID: {}", package_id);

        let protocol_ver = VarInt::read(&mut stream).await?;
        println!("Protocol version: {}", protocol_ver);
    }
}
