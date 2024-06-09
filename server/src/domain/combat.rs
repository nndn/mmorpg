use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub struct Combat {
    pub combatant_states: HashMap<String, Box<CombatantState>>,
    pub environment: Environment,
}

#[derive(Debug)]
pub struct CombatantState {
    pub combatant: dyn Combatant,
}

pub trait Combatant: Debug {
    fn get_name(&self) -> &str;
}

#[derive(Debug)]
pub struct Environment {
    // these values are supposed to be between 0-100
    pub sunlight: i32,
    pub temperature: i32,
    pub wind: i32,
    pub rain: i32,
    pub lightning: i32,
    pub dust: i32,
}

impl Default for Environment {
    fn default() -> Self {
        return Environment {
            sunlight: 50,
            temperature: 50,
            wind: 20,
            rain: 0,
            lightning: 0,
            dust: 10,
        };
    }
}
