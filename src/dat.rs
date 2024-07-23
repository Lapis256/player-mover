use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use fastnbt::Value;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerData {
    pub pos: [f64; 3],
    pub dimension: String,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct LevelDat {
    #[serde(rename = "Data")]
    data: Data,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Data {
    spawn_x: i32,
    spawn_y: i32,
    spawn_z: i32,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

pub fn get_spawn_point(world: &Path) -> Result<[f64; 3], Box<dyn Error>> {
    let level_dat_path = world.join("level.dat");
    let file = File::open(level_dat_path)?;
    let mut decoder = GzDecoder::new(file);
    let mut bytes = vec![];
    decoder.read_to_end(&mut bytes)?;

    let level_dat: LevelDat = fastnbt::from_bytes(&bytes)?;

    Ok([
        level_dat.data.spawn_x as f64,
        level_dat.data.spawn_y as f64,
        level_dat.data.spawn_z as f64,
    ])
}

pub fn read_player_data(path: &Path) -> Result<PlayerData, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut decoder = GzDecoder::new(file);
    let mut bytes = vec![];
    decoder.read_to_end(&mut bytes)?;

    Ok(fastnbt::from_bytes(&bytes)?)
}

pub fn write_player_data(path: &Path, player_data: &PlayerData) -> Result<(), Box<dyn Error>> {
    let bytes = fastnbt::to_bytes(player_data)?;
    let file = File::create(path)?;
    let mut encoder = GzEncoder::new(file, Compression::best());
    encoder.write_all(&bytes)?;

    Ok(())
}
