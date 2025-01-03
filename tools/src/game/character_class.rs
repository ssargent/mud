use protocol::TypeSignature;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterClassSpec {
    pub id: Option<i64>,
    pub world_id: Option<i64>,
    pub code: Option<String>,
    pub name: String,
    pub description: String,
    pub hit_points: i64,
    pub stamina_expression: String,
    pub skillpoint_expression: String,
    pub proficiencies: Option<Vec<String>>,
    pub features: Option<Vec<CharacterClassFeature>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterClassFeature {
    pub level: i32,
    pub name: String,
    pub description: String,
}

impl TypeSignature for CharacterClassSpec {
    fn signature(&self) -> Vec<u8> {
        let mut signature = Vec::new();
        signature.extend_from_slice(&self.world_id.unwrap_or(0).to_be_bytes());
        signature.extend_from_slice(self.code.clone().unwrap_or("".to_string()).as_bytes());
        signature.extend_from_slice(self.name.as_bytes());
        signature.extend_from_slice(self.description.as_bytes());
        signature.extend_from_slice(&self.hit_points.to_be_bytes());
        signature.extend_from_slice(self.stamina_expression.as_bytes());
        signature.extend_from_slice(self.skillpoint_expression.as_bytes());

        if let Some(proficiencies) = &self.proficiencies {
            for proficiency in proficiencies {
                signature.extend_from_slice(proficiency.as_bytes());
            }
        }
        if let Some(features) = &self.features {
            for feature in features {
                signature.extend_from_slice(&feature.level.to_be_bytes());
                signature.extend_from_slice(feature.name.as_bytes());
                signature.extend_from_slice(feature.description.as_bytes());
            }
        }

        Self::as_hashed(signature)
    }
}

impl CharacterClassSpec {
    pub fn is_valid(&self) -> bool {
        self.world_id.unwrap_or(0) > 0
            && !self.code.clone().unwrap_or("".to_string()).is_empty()
            && !self.name.is_empty()
            && !self.description.is_empty()
            && self.hit_points > 0
            && !self.stamina_expression.is_empty()
            && !self.skillpoint_expression.is_empty()
            && self.proficiencies.as_ref().map_or(true, |proficiencies| {
                proficiencies
                    .iter()
                    .all(|proficiency| !proficiency.is_empty())
            })
            && self.features.as_ref().map_or(true, |features| {
                features.iter().all(|feature| feature.is_valid())
            })
    }
}

impl CharacterClassFeature {
    pub fn is_valid(&self) -> bool {
        self.level > 0 && !self.name.is_empty() && !self.description.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanic_character_class() {
        let mechanic = CharacterClassSpec {
            id: Some(1),
            world_id: Some(1),
            code: Some("mechanic".to_string()),
            name: "Mechanic".to_string(),
            description: "A master of machines and technology.".to_string(),
            hit_points: 6,
            stamina_expression: "10 + CON".to_string(),
            skillpoint_expression: "4 + INT".to_string(),
            proficiencies: Some(vec![
                "Light Armor".to_string(),
                "Basic Melee Weapons".to_string(),
                "Small Arms".to_string(),
            ]),
            features: Some(vec![
                CharacterClassFeature {
                    level: 1,
                    name: "Artificial Intelligence".to_string(),
                    description: "You have an AI that assists you.".to_string(),
                },
                CharacterClassFeature {
                    level: 1,
                    name: "Custom Rig".to_string(),
                    description: "You have a custom rig for your tools.".to_string(),
                },
            ]),
        };

        assert!(mechanic.is_valid());
    }

    #[test]
    fn test_mechanic_character_class_json() {
        let mechanic_json = r#"
        {
            "id": 1,
            "world_id": 1,
            "code": "mechanic",
            "name": "Mechanic",
            "description": "A master of machines and technology.",
            "hit_points": 6,
            "stamina_expression": "10 + CON",
            "skillpoint_expression": "4 + INT",
            "proficiencies": ["Light Armor", "Basic Melee Weapons", "Small Arms"],
            "features": [
                {
                    "level": 1,
                    "name": "Artificial Intelligence",
                    "description": "You have an AI that assists you."
                },
                {
                    "level": 1,
                    "name": "Custom Rig",
                    "description": "You have a custom rig for your tools."
                }
            ]
        }
        "#;

        let mechanic: CharacterClassSpec = serde_json::from_str(mechanic_json).unwrap();
        assert!(mechanic.is_valid());
    }
}
