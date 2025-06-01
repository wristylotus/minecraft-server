use super::ClientConnection;
use crate::protocol::types::{MCString, VarInt};
use uuid::Uuid;

#[derive(Debug)]
pub enum Response {
    Status { cluster_info: MCString },
    LoginPong { timestamp: i64 },
    LoginSuccess { uuid: Uuid, username: MCString },
    LoginDisconnect { message: MCString },
    // LoginPlay {
    //     
    // },
    ConfigurationDisconnect { message: MCString },
    ConfigurationFinish,
    
}

pub trait SendResponse {
    #[allow(async_fn_in_trait)]
    async fn send_response(&mut self, response: Response) -> anyhow::Result<()>;
}

impl SendResponse for ClientConnection<'_> {
    async fn send_response(&mut self, response: Response) -> anyhow::Result<()> {
        match response {
            Response::Status { cluster_info } => {
                self.writer.write(cluster_info)?;
                self.writer.send_packet(0x00.into()).await
            }
            Response::LoginPong { timestamp } => {
                self.writer.write(timestamp)?;
                self.writer.send_packet(0x01.into()).await
            }
            Response::LoginSuccess { uuid, username } => {
                self.writer.write(uuid)?;
                self.writer.write(username)?;
                self.writer.write(VarInt::new(0))?;
                self.writer.send_packet(0x02.into()).await
            }
            Response::LoginDisconnect { message } => {
                self.writer.write(message)?;
                self.writer.send_packet(0x00.into()).await
            }
            Response::ConfigurationDisconnect { message } => {
                self.writer.write(message)?;
                self.writer.send_packet(0x02.into()).await
            }
            Response::ConfigurationFinish => self.writer.send_packet(0x03.into()).await,
        }
    }
}
