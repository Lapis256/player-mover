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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerData {
    pub pos: [f64; 3],
    pub dimension: String,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LevelDat {
    pub data: Data,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    spawn_x: i32,
    spawn_y: i32,
    spawn_z: i32,

    pub player: PlayerData,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

pub fn get_spawn_point(world: &Path) -> Result<[f64; 3], Box<dyn Error>> {
    let level_dat: LevelDat = fastnbt::from_bytes(&read_gzip(&world.join("level.dat"))?)?;

    Ok([
        level_dat.data.spawn_x as f64,
        level_dat.data.spawn_y as f64,
        level_dat.data.spawn_z as f64,
    ])
}

pub fn read_gzip(path: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut decoder = GzDecoder::new(file);
    let mut buf = vec![];
    decoder.read_to_end(&mut buf)?;

    Ok(buf)
}

pub fn write_gzip(path: &Path, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    let mut encoder = GzEncoder::new(file, Compression::best());
    encoder.write_all(&data)?;

    Ok(())
}
