-- Your SQL goes here
alter table system.settings drop constraint pk_settings_id;
alter table system.settings drop column id;
alter table system.settings add column id bigserial not null;
alter table system.settings add constraint pk_settings_id primary key (id);

alter table game.world_node_features drop constraint pk_world_node_features_id;
alter table game.world_node_features drop column id;
alter table game.world_node_features add column id bigserial not null;
alter table game.world_node_features add constraint pk_world_node_features_id primary key (id);

alter table game.npc_spawn_rules drop constraint pk_npc_spawn_rules_id;
alter table game.npc_spawn_rules drop column id;
alter table game.npc_spawn_rules add column id bigserial not null;
alter table game.npc_spawn_rules add constraint pk_npc_spawn_rules_id primary key (id);

