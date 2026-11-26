use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{collections::BTreeMap, io::Read};

use crate::{
    abilities::definition::AbilityId, characters::components::convictions::Convictions,
    items::definition::ItemId,
};

use godot::{classes::file_access::ModeFlags, tools::GFile};

pub const SAVE_FILE_PATH: &str = "res://save_game.json";

#[derive(Debug)]
pub enum SaveLoadError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl From<std::io::Error> for SaveLoadError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for SaveLoadError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveData {
    pub version: u32,
    pub player_state: PlayerState,
    pub player_convictions: PlayerConvictions,
    pub player_item: PlayerItems,
    pub player_ability: PlayerAbility,
    pub player_position: SavedPosition,
    pub open_chest: Vec<u32>,
    pub last_save_point: SavedPosition,
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

pub fn save_to_file(content: &str) -> std::io::Result<()> {
    let mut file = GFile::open(SAVE_FILE_PATH, ModeFlags::WRITE)?;
    file.write_all(content.as_bytes())
}

pub fn load_from_file() -> Result<SaveData, SaveLoadError> {
    let mut file = GFile::open(SAVE_FILE_PATH, ModeFlags::READ)?;
    let mut string_save = String::new();
    file.read_to_string(&mut string_save)?;
    Ok(serde_json::from_str(&string_save)?)
}
