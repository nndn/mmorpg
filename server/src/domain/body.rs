pub struct Body {
    parts: HashMap<String, Box<Part>>,
}

pub enum PartType {
    Head,
    Body,
    Hand,
    Leg,
}
