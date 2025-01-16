use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemySpec {
    pub id: Option<i64>,
    pub code: Option<String>,
    pub name: String,
    pub class: String,
    pub level: i32,
    pub description: String,
    pub hit_points: i32,
    pub stamina: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
    pub weapons: Option<Vec<serde_json::Value>>,
    pub armor: Option<serde_json::Value>,
}
