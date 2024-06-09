use super::{
    account::Account,
    avatar::{Avatar, BaseStats},
    combat::Combatant,
    map::{Map, SubMap},
    position::Position,
};

#[derive(Debug)]
pub struct Session {
    pub account: Account,
    pub avatar: Avatar,

    pub map: Map,
    pub submap: SubMap,
    pub position: Position,

    pub session_stats: SessionStats,
}

#[derive(Debug)]
pub struct SessionStats {
    // base stats of avatar may change in the session often
    // ex: some special move that hurts the user, it'll take time to recover
    pub base_stats: BaseStats,

    pub stamina: i64,
}

impl Combatant for Session {
    fn get_name(&self) -> &str {
        return self.avatar.name.as_str();
    }
}
