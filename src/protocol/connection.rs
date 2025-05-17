use crate::protocol::{ProtocolReader, ProtocolWriter};
use anyhow::bail;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use tokio::net::TcpStream;

#[derive(PartialEq, Debug)]
pub enum ClientState {
    Status,
    Login,
    Configuration,
    Play,
}

impl Display for ClientState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
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
}
