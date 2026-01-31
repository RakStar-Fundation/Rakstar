use crate::types::*;

pub type Checkpoint_Set_t = unsafe extern "C" fn(
    player: PlayerPtr,
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
) -> bool;

pub type Checkpoint_Disable_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;
pub type Checkpoint_IsPlayerIn_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;
pub type Checkpoint_IsActive_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;

pub type Checkpoint_Get_t = unsafe extern "C" fn(
    player: PlayerPtr,
    x: *mut f32,
    y: *mut f32,
    z: *mut f32,
    radius: *mut f32,
) -> bool;

pub type RaceCheckpoint_Set_t = unsafe extern "C" fn(
    player: PlayerPtr,
    type_: i32,
    x: f32,
    y: f32,
    z: f32,
    nextX: f32,
    nextY: f32,
    nextZ: f32,
    radius: f32,
) -> bool;

pub type RaceCheckpoint_Disable_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;
pub type RaceCheckpoint_IsPlayerIn_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;
pub type RaceCheckpoint_IsActive_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;

pub type RaceCheckpoint_Get_t = unsafe extern "C" fn(
    player: PlayerPtr,
    x: *mut f32,
    y: *mut f32,
    z: *mut f32,
    nextX: *mut f32,
    nextY: *mut f32,
    nextZ: *mut f32,
    radius: *mut f32,
) -> bool;

pub struct CheckpointAPI {
    pub set: Option<Checkpoint_Set_t>,
    pub disable: Option<Checkpoint_Disable_t>,
    pub is_player_in: Option<Checkpoint_IsPlayerIn_t>,
    pub is_active: Option<Checkpoint_IsActive_t>,
    pub get: Option<Checkpoint_Get_t>,
    pub race_set: Option<RaceCheckpoint_Set_t>,
    pub race_disable: Option<RaceCheckpoint_Disable_t>,
    pub race_is_player_in: Option<RaceCheckpoint_IsPlayerIn_t>,
    pub race_is_active: Option<RaceCheckpoint_IsActive_t>,
    pub race_get: Option<RaceCheckpoint_Get_t>,
}

impl Default for CheckpointAPI {
    fn default() -> Self {
        Self {
            set: None,
            disable: None,
            is_player_in: None,
            is_active: None,
            get: None,
            race_set: None,
            race_disable: None,
            race_is_player_in: None,
            race_is_active: None,
            race_get: None,
        }
    }
}
