-- Your SQL goes here
create table game.enemies (
    id bigserial not null,
    world_id bigint not null,
    code varchar(32) not null,
    name varchar(32) not null,
    description text not null,
    class varchar(32) not null,
    level int not null,
    hit_points int not null,
    stamina int not null,
    strength int not null,
    dexterity int not null,
    constitution int not null,
    intelligence int not null,
    wisdom int not null,
    weapons jsonb not null,
    armor jsonb not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    constraint pk_enemies_id primary key (id),
    constraint fk_enemies_worlds foreign key (world_id) references game.worlds (id)
);

create index idx_enemies_world_id on game.enemies (world_id);
create unique index idx_enemies_world_id_code on game.enemies (world_id, code);
