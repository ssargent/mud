// @generated automatically by Diesel CLI.

pub mod player {
    diesel::table! {
        /// Representation of the `player.characters` table.
        ///
        /// (Automatically generated by Diesel.)
        player.characters (id) {
            /// The `id` column of the `player.characters` table.
            ///
            /// Its SQL type is `Uuid`.
            ///
            /// (Automatically generated by Diesel.)
            id -> Uuid,
            /// The `user_id` column of the `player.characters` table.
            ///
            /// Its SQL type is `Uuid`.
            ///
            /// (Automatically generated by Diesel.)
            user_id -> Uuid,
            /// The `character_name` column of the `player.characters` table.
            ///
            /// Its SQL type is `Varchar`.
            ///
            /// (Automatically generated by Diesel.)
            #[max_length = 32]
            character_name -> Varchar,
            /// The `class` column of the `player.characters` table.
            ///
            /// Its SQL type is `Varchar`.
            ///
            /// (Automatically generated by Diesel.)
            #[max_length = 32]
            class -> Varchar,
            /// The `character_level` column of the `player.characters` table.
            ///
            /// Its SQL type is `Int4`.
            ///
            /// (Automatically generated by Diesel.)
            character_level -> Int4,
            /// The `character_definition` column of the `player.characters` table.
            ///
            /// Its SQL type is `Jsonb`.
            ///
            /// (Automatically generated by Diesel.)
            character_definition -> Jsonb,
            /// The `created_at` column of the `player.characters` table.
            ///
            /// Its SQL type is `Timestamp`.
            ///
            /// (Automatically generated by Diesel.)
            created_at -> Timestamp,
            /// The `updated_at` column of the `player.characters` table.
            ///
            /// Its SQL type is `Timestamp`.
            ///
            /// (Automatically generated by Diesel.)
            updated_at -> Timestamp,
        }
    }
}
