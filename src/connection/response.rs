use super::ClientConnection;
use crate::protocol::types::enums::GameMode;
use crate::protocol::types::{Identifier, MCString, VarInt};
use uuid::Uuid;

#[derive(Debug)]
pub enum Response {
    Status {
        cluster_info: MCString,
    },
    LoginPong {
        timestamp: i64,
    },
    LoginSuccess {
        uuid: Uuid,
        username: MCString,
    },
    LoginDisconnect {
        message: MCString,
    },
    LoginPlay {
        entity_id: i32,
        is_hardcore: bool,
        dimension_names: Vec<Identifier>,
        max_players: VarInt,
        view_distance: VarInt,
        simulation_distance: VarInt,
        reduced_debug_info: bool,
        enable_respawn_screen: bool,
        do_limited_crafting: bool,
        dimension_type: VarInt,
        dimension_name: Identifier,
        hashed_seed: i64,
        game_mode: GameMode,
        previous_game_mode: GameMode,
        is_debug: bool,
        is_flat: bool,
        has_death_location: bool,
        death_dimension_name: Option<Identifier>,
        death_location: Option<(f64, f64, f64)>, //TODO Position type,
        portal_cooldown: VarInt,
        sea_level: VarInt,
        enforces_secure_chat: bool,
    },
    ConfigurationDisconnect {
        message: MCString,
    },
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
            Response::LoginPlay {
                entity_id,
                is_hardcore,
                dimension_names,
                max_players,
                view_distance,
                simulation_distance,
                reduced_debug_info,
                enable_respawn_screen,
                do_limited_crafting,
                dimension_type,
                dimension_name,
                hashed_seed,
                game_mode,
                previous_game_mode,
                is_debug,
                is_flat,
                has_death_location,
                death_dimension_name,
                death_location,
                portal_cooldown,
                sea_level,
                enforces_secure_chat,
            } => {
                self.writer.write(entity_id)?;
                self.writer.write(is_hardcore)?;
                //TODO self.writer.write(dimension_names)?;
                self.writer.write(max_players)?;
                self.writer.write(view_distance)?;
                self.writer.write(simulation_distance)?;
                self.writer.write(reduced_debug_info)?;
                self.writer.write(enable_respawn_screen)?;
                self.writer.write(do_limited_crafting)?;
                self.writer.write(dimension_type)?;
                self.writer.write(dimension_name)?;
                self.writer.write(hashed_seed)?;
                self.writer.write::<VarInt>(game_mode.into())?;
                self.writer.write::<VarInt>(previous_game_mode.into())?;
                self.writer.write(is_debug)?;
                self.writer.write(is_flat)?;
                self.writer.write(has_death_location)?;
                //TODO self.writer.write(death_dimension_name)?;
                //TODO self.writer.write(death_location)?;
                self.writer.write(portal_cooldown)?;
                self.writer.write(sea_level)?;
                self.writer.write(enforces_secure_chat)?;

                todo!();
                self.writer.send_packet(0x2B.into()).await
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
