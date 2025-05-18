use super::ClientConnection;
use uuid::Uuid;

#[derive(Debug)]
pub enum Response {
    Status { cluster_info: String },
    LoginPong { timestamp: i64 },
    LoginSuccess { uuid: Uuid, username: String },
    LoginDisconnect { message: String },
    ConfigurationDisconnect { message: String },
    ConfigurationFinish
}

pub trait SendResponse {
    #[allow(async_fn_in_trait)]
    async fn send_response(&mut self, response: Response) -> anyhow::Result<()>;
}

impl SendResponse for ClientConnection {
    async fn send_response(&mut self, response: Response) -> anyhow::Result<()> {
        match response {
            Response::Status { cluster_info } => {
                self.writer.write_string(cluster_info.as_str())?;
                self.writer.send_packet(0x00).await
            }
            Response::LoginPong { timestamp } => {
                self.writer.write_i64(timestamp)?;
                self.writer.send_packet(0x01).await
            }
            Response::LoginSuccess { uuid, username } => {
                self.writer.write_uuid(uuid)?;
                self.writer.write_string(username.as_str())?;
                self.writer.write_varint(0)?;
                self.writer.send_packet(0x02).await
            }
            Response::LoginDisconnect { message } => {
                self.writer.write_string(message.as_str())?;
                self.writer.send_packet(0x00).await
            }
            Response::ConfigurationDisconnect { message } => {
                self.writer.write_nbt_string(message.as_str())?;
                self.writer.send_packet(0x02).await
            },
            Response::ConfigurationFinish => {
                self.writer.send_packet(0x03).await
            }
        }
    }
}
