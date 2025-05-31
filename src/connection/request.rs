use super::{ClientConnection, ClientState};
use crate::protocol::types::{MCString, VarInt};
use anyhow::bail;
use uuid::Uuid;

#[derive(Debug)]
pub enum Request {
    Status {
        packet_id: VarInt,
    },
    Ping {
        packet_id: VarInt,
        timestamp: i64,
    },
    LoginStart {
        packet_id: VarInt,
        username: MCString,
        uuid: Uuid,
    },
    LoginAcknowledged {
        packet_id: VarInt,
    },
    ClientConfiguration {
        packet_id: VarInt,
        locale: MCString,
        view_distance: i8,
        chat_mode: VarInt,
        enable_chat_colors: bool,
        displayed_skin_parts: u8,
        main_hand: VarInt,
        enable_text_filtering: bool,
        allow_server_listings: bool,
        particle_status: VarInt,
    },
    PluginMessage {
        packet_id: VarInt,
        id: MCString,
        data: MCString,
    },
    AcknowledgeFinishConfiguration {
        packet_id: VarInt,
    },
}

pub trait ReadRequest {
    #[allow(async_fn_in_trait)]
    async fn read_request(&mut self) -> anyhow::Result<Request>;
}

impl ReadRequest for ClientConnection<'_> {
    async fn read_request(&mut self) -> anyhow::Result<Request> {
        let packet_id = self.reader.packet_id().await?;

        match (&self.state, packet_id.into()) {
            // Status
            (ClientState::Status, 0x00) => Ok(Request::Status { packet_id }),
            (ClientState::Status, 0x01) => Ok(Request::Ping {
                packet_id,
                timestamp: self.reader.read_i64().await?,
            }),
            (ClientState::Status, _) => {
                bail!("Unknown packet ID: '0x{:02X}' for state: 'Status'", packet_id)
            }
            // Login
            (ClientState::Login, 0x00) => Ok(Request::LoginStart {
                packet_id,
                username: self.reader.read_string().await?,
                uuid: self.reader.read_uuid().await?,
            }),
            (ClientState::Login, 0x03) => Ok(Request::LoginAcknowledged { packet_id }),
            (ClientState::Login, _) => {
                bail!("Unknown packet ID: '0x{:02X}' for state: 'Login'", packet_id)
            }
            // Configuration
            (ClientState::Configuration, 0x00) => Ok(Request::ClientConfiguration {
                packet_id,
                locale: self.reader.read_string().await?,
                view_distance: self.reader.read_i8().await?,
                chat_mode: self.reader.read_varint().await?,
                enable_chat_colors: self.reader.read_bool().await?,
                displayed_skin_parts: self.reader.read_u8().await?,
                main_hand: self.reader.read_varint().await?,
                enable_text_filtering: self.reader.read_bool().await?,
                allow_server_listings: self.reader.read_bool().await?,
                particle_status: self.reader.read_varint().await?,
            }),
            (ClientState::Configuration, 0x02) => Ok(Request::PluginMessage {
                packet_id,
                id: self.reader.read_string().await?,
                data: self.reader.read_string().await?,
            }),
            (ClientState::Configuration, 0x03) => Ok(Request::AcknowledgeFinishConfiguration { packet_id }),
            (ClientState::Configuration, _) => {
                bail!("Unknown packet ID: '0x{:02X}' for state: 'Configuration'", packet_id)
            }
            // Play
            (ClientState::Play, _) => {
                bail!("Unknown packet ID: '0x{:02X}' for state: 'Play'", packet_id)
            }
        }
    }
}
