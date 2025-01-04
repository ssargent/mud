pub mod system {
    use chrono::NaiveDateTime;
    use diesel::prelude::*;
    use diesel::Queryable;
    use diesel::Selectable;
    use serde_json;
    use uuid::Uuid;
    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::system_schema::system::users)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct User {
        pub id: Uuid,
        pub username: String,
        #[serde(skip_serializing)]
        pub password: String,
        pub email: String,
        pub full_name: String,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl User {
        // as_json returns a serialized json string of the User struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }

    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::system_schema::system::settings)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Setting {
        pub id: Uuid,
        pub name: String,
        pub data_type: String,
        pub value: String,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl Setting {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }
}

pub mod game {
    use chrono::NaiveDateTime;
    use diesel::prelude::*;
    use diesel::sql_types::Jsonb;
    use diesel::Queryable;
    use diesel::Selectable;
    use protocol::TypeSignature;
    use serde_json;
    use uuid::Uuid;

    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::game_schema::game::worlds)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct World {
        pub id: i64,
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
        pub id: Uuid,
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
        pub id: Uuid,
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
}

pub mod player {
    use chrono::NaiveDateTime;
    use diesel::prelude::*;
    use diesel::Queryable;
    use diesel::Selectable;
    use serde_json;
    use uuid::Uuid;

    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::player_schema::player::characters)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Character {
        pub id: i64,
        pub world_id: i64,
        pub user_id: Uuid,
        pub race_id: i64,
        pub name: String,
        pub class: String,
        pub theme: String,
        pub level: i32,
        pub experience: i64,
        pub hit_points: i32,
        pub stamina: i32,
        pub abilities: serde_json::Value,
        pub feats: serde_json::Value,
        pub skills: serde_json::Value,
    }

    impl Character {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }

    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::player_schema::player::character_inventory)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct CharacterInventory {
        pub id: i64,
        pub character_id: i64,
        pub item_id: i64,
        pub quantity: i32,
    }

    impl CharacterInventory {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }

    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::player_schema::player::character_currency_ledger)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct CharacterCurrencyLedger {
        pub id: i64,
        pub character_id: i64,
        pub currency_id: i64,
        pub entry_type: String,
        pub amount: i32,
        pub created_at: NaiveDateTime,
        pub memo: String,
    }

    impl CharacterCurrencyLedger {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }
}
