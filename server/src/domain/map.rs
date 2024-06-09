use std::collections::HashMap;

#[derive(Debug)]
pub struct Map {
    pub submaps: HashMap<String, SubMap>,
}

#[derive(Debug)]
pub struct SubMap {
    pub cells: Vec<Vec<Cell>>,
}

#[derive(Debug)]
pub struct Cell {
    // index corresponds to z-index
    pub cell_contents: Vec<CellLayer>,
}

#[derive(Debug)]
pub enum CellLayer {
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

pub struct MapManager {}
