use super::common::ID;

#[derive(Debug)]
pub struct Position {
    pub current_map: ID,
    pub current_sub_map: ID,
    pub position_x: i32,
    pub position_y: i32,
    pub direction: Direction,
}

#[derive(Debug, Default)]
pub enum Direction {
    #[default]
    Down,
    Up,
    Left,
    Right,
}
