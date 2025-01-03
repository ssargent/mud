// @generated automatically by Diesel CLI.

pub mod player {
    diesel::table! {
        /// Representation of the `player.character_currency_ledger` table.
        ///
        /// (Automatically generated by Diesel.)
        player.character_currency_ledger (id) {
            /// The `id` column of the `player.character_currency_ledger` table.
            ///
            /// Its SQL type is `Int8`.
            ///
            /// (Automatically generated by Diesel.)
            id -> Int8,
            /// The `character_id` column of the `player.character_currency_ledger` table.
            ///
            /// Its SQL type is `Int8`.
            ///
            /// (Automatically generated by Diesel.)
            character_id -> Int8,
            /// The `currency_id` column of the `player.character_currency_ledger` table.
            ///
            /// Its SQL type is `Int8`.
            ///
            /// (Automatically generated by Diesel.)
            currency_id -> Int8,
            /// The `entry_type` column of the `player.character_currency_ledger` table.
            ///
            /// Its SQL type is `Varchar`.
            ///
            /// (Automatically generated by Diesel.)
            #[max_length = 32]
            entry_type -> Varchar,
            /// The `amount` column of the `player.character_currency_ledger` table.
            ///
            /// Its SQL type is `Int4`.
            ///
            /// (Automatically generated by Diesel.)
            amount -> Int4,
            /// The `created_at` column of the `player.character_currency_ledger` table.
            ///
            /// Its SQL type is `Timestamp`.
            ///
            /// (Automatically generated by Diesel.)
            created_at -> Timestamp,
            /// The `memo` column of the `player.character_currency_ledger` table.
            ///
            /// Its SQL type is `Text`.
            ///
            /// (Automatically generated by Diesel.)
            memo -> Text,
        }
    }

    diesel::table! {
        /// Representation of the `player.character_inventory` table.
        ///
        /// (Automatically generated by Diesel.)
        player.character_inventory (id) {
            /// The `id` column of the `player.character_inventory` table.
            ///
            /// Its SQL type is `Int8`.
            ///
            /// (Automatically generated by Diesel.)
            id -> Int8,
            /// The `character_id` column of the `player.character_inventory` table.
            ///
            /// Its SQL type is `Int8`.
            ///
            /// (Automatically generated by Diesel.)
            character_id -> Int8,
            /// The `item_id` column of the `player.character_inventory` table.
            ///
            /// Its SQL type is `Int8`.
            ///
            /// (Automatically generated by Diesel.)
            item_id -> Int8,
            /// The `quantity` column of the `player.character_inventory` table.
            ///
            /// Its SQL type is `Int4`.
            ///
            /// (Automatically generated by Diesel.)
            quantity -> Int4,
        }
    }

    diesel::table! {
        /// Representation of the `player.characters` table.
        ///
        /// (Automatically generated by Diesel.)
        player.characters (id) {
            /// The `id` column of the `player.characters` table.
            ///
            /// Its SQL type is `Int8`.
            ///
            /// (Automatically generated by Diesel.)
            id -> Int8,
            /// The `world_id` column of the `player.characters` table.
            ///
            /// Its SQL type is `Int8`.
            ///
            /// (Automatically generated by Diesel.)
            world_id -> Int8,
            /// The `user_id` column of the `player.characters` table.
            ///
            /// Its SQL type is `Uuid`.
            ///
            /// (Automatically generated by Diesel.)
            user_id -> Uuid,
            /// The `race_id` column of the `player.characters` table.
            ///
            /// Its SQL type is `Int8`.
            ///
            /// (Automatically generated by Diesel.)
            race_id -> Int8,
            /// The `name` column of the `player.characters` table.
            ///
            /// Its SQL type is `Varchar`.
            ///
            /// (Automatically generated by Diesel.)
            #[max_length = 32]
            name -> Varchar,
            /// The `class` column of the `player.characters` table.
            ///
            /// Its SQL type is `Varchar`.
            ///
            /// (Automatically generated by Diesel.)
            #[max_length = 32]
            class -> Varchar,
            /// The `theme` column of the `player.characters` table.
            ///
            /// Its SQL type is `Varchar`.
            ///
            /// (Automatically generated by Diesel.)
            #[max_length = 32]
            theme -> Varchar,
            /// The `level` column of the `player.characters` table.
            ///
            /// Its SQL type is `Int4`.
            ///
            /// (Automatically generated by Diesel.)
            level -> Int4,
            /// The `experience` column of the `player.characters` table.
            ///
            /// Its SQL type is `Int8`.
            ///
            /// (Automatically generated by Diesel.)
            experience -> Int8,
            /// The `hit_points` column of the `player.characters` table.
            ///
            /// Its SQL type is `Int4`.
            ///
            /// (Automatically generated by Diesel.)
            hit_points -> Int4,
            /// The `stamina` column of the `player.characters` table.
            ///
            /// Its SQL type is `Int4`.
            ///
            /// (Automatically generated by Diesel.)
            stamina -> Int4,
            /// The `abilities` column of the `player.characters` table.
            ///
            /// Its SQL type is `Jsonb`.
            ///
            /// (Automatically generated by Diesel.)
            abilities -> Jsonb,
            /// The `feats` column of the `player.characters` table.
            ///
            /// Its SQL type is `Jsonb`.
            ///
            /// (Automatically generated by Diesel.)
            feats -> Jsonb,
            /// The `skills` column of the `player.characters` table.
            ///
            /// Its SQL type is `Jsonb`.
            ///
            /// (Automatically generated by Diesel.)
            skills -> Jsonb,
        }
    }

    diesel::joinable!(character_currency_ledger -> characters (character_id));
    diesel::joinable!(character_inventory -> characters (character_id));

    diesel::allow_tables_to_appear_in_same_query!(
        character_currency_ledger,
        character_inventory,
        characters,
    );
}
