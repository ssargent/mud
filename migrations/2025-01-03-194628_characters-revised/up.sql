-- Your SQL goes here
/*

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: Option<i64>,
    pub code: Option<String>,
    pub world_id: Option<i64>,
    pub player_id: Option<i64>,
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
 
*/

drop table player.characters;
create table player.characters (
    id bigserial not null,
    world_id bigint not null,
    user_id uuid not null,
    race_id bigint not null,
    name varchar(32) not null,
    class varchar(32) not null,
    theme varchar(32) not null,
    level int not null,
    experience bigint not null,
    hit_points int not null,
    stamina int not null,
    abilities jsonb not null,
    feats jsonb not null,
    skills jsonb not null,
    constraint pk_characters_id primary key (id),
    constraint fk_characters_worlds foreign key (world_id) references game.worlds (id),
    constraint fk_characters_users foreign key (user_id) references system.users (id),
    constraint fk_characters_races foreign key (race_id) references game.races (id)
);

create index idx_characters_world_id on player.characters (world_id);
create index idx_characters_user_id on player.characters (user_id);
create index idx_characters_race_id on player.characters (race_id);

create unique index idx_characters_world_id_name on player.characters (world_id, name);

create table player.character_inventory (
    id bigserial not null,
    character_id bigint not null,
    item_id bigint not null,
    quantity int not null,
    constraint pk_character_inventory_id primary key (id),
    constraint fk_character_inventory_characters foreign key (character_id) references player.characters (id),
    constraint fk_character_inventory_items foreign key (item_id) references game.items (id)
);

create index idx_character_inventory_character_id on player.character_inventory (character_id);
create index idx_character_inventory_item_id on player.character_inventory (item_id);

create table game.currency (
    id bigint not null,
    world_id bigint not null,
    code varchar(32) not null,
    name varchar(32) not null,
    description text not null,
    is_spendable boolean not null default true,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    constraint pk_currency_id primary key (id),
    constraint fk_currency_worlds foreign key (world_id) references game.worlds (id)
);

create index idx_currency_world_id on game.currency (world_id);
create unique index idx_currency_world_id_code on game.currency (world_id, code);

create table player.character_currency_ledger (
    id bigserial not null,
    character_id bigint not null,
    currency_id bigint not null,
    entry_type varchar(32) not null,
    amount int not null,
    created_at timestamp not null default now(),
    memo text not null,
    constraint pk_character_currency_ledger_id primary key (id),
    constraint fk_character_currency_ledger_characters foreign key (character_id) references player.characters (id),
    constraint fk_character_currency_ledger_currency foreign key (currency_id) references game.currency (id)
);

create index idx_character_currency_ledger_character_id on player.character_currency_ledger (character_id);
create index idx_character_currency_ledger_currency_id on player.character_currency_ledger (currency_id);
