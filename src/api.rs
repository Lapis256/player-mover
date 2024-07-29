use std::error::Error;

use serde::Deserialize;

#[derive(Deserialize)]
struct Player {
    name: String,
}

pub fn fetch_player_name(uuid: &str) -> Result<String, Box<dyn Error>> {
    let player: Player = ureq::get(&format!(
        "https://sessionserver.mojang.com/session/minecraft/profile/{}",
        uuid.replace('-', "").to_lowercase()
    ))
    .call()?
    .into_json()?;

    Ok(player.name)
}

#[test]
fn test_fetch_player_name() {
    let name = fetch_player_name("069a79f4-44e9-4726-a5be-fca90e38aaf5").unwrap();
    assert_eq!(name, "Notch");
}
