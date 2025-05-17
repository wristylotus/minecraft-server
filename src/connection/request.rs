use super::{ClientConnection, ClientState};
use anyhow::bail;

#[derive(Debug)]
pub enum Request {
    Status { id: i32 },
    Ping { id: i32, timestamp: i64 },
}

pub trait ReadRequest {
    #[allow(async_fn_in_trait)]
    async fn read_request(&self) -> anyhow::Result<Request>;
}

impl ReadRequest for ClientConnection {
    async fn read_request(&self) -> anyhow::Result<Request> {
        let packet_id = self.reader.packet_id().await?;

        match (&self.state, packet_id) {
            (ClientState::Status, 0x00) => Ok(Request::Status { id: packet_id }),
            (ClientState::Status, 0x01) => Ok(Request::Ping {
                id: packet_id,
                timestamp: self.reader.read_i64().await?,
            }),
            (ClientState::Status, _) => {
                bail!("Unknown packet ID: '0x{:02X}' for state: 'Status'", packet_id)
            }
            (ClientState::Login, _) => {
                bail!("Unknown packet ID: '0x{:02X}' for state: 'Login'", packet_id)
            }
            (ClientState::Configuration, _) => {
                bail!("Unknown packet ID: '0x{:02X}' for state: 'Configuration'", packet_id)
            }
            (ClientState::Play, _) => {
                bail!("Unknown packet ID: '0x{:02X}' for state: 'Play'", packet_id)
            }
        }
    }
}
