use crate::protocol::types::enums::ClientState;
use crate::protocol::types::{MCString, VarInt};
use crate::protocol::{ProtocolReader, ProtocolWriter};
use anyhow::bail;
use tokio::net::TcpStream;

pub mod request;
pub mod response;

#[derive(Debug)]
pub struct Handshake {
    pub id: VarInt,
    pub protocol_ver: VarInt,
    pub host: MCString,
    pub port: u16,
    pub state: ClientState,
}

pub struct ClientConnection<'a> {
    pub state: ClientState,
    reader: ProtocolReader<'a>,
    writer: ProtocolWriter<'a>,
}

impl<'a> ClientConnection<'a> {
    pub fn new(stream: &'a mut TcpStream) -> anyhow::Result<Self> {
        let (reader, writer) = stream.split();

        Ok(Self {
            state: ClientState::Status,
            reader: ProtocolReader::from_stream(reader)?,
            writer: ProtocolWriter::from_stream(writer)?,
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
                state: self.reader.read_varint().await?.into(),
            };
            self.state = handshake.state;

            Ok(handshake)
        } else {
            bail!("Unexpected packet ID 0x{:02X}", packet_id);
        }
    }
}
