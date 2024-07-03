pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

pub trait PhysicalObject {
    fn get_coordinates() -> Vec<i32>;
    fn surface_area() -> i32;
    fn weight() -> i32;
    fn height() -> i32;
    fn width() -> i32;
    fn temperature() -> i32;
    fn velocity() -> Velocity;
}
