use crate::domain::map::{SubMap, Tile, TileLayer};
use bevy_ecs::system::Commands;

pub fn load_map(mut commands: Commands) {
    println!("loading map");
    commands.spawn(SubMap {
        tile_map: vec![vec![Tile {
            layers: vec![TileLayer::Grass(
                crate::domain::map::GrassType::FlatGreenYellow,
            )],
        }]],
    });
}

pub fn unload_map(mut _commands: Commands) {
    println!("unloading map");
}

pub fn update_map(mut _commands: Commands) {
    // println!("update map");
}
