mod api;
mod dat;
mod player;

use std::{error::Error, path::PathBuf};

use clap::Parser;
use dialoguer::Select;

use dat::get_spawn_point;
use player::get_all_player;

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

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let path = &cli.world;
    let mut players = get_all_player(path)?;

    let choice: usize = Select::new()
        .items(
            &players
                .iter()
                .map(|player| player.get_name())
                .collect::<Vec<_>>(),
        )
        .default(0)
        .with_prompt("Select a player")
        .interact()?;

    let player = players[choice].as_mut();

    let mut player_data = player.load_data().expect("Failed to read player data");

    player_data.pos = cli
        .change_position
        .unwrap_or(get_spawn_point(path).expect("Failed to get spawn point"));
    player_data.dimension = cli
        .change_dimension
        .unwrap_or("minecraft:overworld".to_string());

    player
        .save_data(player_data)
        .expect("Failed to write player data");

    println!("Player moved successfully");
    println!(
        "Old player data backed up to: {}",
        player
            .get_path()
            .with_extension("dat_pm_old")
            .to_string_lossy()
    );

    Ok(())
}
