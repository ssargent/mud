pub mod system {
    use chrono::NaiveDateTime;
    use diesel::prelude::*;
    use diesel::Queryable;
    use diesel::Selectable;
    use protocol::TypeSignature;
    use serde_json;

    #[derive(serde::Serialize, serde::Deserialize, Debug, Queryable)]
    pub struct ActiveUserRole {
        pub role_name: String,
        pub is_read_only: bool,
    }

    /** ApiKey */
    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::system_schema::system::user_api_keys)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct UserApiKey {
        pub id: i64,
        pub user_id: i64,
        pub key_type: String,
        pub api_key: String,
        pub private_key: Option<String>,
        pub expiration: Option<NaiveDateTime>,
        pub created_at: NaiveDateTime,
        pub created_by: String,
        pub updated_at: NaiveDateTime,
        pub updated_by: String,
    }

    impl TypeSignature for UserApiKey {
        fn signature(&self) -> Vec<u8> {
            let mut signature = Vec::new();
            signature.extend_from_slice(self.user_id.to_string().as_bytes());
            signature.extend_from_slice(self.key_type.as_bytes());
            signature.extend_from_slice(self.api_key.as_bytes());
            Self::as_hashed(signature)
        }
    }

    impl UserApiKey {
        // as_json returns a serialized json string of the UserApiKey struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        // from_json returns a UserApiKey struct from a json string.
        pub fn from_json(json: &str) -> Self {
            serde_json::from_str(json).unwrap()
        }
    }

    /** New ApiKey */
    #[derive(Insertable, Queryable, QueryableByName, Debug, Clone)]
    #[diesel(table_name = crate::system_schema::system::user_api_keys)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct NewUserApiKey {
        pub user_id: i64,
        pub key_type: String,
        pub api_key: String,
        pub private_key: Option<String>,
        pub expiration: Option<NaiveDateTime>,
        pub created_at: NaiveDateTime,
        pub created_by: String,
        pub updated_at: NaiveDateTime,
        pub updated_by: String,
    }

    impl TypeSignature for NewUserApiKey {
        fn signature(&self) -> Vec<u8> {
            let mut signature = Vec::new();
            signature.extend_from_slice(self.user_id.to_string().as_bytes());
            signature.extend_from_slice(self.key_type.as_bytes());
            signature.extend_from_slice(self.api_key.as_bytes());
            Self::as_hashed(signature)
        }
    }

    impl NewUserApiKey {
        // as_json returns a serialized json string of the UserApiKey struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        // from_json returns a UserApiKey struct from a json string.
        pub fn from_json(json: &str) -> Self {
            serde_json::from_str(json).unwrap()
        }
    }

    #[derive(Insertable, Queryable, QueryableByName, Debug, Clone)]
    #[diesel(table_name = crate::system_schema::system::users)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct NewUser {
        pub username: String,
        #[serde(skip_serializing)]
        pub password: String,
        pub email: String,
        pub full_name: String,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl NewUser {
        // as_json returns a serialized json string of the User struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }
    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::system_schema::system::users)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct User {
        pub id: i64,
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
        pub id: i64,
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

    /** Permission */
    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::system_schema::system::permissions)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Permission {
        pub id: i64,
        pub name: String,
        pub description: String,
        pub created_at: NaiveDateTime,
        pub created_by: String,
        pub updated_at: NaiveDateTime,
        pub updated_by: String,
    }

    impl TypeSignature for Permission {
        fn signature(&self) -> Vec<u8> {
            let mut signature = Vec::new();
            signature.extend_from_slice(self.name.as_bytes());
            signature.extend_from_slice(self.description.as_bytes());
            Self::as_hashed(signature)
        }
    }

    impl Permission {
        // as_json returns a serialized json string of the Permission struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        // from_json returns a Permission struct from a json string.
        pub fn from_json(json: &str) -> Self {
            serde_json::from_str(json).unwrap()
        }
    }

    /* New Permission */
    #[derive(Insertable, Queryable, QueryableByName, Debug, Clone)]
    #[diesel(table_name = crate::system_schema::system::permissions)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct NewPermission {
        pub name: String,
        pub description: String,
        pub created_at: NaiveDateTime,
        pub created_by: String,
        pub updated_at: NaiveDateTime,
        pub updated_by: String,
    }

    impl TypeSignature for NewPermission {
        fn signature(&self) -> Vec<u8> {
            let mut signature = Vec::new();
            signature.extend_from_slice(self.name.as_bytes());
            signature.extend_from_slice(self.description.as_bytes());
            Self::as_hashed(signature)
        }
    }

    impl NewPermission {
        // as_json returns a serialized json string of the User struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        // from_json returns a Permission struct from a json string.
        pub fn from_json(json: &str) -> Self {
            serde_json::from_str(json).unwrap()
        }
    }

    /** Role */
    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::system_schema::system::roles)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Role {
        pub id: i64,
        pub name: String,
        pub description: String,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl TypeSignature for Role {
        fn signature(&self) -> Vec<u8> {
            let mut signature = Vec::new();
            signature.extend_from_slice(self.name.as_bytes());
            signature.extend_from_slice(self.description.as_bytes());
            Self::as_hashed(signature)
        }
    }

    impl Role {
        // as_json returns a serialized json string of the Role struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        // from_json returns a Role struct from a json string.
        pub fn from_json(json: &str) -> Self {
            serde_json::from_str(json).unwrap()
        }
    }

    /** New Role */
    #[derive(Insertable, Queryable, QueryableByName, Debug, Clone)]
    #[diesel(table_name = crate::system_schema::system::roles)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct NewRole {
        pub name: String,
        pub description: String,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

    impl TypeSignature for NewRole {
        fn signature(&self) -> Vec<u8> {
            let mut signature = Vec::new();
            signature.extend_from_slice(self.name.as_bytes());
            signature.extend_from_slice(self.description.as_bytes());
            Self::as_hashed(signature)
        }
    }

    impl NewRole {
        // as_json returns a serialized json string of the User struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        // from_json returns a Role struct from a json string.
        pub fn from_json(json: &str) -> Self {
            serde_json::from_str(json).unwrap()
        }
    }

    /** Role Permission */
    #[derive(Insertable, Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
    #[diesel(table_name = crate::system_schema::system::role_permissions)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct RolePermission {
        pub id: i64,
        pub role_id: i64,
        pub permission_id: i64,
        pub created_at: NaiveDateTime,
        pub created_by: String,
        pub updated_at: NaiveDateTime,
        pub updated_by: String,
    }

    impl TypeSignature for RolePermission {
        fn signature(&self) -> Vec<u8> {
            let mut signature = Vec::new();
            signature.extend_from_slice(self.role_id.to_string().as_bytes());
            signature.extend_from_slice(self.permission_id.to_string().as_bytes());
            Self::as_hashed(signature)
        }
    }

    impl RolePermission {
        // as_json returns a serialized json string of the RolePermission struct.
        pub fn as_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        // from_json returns a RolePermission struct from a json string.
        pub fn from_json(json: &str) -> Self {
            serde_json::from_str(json).unwrap()
        }
    }
}
