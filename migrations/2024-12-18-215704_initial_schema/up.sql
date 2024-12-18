-- Your SQL goes here
create extension if not exists "uuid-ossp";

create schema system
    create table users (
        id uuid not null default uuid_generate_v4(),
        username varchar(32) not null,
        password varchar(128) not null,
        email varchar(128) not null,
        full_name varchar(128) not null,
        created_at timestamp not null default now(),
        updated_at timestamp not null default now(),
        constraint pk_users_id primary key (id)
    )
    create table settings (
        id uuid not null default uuid_generate_v4(),
        name varchar(32) not null,
        data_type varchar(32) not null,
        value text not null,
        created_at timestamp not null default now(),
        updated_at timestamp not null default now(),
        constraint pk_settings_id primary key (id)
    );
create schema game
   create table attributes (
        id bigserial not null,
        name varchar(32) not null,
        description text not null,
        created_at timestamp not null default now(),
        updated_at timestamp not null default now(),
        constraint pk_attributes_id primary key (id)
   )
   create table worlds (
        id uuid not null default uuid_generate_v4(),
        name varchar(32) not null,
        description text not null,
        created_at timestamp not null default now(),
        updated_at timestamp not null default now(),
        constraint pk_worlds_id primary key (id)    
   )
   create table world_nodes (
        id bigserial not null,
        world_id uuid not null,
        parent_id bigint null,
        name varchar(32) not null,
        description text not null,
        created_at timestamp not null default now(),
        updated_at timestamp not null default now(),
		constraint pk_world_nodes_id primary key (id)
   )
   create table world_node_features (
        id uuid not null default uuid_generate_v4(),
        world_node_id bigint not null,
        feature_name varchar(32) not null,
        feature_value text not null,
        feature_properties jsonb not null,
        created_at timestamp not null default now(),
        updated_at timestamp not null default now(),
        constraint pk_world_node_features_id primary key (id),
        constraint fk_world_node_features_world_node_id foreign key (world_node_id) references game.world_nodes (id)
   )
   create index idx_world_node_features_world_node_id on game.world_node_features (world_node_id)
   create table npc_templates (
        id bigserial not null,
        name varchar(32) not null,
        description text not null,
        npc_properties jsonb not null,
        can_spawn_multiple boolean not null,
        can_respawn boolean not null,
        created_at timestamp not null default now(),
        updated_at timestamp not null default now(),
        constraint pk_npc_templates_id primary key (id)
   )
   create table npc_spawn_rules (
        id uuid not null default uuid_generate_v4(),
        npc_template_id bigint not null,
        world_node_id bigint not null,
        spawn_chance int not null,
        spawn_quantity_min int not null default (1),
        spawn_quantity_max int not null default (1),
        created_at timestamp not null default now(),
        updated_at timestamp not null default now(),
        constraint pk_npc_spawn_rules_id primary key (id),
        constraint fk_npc_spawn_rules_npc_template_id foreign key (npc_template_id) references game.npc_templates (id),
        constraint fk_npc_spawn_rules_world_node_id foreign key (world_node_id) references game.world_nodes (id)
   )
    create index idx_npc_spawn_rules_npc_template_id on game.npc_spawn_rules (npc_template_id)
    create index idx_npc_spawn_rules_world_node_id on game.npc_spawn_rules (world_node_id)
    create table item_categories (
        id bigserial not null,
        parent_id bigint null,
        name varchar(32) not null,
        description text not null,
        created_at timestamp not null default now(),
        updated_at timestamp not null default now(),
        constraint pk_item_categories_id primary key (id),
        constraint fk_item_categories_parent_id foreign key (parent_id) references game.item_categories (id)
    )
    create table items (
        id bigserial not null,
        category_id bigint not null,
        name varchar(32) not null,
        description text not null,
        item_properties jsonb not null,
        created_at timestamp not null default now(),
        updated_at timestamp not null default now(),
        constraint pk_items_id primary key (id),
        constraint fk_items_category_id foreign key (category_id) references game.item_categories (id)
    )
    create index idx_items_category_id on game.items (category_id);

create schema player
    create table characters (
        id uuid not null default uuid_generate_v4(),
        user_id uuid not null,
        character_name varchar(32) not null,
        class varchar(32) not null,
        character_level int not null,
        character_definition jsonb not null,
        created_at timestamp not null default now(),
        updated_at timestamp not null default now(),
        constraint pk_characters_id primary key (id),
        constraint fk_characters_user_id foreign key (user_id) references system.users (id)
    )
    create index idx_characters_user_id on player.characters (user_id);
   