use std::collections::HashMap;

use bevy_ecs::component::Component;

#[derive(Debug)]
pub struct Map {
    pub submaps: HashMap<String, SubMap>,
}

#[derive(Debug, Component)]
pub struct SubMap {
    pub tile_map: Vec<Vec<Tile>>,
}

#[derive(Debug)]
pub struct Tile {
    // index corresponds to z-index
    pub layers: Vec<TileLayer>,
}

#[derive(Debug)]
pub enum TileLayer {
    Grass(GrassType),
    Rock(RockType),
    Tree(TreeType),
    Land(LandType),
    Path(PathType, PathDirection),
}

#[derive(Debug)]
pub enum LandType {
    SuperDry,
    Dry,
    Normal,
    Wet,
    Soil,
    Sand,
    HeavySand,
    QuickSand,
    Rocky,
    VeryRocky,
}

#[derive(Debug)]
pub enum GrassType {
    FlatGreen,
    FlatGreenYellow,
    FlatYellow,
    BushGreen,
    BushGreenYellow,
    BushFlatYellow,
    TallBushGreen,
    TallBushGreenYellow,
    TallBushFlatYellow,
}

#[derive(Debug)]
pub enum RockType {
    Boulder,
}

#[derive(Debug)]
pub enum TreeType {
    Pine,
    Christmas,
}

#[derive(Debug)]
pub enum PathType {
    Kaccha,
    Stoned,
    Pakka,
}

#[derive(Debug)]
pub enum PathDirection {
    Vertical,
    Horizontal,
    UpToRight,
    UpToLeft,
    DownToRight,
    DownToLeft,
}
