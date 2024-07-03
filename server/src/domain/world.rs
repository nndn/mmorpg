pub struct PlayersCurrentKnownWorld {
    pub me: Me,
    pub rest: Vec<Character>,
}

pub struct Me {
    pub id: i32,
    pub name: String,
}

pub struct Character {
    pub id: i32,
    pub name: String,
}
