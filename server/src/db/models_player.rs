pub mod player {
    use chrono::NaiveDateTime;
    use diesel::prelude::*;
    use diesel::Queryable;
    use diesel::Selectable;
    use serde_json;

    #[derive(Insertable, Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[diesel(table_name = crate::player_schema::player::entitlements)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct NewEntitlement {
        pub name: String,
        pub code: String,
        pub description: String,
        pub world_id: i64,
        pub entitlement_type: String,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl NewEntitlement {
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
    #[diesel(table_name = crate::player_schema::player::entitlements)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Entitlement {
        pub id: i64,
        pub name: String,
        pub code: String,
        pub description: String,
        pub world_id: i64,
        pub entitlement_type: String,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl Entitlement {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }

    #[derive(Insertable, Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[diesel(table_name = crate::player_schema::player::entitlement_mappings)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct NewEntitlementMapping {
        pub entitlement_id: i64,
        pub user_id: i64,
        pub is_consumable: bool,
        pub is_consumed: bool,
        pub start_date: NaiveDateTime,
        pub end_date: Option<NaiveDateTime>,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl NewEntitlementMapping {
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
        Associations,
        Debug,
        Clone,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[diesel(belongs_to(Entitlement))]
    #[diesel(table_name = crate::player_schema::player::entitlement_mappings)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct EntitlementMapping {
        pub id: i64,
        pub entitlement_id: i64,
        pub user_id: i64,
        pub is_consumable: bool,
        pub is_consumed: bool,
        pub start_date: NaiveDateTime,
        pub end_date: Option<NaiveDateTime>,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl EntitlementMapping {
        // as_json returns a serialized json string of the Setting struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }

    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::player_schema::player::characters)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Character {
        pub id: i64,
        pub world_id: i64,
        pub user_id: i64,
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

        pub fn as_protocol_character(&self) -> protocol::types::character::Character {
            protocol::types::character::Character {
                id: Some(self.id),
                world_id: Some(self.world_id),
                user_id: Some(self.user_id),
                name: self.name.clone(),
                class: self.class.clone(),
                theme: self.theme.clone(),
                level: self.level,
                experience: self.experience, // Cast to i32 for compatibility
                hit_points: self.hit_points,
                stamina: self.stamina,
                abilities: serde_json::from_value(self.abilities.clone()).unwrap(),
                feats: serde_json::from_value(self.feats.clone()).unwrap(),
                skills: serde_json::from_value(self.skills.clone()).unwrap(),
            }
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
