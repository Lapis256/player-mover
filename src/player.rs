use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use crate::{
    api,
    dat::{read_gzip, write_gzip, LevelDat, PlayerData},
};

pub trait Player {
    fn decode(&mut self, nbt: Vec<u8>) -> Result<PlayerData, Box<dyn Error>>;
    fn encode(&self, data: PlayerData) -> Result<Vec<u8>, Box<dyn Error>>;

    fn get_path(&self) -> &Path;
    fn get_name(&self) -> &str;

    fn load_data(&mut self) -> Result<PlayerData, Box<dyn Error>> {
        self.decode(read_gzip(self.get_path())?)
    }

    fn save_data(&self, data: PlayerData) -> Result<(), Box<dyn Error>> {
        let path = self.get_path();
        fs::copy(path, path.with_extension("dat_pm_old"))?;
        write_gzip(path, self.encode(data)?)?;

        Ok(())
    }
}

pub struct SinglePlayer {
    name: String,
    path: PathBuf,
    level: Option<LevelDat>,
}

impl SinglePlayer {
    pub fn new(name: String, path: &Path) -> Self {
        Self {
            name,
            path: path.to_path_buf(),
            level: None,
        }
    }

    pub fn from_path(path: &Path) -> Result<Self, Box<dyn Error>> {
        Ok(Self::new("Local Player".to_string(), path))
    }
}

impl Player for SinglePlayer {
    fn decode(&mut self, nbt: Vec<u8>) -> Result<PlayerData, Box<dyn Error>> {
        let level: LevelDat = fastnbt::from_bytes(&nbt)?;
        let player = level.data.player.clone();
        self.level = Some(level);
        Ok(player)
    }

    fn encode(&self, data: PlayerData) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut level = self.level.clone().ok_or("No level data")?;
        level.data.player = data;
        Ok(fastnbt::to_bytes(&level)?)
    }

    fn get_path(&self) -> &Path {
        &self.path
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

pub struct MultiPlayer {
    name: String,
    path: PathBuf,
}

impl MultiPlayer {
    pub fn new(name: String, path: &Path) -> Self {
        Self {
            name,
            path: path.to_path_buf(),
        }
    }

    pub fn from_path(path: &Path) -> Result<Self, Box<dyn Error>> {
        let uuid = path.file_stem().unwrap().to_str().unwrap();
        let name = api::fetch_player_name(uuid)?;

        Ok(Self::new(name, path))
    }
}

impl Player for MultiPlayer {
    fn decode(&mut self, nbt: Vec<u8>) -> Result<PlayerData, Box<dyn Error>> {
        Ok(fastnbt::from_bytes(&nbt)?)
    }

    fn encode(&self, data: PlayerData) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(fastnbt::to_bytes(&data)?)
    }

    fn get_path(&self) -> &Path {
        &self.path
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

pub fn get_all_player(world: &Path) -> Result<Vec<Box<dyn Player>>, Box<dyn Error>> {
    let mut players: Vec<Box<dyn Player>> =
        vec![Box::new(SinglePlayer::from_path(&world.join("level.dat"))?)];

    let builder = globmatch::Builder::new(
        "[0-9a-fA-F]*-[0-9a-fA-F]*-4[0-9a-fA-F]*-[89abAB][0-9a-fA-F]*-[0-9a-fA-F]*.dat",
    )
    .build(world.join("playerdata"))?;

    for path in builder.into_iter().flatten() {
        players.push(Box::new(MultiPlayer::from_path(&path)?));
    }

    Ok(players)
}
