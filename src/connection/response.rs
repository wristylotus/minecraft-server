use super::ClientConnection;

#[derive(Debug)]
pub enum Response {
    Status { id: i32, cluster_info: String },
    Pong { id: i32, timestamp: i64 },
}

pub trait SendResponse {
    #[allow(async_fn_in_trait)]
    async fn send_response(&mut self, response: Response) -> anyhow::Result<()>;
}

impl SendResponse for ClientConnection {
    async fn send_response(&mut self, response: Response) -> anyhow::Result<()> {
        match response {
            Response::Status { id, cluster_info } => {
                self.writer.write_string(cluster_info.as_str())?;
                self.writer.send_packet(id).await
            }
            Response::Pong { id, timestamp } => {
                self.writer.write_i64(timestamp)?;
                self.writer.send_packet(id).await
            }
        }
    }
}
