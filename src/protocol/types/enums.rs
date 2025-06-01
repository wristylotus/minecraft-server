use crate::protocol::types::VarInt;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ClientState {
    Status,
    Login,
    Configuration,
    Play,
}

impl From<VarInt> for ClientState {
    fn from(value: VarInt) -> Self {
        match value.into() {
            1 => ClientState::Status,
            2 => ClientState::Login,
            3 => ClientState::Play,
            _ => panic!("Unknown handshake client state: {}", value),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ChatMode {
    Enabled = 0,
    CommandsOnly = 1,
    Hidden = 2,
}

impl From<VarInt> for ChatMode {
    fn from(value: VarInt) -> Self {
        match value.into() {
            0 => ChatMode::Enabled,
            1 => ChatMode::CommandsOnly,
            2 => ChatMode::Hidden,
            _ => panic!("Unknown chat mode: {}", value),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Hand {
    Left = 0,
    Right = 1,
}

impl From<VarInt> for Hand {
    fn from(value: VarInt) -> Self {
        match value.into() {
            0 => Hand::Left,
            1 => Hand::Right,
            _ => panic!("Unknown hand: {}", value),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ParticleStatus {
    All,
    Decreased,
    Minimal,
}

impl From<VarInt> for ParticleStatus {
    fn from(value: VarInt) -> Self {
        match value.into() {
            0 => ParticleStatus::All,
            1 => ParticleStatus::Decreased,
            2 => ParticleStatus::Minimal,
            _ => panic!("Unknown particle status: {}", value),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum GameMode {
    Undefined,
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl From<VarInt> for GameMode {
    fn from(value: VarInt) -> Self {
        match value.into() {
            -1 => GameMode::Undefined,
            0 => GameMode::Survival,
            1 => GameMode::Creative,
            2 => GameMode::Adventure,
            3 => GameMode::Spectator,
            _ => panic!("Unknown game mode: {}", value),
        }
    }
}

impl Into<VarInt> for GameMode {
    fn into(self) -> VarInt {
        VarInt::new(self as i32)
    }
}
