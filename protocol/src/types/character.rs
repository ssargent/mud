use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: Option<i64>,
    pub code: Option<String>,
    pub world_id: Option<i64>,
    pub user_id: Option<i64>,
    pub name: String,
    pub class: String,
    pub theme: String,
    pub level: i32,
    pub experience: i32,
    pub hit_points: i32,
    pub stamina: i32,
    pub abilities: AbilityScores,
    pub feats: Vec<Feat>,
    pub skills: Vec<CharacterSkill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityScores {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feat {
    pub id: Option<i64>,
    pub code: Option<String>,
    pub world_id: Option<i64>,
    pub name: String,
    pub description: String,
    pub prerequisites: Option<String>,
    pub benefits: String,
    pub special: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: Option<i64>,
    pub code: Option<String>,
    pub world_id: Option<i64>,
    pub name: String,
    pub description: String,
    pub ability: String,
    pub trained_only: bool,
    pub armor_penalty: bool,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSkill {
    pub skill_id: i64,
    pub is_class_skill: bool,
    pub ranks: i32,
}
