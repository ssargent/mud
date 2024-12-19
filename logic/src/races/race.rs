use crate::abilities::Ability;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Race {
    // Eventually, we will use a UUID to identify
    // Races, since this will come from a database.
    //    pub id: uuid::Uuid,
    pub name: String,
    pub ability_modifiers: HashMap<Ability, i32>,
    pub special_abilities: Vec<String>,
    pub speed: i32,
}

impl Race {
    pub fn new(
        name: &str,
        ability_modifiers: HashMap<Ability, i32>,
        special_abilities: Vec<String>,
        speed: i32,
    ) -> Self {
        Self {
            name: name.to_string(),
            ability_modifiers,
            special_abilities,
            speed,
        }
    }
}
