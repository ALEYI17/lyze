use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    abilities::definition::AbilityId, characters::components::convictions::Convictions,
    items::definition::ItemId,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveData {
    pub version: u32,
    pub player_state: PlayerState,
    pub player_convictions: PlayerConvictions,
    pub player_item: PlayerItems,
    pub player_ability: PlayerAbility,
    pub player_position: SavedPosition,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerState {
    pub health: f32,
    pub max_health: f32,
    pub level: u32,
    pub current_experience: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerConvictions {
    pub convictions: Convictions,
    pub avaible_points: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerItems {
    pub items: BTreeMap<ItemId, u32>,
    pub prosthetics: BTreeMap<ItemId, u32>,
    pub left_arm: Option<ItemId>,
    pub right_leg: Option<ItemId>,
    pub granted_prosthetics: Vec<ItemId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerAbility {
    pub learned_abilities: Vec<AbilityId>,
    pub equiped_abilities: Vec<AbilityId>,
    pub avaible_points: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SavedPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub scene_path: Option<String>,
}

pub fn write_save_data(data: &SaveData) -> Result<String, serde_json::Error> {
    let json = serde_json::to_string_pretty(data)?;

    Ok(json)
}
