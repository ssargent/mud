-- Your SQL goes here
alter table game.worlds drop constraint pk_worlds_id;
 
alter table game.world_nodes drop world_id;
alter table game.worlds drop id;

alter table game.worlds add id bigserial not null;
alter table game.world_nodes add world_id bigint not null;

alter table game.worlds add constraint pk_worlds_id primary key (id);
alter table game.world_nodes add constraint fk_world_nodes_worlds_id foreign key (world_id) references game.worlds (id);

create index idx_world_nodes_world_id on game.world_nodes (world_id);

create table game.skills (
    id bigserial not null,
    world_id bigint not null,
    code varchar(32) not null,
    name varchar(32) not null,
    description text not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    constraint pk_skills_id primary key (id),
    constraint fk_skills_world_id foreign key (world_id) references game.worlds (id)
);

create unique index uq_skills_code on game.skills (world_id, code);
create index idx_skills_world on game.skills (world_id);

create table game.feats (
    id bigserial not null,
    world_id bigint not null,
    code varchar(32) not null,
    name varchar(32) not null,
    description text not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    constraint pk_feats_id primary key (id),
    constraint fk_feats_world_id foreign key (world_id) references game.worlds (id)
);

create unique index uq_feats_code on game.feats (world_id, code);
create index idx_feats_world on game.feats (world_id);

create table game.races (
    id bigserial not null,
    world_id bigint not null,
    code varchar(32) not null,
    name varchar(32) not null,
    description text not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    constraint pk_races_id primary key (id),
    constraint fk_races_world_id foreign key (world_id) references game.worlds (id)
);

create unique index uq_races_code on game.races (world_id, code);
create index idx_races_world on game.races (world_id);

create table game.weapons (
    id bigserial not null,
    world_id bigint not null,
    code varchar(32) not null,
    name varchar(32) not null,
    description text not null,
    level integer not null,
    base_price integer not null,
    damage varchar(32) not null,
    weapon_properties jsonb not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    constraint pk_weapons_id primary key (id),
    constraint fk_weapons_world_id foreign key (world_id) references game.worlds (id)
);

create unique index uq_weapons_code on game.weapons (world_id, code);
create index idx_weapons_world on game.weapons (world_id);

create table game.armors (
    id bigserial not null,
    world_id bigint not null,
    code varchar(32) not null,
    name varchar(32) not null,
    description text not null,
    level integer not null,
    base_price integer not null,
    armor_class integer not null,
    armor_properties jsonb not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    constraint pk_armors_id primary key (id),
    constraint fk_armors_world_id foreign key (world_id) references game.worlds (id)
);

create unique index uq_armors_code on game.armors (world_id, code);
create index idx_armors_world on game.armors (world_id);


