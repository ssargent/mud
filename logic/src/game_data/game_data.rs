use std::collections::HashMap;

pub struct Weapon {
    pub name: String,
    pub damage: i32,
    pub damage_type: String,
    pub range: i32,
    pub properties: Vec<String>,
}

pub enum EntityTypes {
    Weapon(String),
    Enemy(String),
}

pub struct Enemy {
    pub name: String,
    pub hit_points: i32,
    pub armor_class: i32,
    pub weapons: Vec<Weapon>,
}

pub struct GameData {
    weapons: HashMap<String, Weapon>,
    enemies: HashMap<String, Enemy>,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            weapons: HashMap::new(),
            enemies: HashMap::new(),
        }
    }

    /// Get a reference to an entity by name.
    /// 
    /// # Arguments
    /// 
    /// * `entity_type` - The type of entity to get.
    /// 
    /// # Returns
    /// 
    /// An option containing a reference to the entity, if it exists.
    /// 
    /// # Example
    /// 
    /// ```
    /// use crate::game_data::game_data::{GameData, EntityTypes};
    /// 
    /// let mut game_data = GameData::new();
    /// 
    /// let weapon = game_data.get::<Weapon>(EntityTypes::Weapon("Dagger".to_string()));
    /// ```
    pub fn get<T>(&self, entity_type: EntityTypes) -> Option<&T>
    where 
        T: 'static,
        {
            match entity_type {
                EntityTypes::Weapon(name) => self.weapons.get(&name).map(|w| w as &T),
                EntityTypes::Enemy(name) => self.enemies.get(&name).map(|e| e as &T),
            }
        }
}