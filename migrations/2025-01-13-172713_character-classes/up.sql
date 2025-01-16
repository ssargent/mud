-- Your SQL goes here


create table game.character_classes (
    id bigserial not null,
    world_id bigint not null,
    code varchar(32) not null,
    name varchar(32) not null,
    description text not null,
    hit_points int not null,
    stamina_expression text not null,
    skillpoint_expression text not null,
    proficiencies jsonb not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    constraint pk_character_classes_id primary key (id),
    constraint fk_character_classes_worlds foreign key (world_id) references game.worlds (id)
);

create unique index idx_character_classes_world_id_code on game.character_classes (world_id, code);
create index idx_character_classes_world_id on game.character_classes (world_id);

create table game.character_class_features (
    id bigserial not null,
    class_id bigint not null,
    level int not null,
    name varchar(32) not null,
    description text not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    constraint pk_character_class_features_id primary key (id),
    constraint fk_character_class_features_classes foreign key (class_id) references game.character_classes (id)
);

create index idx_character_class_features_class_id on game.character_class_features (class_id);