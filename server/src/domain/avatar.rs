use super::{common::ID, position::Position};

// Character a player controls
#[derive(Debug)]
pub struct Avatar {
    // details
    pub id: ID,
    pub name: String,
    pub sprite_sheet: String,

    // position
    pub position: Position,

    // stats
    pub affinity: Box<[Art]>,
    pub base_stats: BaseStats,
}

#[derive(Debug)]
pub enum Art {
    DarkMagic,
    Magic,
    MarshalArts,
    Swordsmenship,
    Fire,
    Water,
}

#[derive(Debug)]
pub struct BaseStats {
    pub speed: i32,
    pub defence: i32,
    pub stamina: i32,
    pub mana: i32,
    pub strength: i32,
}
