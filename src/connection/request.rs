use super::{ClientConnection, ClientState};
use anyhow::bail;
use uuid::Uuid;

#[derive(Debug)]
pub enum Request {
    Status {
        packet_id: i32,
    },
    Ping {
        packet_id: i32,
        timestamp: i64,
    },
    LoginStart {
        packet_id: i32,
        username: String,
        uuid: Uuid,
    },
    LoginAcknowledged {
        packet_id: i32,
    },
    ClientConfiguration {
        packet_id: i32,
        locale: String,
        view_distance: i8,
        chat_mode: i32,
        enable_chat_colors: bool,
        displayed_skin_parts: u8,
        main_hand: i32,
        enable_text_filtering: bool,
        allow_server_listings: bool,
        particle_status: i32,
    },
    PluginMessage {
        packet_id: i32,
        id: String,
        data: String,
    },
    AcknowledgeFinishConfiguration {
        packet_id: i32,
    },
}

pub trait ReadRequest {
    #[allow(async_fn_in_trait)]
    async fn read_request(&self) -> anyhow::Result<Request>;
}

impl ReadRequest for ClientConnection {
    async fn read_request(&self) -> anyhow::Result<Request> {
        let packet_id = self.reader.packet_id().await?;

        match (&self.state, packet_id) {
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
