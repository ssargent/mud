pub mod game {
    use chrono::NaiveDateTime;
    use diesel::prelude::*;

    use diesel::Queryable;
    use diesel::Selectable;
    use protocol::TypeSignature;
    use serde::Deserialize;
    use serde::Serialize;
    use serde_json;

    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::game_schema::game::worlds)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct World {
        pub id: i64,
        pub code: String,
        pub name: String,
        pub description: String,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl World {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        pub fn as_new_world(&self) -> NewWorld {
            NewWorld {
                code: self.code.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                created_at: self.created_at,
                updated_at: self.updated_at,
            }
        }
    }

    impl TypeSignature for World {
        fn signature(&self) -> Vec<u8> {
            let mut signature = Vec::new();
            signature.extend_from_slice(self.code.as_bytes());
            signature.extend_from_slice(self.name.as_bytes());
            signature.extend_from_slice(self.description.as_bytes());

            Self::as_hashed(signature)
        }
    }

    #[derive(Insertable, Queryable, QueryableByName, Selectable, Debug, Clone)]
    #[diesel(table_name = crate::game_schema::game::worlds)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct NewWorld {
        pub code: String,
        pub name: String,
        pub description: String,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl NewWorld {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }

    impl TypeSignature for NewWorld {
        fn signature(&self) -> Vec<u8> {
            let mut signature = Vec::new();
            signature.extend_from_slice(self.code.as_bytes());
            signature.extend_from_slice(self.name.as_bytes());
            signature.extend_from_slice(self.description.as_bytes());

            Self::as_hashed(signature)
        }
    }

    // model for game.capabilities
    #[derive(
        Insertable,
        Queryable,
        QueryableByName,
        Selectable,
        Identifiable,
        Debug,
        Clone,
        Serialize,
        Deserialize,
    )]
    #[diesel(table_name = crate::game_schema::game::capabilities)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Capability {
        pub id: i64,
        pub world_id: i64,
        pub parent_id: Option<i64>,
        pub capability_type: String,
        pub code: String,
        pub name: String,
        pub description: String,
        pub requirements: serde_json::Value,
        pub actions: Option<serde_json::Value>,
        pub access_requirements: serde_json::Value,
        pub gameplay_definition: Option<serde_json::Value>,
        pub tags: Vec<Option<String>>,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl TypeSignature for Capability {
        fn signature(&self) -> Vec<u8> {
            let mut signature = Vec::new();
            signature.extend_from_slice(&self.world_id.to_be_bytes());
            signature.extend_from_slice(self.code.as_bytes());
            signature.extend_from_slice(self.name.as_bytes());
            signature.extend_from_slice(self.description.as_bytes());
            signature.extend_from_slice(self.capability_type.as_bytes());
            signature.extend_from_slice(self.requirements.to_string().as_bytes());
            signature.extend_from_slice(
                self.actions
                    .as_ref()
                    .cloned()
                    .unwrap_or_default()
                    .to_string()
                    .as_bytes(),
            );
            signature.extend_from_slice(self.access_requirements.to_string().as_bytes());
            signature.extend_from_slice(
                self.gameplay_definition
                    .as_ref()
                    .cloned()
                    .unwrap_or_default()
                    .to_string()
                    .as_bytes(),
            );
            signature.extend_from_slice(serde_json::to_string(&self.tags).unwrap().as_bytes());

            Self::as_hashed(signature)
        }
    }

    impl Capability {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
            serde_json::from_str(json_str)
        }

        pub fn as_new_capability(&self) -> NewCapability {
            NewCapability {
                world_id: self.world_id,
                parent_id: self.parent_id,
                capability_type: self.capability_type.clone(),
                code: self.code.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                requirements: self.requirements.clone(),
                actions: self.actions.clone(),
                access_requirements: self.access_requirements.clone(),
                gameplay_definition: self.gameplay_definition.clone(),
                tags: self.tags.clone(),
            }
        }
    }

    #[derive(
        Insertable, Queryable, QueryableByName, Selectable, Debug, Clone, Serialize, Deserialize,
    )]
    #[diesel(table_name = crate::game_schema::game::capabilities)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct NewCapability {
        pub world_id: i64,
        pub parent_id: Option<i64>,
        pub capability_type: String,
        pub code: String,
        pub name: String,
        pub description: String,
        pub requirements: serde_json::Value,
        pub actions: Option<serde_json::Value>,
        pub access_requirements: serde_json::Value,
        pub gameplay_definition: Option<serde_json::Value>,
        pub tags: Vec<Option<String>>,
    }

    impl TypeSignature for NewCapability {
        fn signature(&self) -> Vec<u8> {
            let mut signature = Vec::new();
            signature.extend_from_slice(&self.world_id.to_be_bytes());
            signature.extend_from_slice(self.code.as_bytes());
            signature.extend_from_slice(self.name.as_bytes());
            signature.extend_from_slice(self.description.as_bytes());
            signature.extend_from_slice(self.capability_type.as_bytes());
            signature.extend_from_slice(self.requirements.to_string().as_bytes());
            signature.extend_from_slice(
                self.actions
                    .as_ref()
                    .cloned()
                    .unwrap_or_default()
                    .to_string()
                    .as_bytes(),
            );
            signature.extend_from_slice(self.access_requirements.to_string().as_bytes());
            signature.extend_from_slice(
                self.gameplay_definition
                    .as_ref()
                    .cloned()
                    .unwrap_or_default()
                    .to_string()
                    .as_bytes(),
            );
            signature.extend_from_slice(serde_json::to_string(&self.tags).unwrap().as_bytes());

            Self::as_hashed(signature)
        }
    }

    impl NewCapability {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
            serde_json::from_str(json_str)
        }
    }

    #[derive(
        Insertable,
        Queryable,
        QueryableByName,
        Selectable,
        Identifiable,
        Debug,
        Clone,
        Serialize,
        Deserialize,
    )]
    #[diesel(table_name = crate::game_schema::game::character_classes)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct CharacterClass {
        pub id: i64,
        pub world_id: i64,
        pub code: String,
        pub name: String,
        pub description: String,
        pub hit_points: i32,
        pub stamina_expression: String,
        pub skillpoint_expression: String,
        pub proficiencies: serde_json::Value,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl TypeSignature for CharacterClass {
        fn signature(&self) -> Vec<u8> {
            let mut signature = Vec::new();
            signature.extend_from_slice(&self.world_id.to_be_bytes());
            signature.extend_from_slice(self.code.as_bytes());
            signature.extend_from_slice(self.name.as_bytes());
            signature.extend_from_slice(self.description.as_bytes());
            signature.extend_from_slice(&self.hit_points.to_be_bytes());
            signature.extend_from_slice(self.stamina_expression.as_bytes());
            signature.extend_from_slice(self.skillpoint_expression.as_bytes());
            signature.extend_from_slice(self.proficiencies.to_string().as_bytes());

            Self::as_hashed(signature)
        }
    }

    impl CharacterClass {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        pub fn as_new_character_class(&self) -> NewCharacterClass {
            NewCharacterClass {
                world_id: self.world_id,
                code: self.code.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                hit_points: self.hit_points,
                stamina_expression: self.stamina_expression.clone(),
                skillpoint_expression: self.skillpoint_expression.clone(),
                proficiencies: self.proficiencies.clone(),
            }
        }
    }

    #[derive(
        Insertable, Queryable, QueryableByName, Selectable, Debug, Clone, Serialize, Deserialize,
    )]
    #[diesel(table_name = crate::game_schema::game::character_classes)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct NewCharacterClass {
        pub world_id: i64,
        pub code: String,
        pub name: String,
        pub description: String,
        pub hit_points: i32,
        pub stamina_expression: String,
        pub skillpoint_expression: String,
        pub proficiencies: serde_json::Value,
    }

    impl TypeSignature for NewCharacterClass {
        fn signature(&self) -> Vec<u8> {
            let mut signature = Vec::new();
            signature.extend_from_slice(&self.world_id.to_be_bytes());
            signature.extend_from_slice(self.code.as_bytes());
            signature.extend_from_slice(self.name.as_bytes());
            signature.extend_from_slice(self.description.as_bytes());
            signature.extend_from_slice(&self.hit_points.to_be_bytes());
            signature.extend_from_slice(self.stamina_expression.as_bytes());
            signature.extend_from_slice(self.skillpoint_expression.as_bytes());
            signature.extend_from_slice(self.proficiencies.to_string().as_bytes());

            Self::as_hashed(signature)
        }
    }

    impl NewCharacterClass {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }

    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::game_schema::game::character_class_features)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct CharacterClassFeature {
        pub id: i64,
        pub class_id: i64,
        pub level: i32,
        pub code: String,
        pub name: String,
        pub description: String,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl TypeSignature for CharacterClassFeature {
        fn signature(&self) -> Vec<u8> {
            let mut signature = Vec::new();
            signature.extend_from_slice(&self.class_id.to_be_bytes());
            signature.extend_from_slice(&self.level.to_be_bytes());
            signature.extend_from_slice(self.code.as_bytes());
            signature.extend_from_slice(self.name.as_bytes());
            signature.extend_from_slice(self.description.as_bytes());

            Self::as_hashed(signature)
        }
    }

    impl CharacterClassFeature {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        pub fn as_new_character_class_feature(&self) -> NewCharacterClassFeature {
            NewCharacterClassFeature {
                class_id: self.class_id,
                level: self.level,
                code: self.code.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
            }
        }
    }

    #[derive(
        Insertable, Queryable, QueryableByName, Selectable, Debug, Clone, Serialize, Deserialize,
    )]
    #[diesel(table_name = crate::game_schema::game::character_class_features)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct NewCharacterClassFeature {
        pub class_id: i64,
        pub level: i32,
        pub code: String,
        pub name: String,
        pub description: String,
    }

    impl TypeSignature for NewCharacterClassFeature {
        fn signature(&self) -> Vec<u8> {
            let mut signature = Vec::new();
            signature.extend_from_slice(&self.class_id.to_be_bytes());
            signature.extend_from_slice(&self.level.to_be_bytes());
            signature.extend_from_slice(self.code.as_bytes());
            signature.extend_from_slice(self.name.as_bytes());
            signature.extend_from_slice(self.description.as_bytes());

            Self::as_hashed(signature)
        }
    }

    impl NewCharacterClassFeature {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }

    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::game_schema::game::world_nodes)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct WorldNode {
        pub id: i64,
        pub world_id: i64,
        pub parent_id: Option<i64>,
        pub name: String,
        pub description: String,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl WorldNode {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }

    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::game_schema::game::world_node_features)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct WorldNodeFeature {
        pub id: i64,
        pub world_node_id: i64,
        pub feature_name: String,
        pub feature_value: String,
        pub feature_properties: serde_json::Value,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl WorldNodeFeature {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }

    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::game_schema::game::npc_templates)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct NpcTemplate {
        pub id: i64,
        pub name: String,
        pub description: String,
        pub npc_properties: serde_json::Value,
        pub can_spawn_multiple: bool,
        pub can_respawn: bool,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl NpcTemplate {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }

    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::game_schema::game::npc_spawn_rules)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct NpcSpawnRule {
        pub id: i64,
        pub npc_template_id: i64,
        pub world_node_id: i64,
        pub spawn_chance: i32,
        pub spawn_quantity_min: i32,
        pub spawn_quantity_max: i32,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl NpcSpawnRule {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }

    #[derive(
        Insertable,
        Queryable,
        QueryableByName,
        Selectable,
        Identifiable,
        Debug,
        Clone,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[diesel(table_name = crate::game_schema::game::items)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Item {
        pub id: i64,
        pub world_id: i64,
        pub code: String,
        pub item_type: String,
        pub category_id: i64,
        pub name: String,
        pub description: String,
        pub item_properties: serde_json::Value,
        pub base_price: i64,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl Item {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        pub fn as_new_item(&self) -> NewItem {
            NewItem {
                world_id: self.world_id,
                code: self.code.clone(),
                item_type: self.item_type.clone(),
                category_id: self.category_id,
                name: self.name.clone(),
                description: self.description.clone(),
                item_properties: self.item_properties.clone(),
                base_price: self.base_price,
            }
        }
    }

    impl TypeSignature for Item {
        fn signature(&self) -> Vec<u8> {
            let mut signature = Vec::new();
            signature.extend_from_slice(&self.world_id.to_be_bytes());
            signature.extend_from_slice(&self.category_id.to_be_bytes());
            signature.extend_from_slice(self.code.as_bytes());
            signature.extend_from_slice(self.item_type.as_bytes());
            signature.extend_from_slice(self.name.as_bytes());
            signature.extend_from_slice(self.description.as_bytes());
            signature.extend_from_slice(&self.base_price.to_be_bytes());

            Self::as_hashed(signature)
        }
    }

    #[derive(Insertable, Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[diesel(table_name = crate::game_schema::game::items)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct NewItem {
        pub world_id: i64,
        pub code: String,
        pub item_type: String,
        pub category_id: i64,
        pub name: String,
        pub description: String,
        pub item_properties: serde_json::Value,
        pub base_price: i64,
    }

    impl NewItem {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }

    #[derive(
        Insertable,
        Queryable,
        QueryableByName,
        Selectable,
        Identifiable,
        Debug,
        Clone,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[diesel(table_name = crate::game_schema::game::item_categories)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct ItemCategory {
        pub id: i64,
        pub parent_id: Option<i64>,
        pub name: String,
        pub description: String,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl ItemCategory {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }

    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::game_schema::game::attributes)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Attribute {
        pub id: i64,
        pub name: String,
        pub description: String,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl Attribute {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }

    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::game_schema::game::races)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Race {
        pub id: i64,
        pub world_id: i64,
        pub code: String,
        pub name: String,
        pub description: String,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl Race {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }

    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::game_schema::game::currency)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Currency {
        pub id: i64,
        pub world_id: i64,
        pub code: String,
        pub name: String,
        pub description: String,
        pub is_spendable: bool,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl Currency {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }

    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::game_schema::game::enemies)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Enemy {
        pub id: i64,
        pub world_id: i64,
        pub code: String,
        pub name: String,
        pub description: String,
        pub class: String,
        pub level: i32,
        pub hit_points: i32,
        pub stamina: i32,
        pub strength: i32,
        pub dexterity: i32,
        pub constitution: i32,
        pub intelligence: i32,
        pub wisdom: i32,
        pub weapons: serde_json::Value,
        pub armor: serde_json::Value,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl Enemy {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        pub fn as_new_enemy(&self) -> NewEnemy {
            NewEnemy {
                world_id: self.world_id,
                code: self.code.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                class: self.class.clone(),
                level: self.level,
                hit_points: self.hit_points,
                stamina: self.stamina,
                strength: self.strength,
                dexterity: self.dexterity,
                constitution: self.constitution,
                intelligence: self.intelligence,
                wisdom: self.wisdom,
                weapons: self.weapons.clone(),
                armor: self.armor.clone(),
            }
        }
    }

    impl TypeSignature for Enemy {
        fn signature(&self) -> Vec<u8> {
            let mut signature = Vec::new();
            signature.extend_from_slice(&self.world_id.to_be_bytes());
            signature.extend_from_slice(self.code.as_bytes());
            signature.extend_from_slice(self.name.as_bytes());
            signature.extend_from_slice(self.description.as_bytes());
            signature.extend_from_slice(self.class.as_bytes());
            signature.extend_from_slice(&self.level.to_be_bytes());
            signature.extend_from_slice(&self.hit_points.to_be_bytes());
            signature.extend_from_slice(&self.stamina.to_be_bytes());
            signature.extend_from_slice(&self.strength.to_be_bytes());
            signature.extend_from_slice(&self.dexterity.to_be_bytes());
            signature.extend_from_slice(&self.constitution.to_be_bytes());
            signature.extend_from_slice(&self.intelligence.to_be_bytes());
            signature.extend_from_slice(&self.wisdom.to_be_bytes());
            signature.extend_from_slice(self.weapons.to_string().as_bytes());
            signature.extend_from_slice(self.armor.to_string().as_bytes());

            Self::as_hashed(signature)
        }
    }

    #[derive(Insertable, Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[diesel(table_name = crate::game_schema::game::enemies)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct NewEnemy {
        pub world_id: i64,
        pub code: String,
        pub name: String,
        pub description: String,
        pub class: String,
        pub level: i32,
        pub hit_points: i32,
        pub stamina: i32,
        pub strength: i32,
        pub dexterity: i32,
        pub constitution: i32,
        pub intelligence: i32,
        pub wisdom: i32,
        pub weapons: serde_json::Value,
        pub armor: serde_json::Value,
    }

    impl NewEnemy {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }
}
