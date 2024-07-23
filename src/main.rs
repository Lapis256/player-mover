mod dat;

use std::{
    error::Error,
    fs::{self},
    path::PathBuf,
};

use clap::Parser;
use dialoguer::Select;
use mojang::Player;

use dat::{get_spawn_point, read_player_data, write_player_data};

#[derive(Debug, Parser)]
#[command(author, version, long_about = None, arg_required_else_help = true)]
struct Cli {
    /// The path to the world directory
    world: PathBuf,

    /// The position to move the player to. Format: x,y,z. If not provided, the player will be moved to the world spawn point.
    #[arg(value_parser = parse_pos)]
    change_position: Option<[f64; 3]>,

    /// The dimension to move the player to. If not provided, the player will be moved to the overworld.
    change_dimension: Option<String>,
}

fn parse_pos(s: &str) -> Result<[f64; 3], String> {
    let parts: Result<Vec<f64>, _> = s
        .split_whitespace()
        .collect::<String>()
        .split(',')
        .take(3)
        .map(|part| part.parse().map_err(|_| "not an integer".to_string()))
        .collect();

    match parts.as_deref() {
        Ok([x, y, z]) => Ok([*x, *y, *z]),
        _ => Err("missing or extra coordinates".to_string()),
    }
}

fn select_player(player_dat_paths: Vec<PathBuf>) -> Result<PathBuf, Box<dyn Error>> {
    let players = player_dat_paths
        .iter()
        .map(|p| p.file_stem().unwrap().to_str().unwrap().to_lowercase())
        .filter_map(|p| Some(Player::new(p).ok()?.name))
        .collect::<Vec<_>>();

    let choice: usize = Select::new()
        .items(&players)
        .default(0)
        .with_prompt("Select a player")
        .interact()?;

    Ok(player_dat_paths[choice].clone())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let path = &cli.world;
    let player_data = path.join("playerdata");

    let builder = globmatch::Builder::new(
        "[0-9a-fA-F]*-[0-9a-fA-F]*-4[0-9a-fA-F]*-[89abAB][0-9a-fA-F]*-[0-9a-fA-F]*.dat",
    )
    .build(player_data)?;
    let player_dat_paths = builder.into_iter().flatten().collect::<Vec<_>>();
    let player_path = select_player(player_dat_paths)?;

    fs::copy(&player_path, player_path.with_extension("dat_pm_old"))?;

    let mut player_data = read_player_data(&player_path).expect("Failed to read player data");

    player_data.pos = cli
        .change_position
        .unwrap_or(get_spawn_point(path).expect("Failed to get spawn point"));
    player_data.dimension = cli
        .change_dimension
        .unwrap_or("minecraft:overworld".to_string());

    write_player_data(&player_path, &player_data).expect("Failed to write player data");

    println!("Player moved successfully");
    println!(
        "Old player data backed up to: {}",
        player_path.with_extension("dat_pm_old").to_string_lossy()
    );

    Ok(())
}
