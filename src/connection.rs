use crate::protocol::{ProtocolReader, ProtocolWriter};
use anyhow::bail;
use std::cell::RefCell;
use std::rc::Rc;
use tokio::net::TcpStream;

pub mod request;
pub mod response;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ClientState {
    Status,
    Login,
    Configuration,
    Play,
}

#[derive(Debug)]
pub struct Handshake {
    pub id: i32,
    pub protocol_ver: i32,
    pub host: String,
    pub port: u16,
    pub state: ClientState,
}

impl ClientState {
    pub fn from(num: i32) -> anyhow::Result<ClientState> {
        match num {
            1 => Ok(ClientState::Status),
            2 => Ok(ClientState::Login),
            3 => Ok(ClientState::Play),
            _ => bail!("Unknown handshake client state: {}", num),
        }
    }
}

pub struct ClientConnection {
    pub state: ClientState,
    pub reader: ProtocolReader,
    pub writer: ProtocolWriter,
}

impl ClientConnection {
    pub fn new(stream: TcpStream) -> anyhow::Result<Self> {
        let stream = Rc::new(RefCell::new(stream));

        Ok(Self {
            state: ClientState::Status,
            reader: ProtocolReader::from_stream(stream.clone())?,
            writer: ProtocolWriter::from_stream(stream.clone())?,
        })
    }

    pub async fn handshake(&mut self) -> anyhow::Result<Handshake> {
        let packet_id = self.reader.packet_id().await?;

        if packet_id == 0x00 {
            let handshake = Handshake {
                id: packet_id,
                protocol_ver: self.reader.read_varint().await?,
                host: self.reader.read_string().await?,
                port: self.reader.read_u16().await?,
                state: ClientState::from(self.reader.read_varint().await?)?,
            };
            self.state = handshake.state;

            Ok(handshake)
        } else {
            bail!("Unexpected packet ID 0x{:02X}", packet_id);
        }
    }
}
