-- Create new capabilities table
create table game.capabilities (
    id bigserial not null,
    world_id bigint not null,
    parent_id bigint, 
    type text not null,
    code text not null,
    name text not null,
    description text not null,
    requirements jsonb not null default '{}'::jsonb,
    actions jsonb,
    access_requirements jsonb not null default '{}'::jsonb,
    tags text[] not null default '{}',
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    constraint pk_capabilities_id primary key (id),
    constraint fk_capabilities_worlds 
        foreign key (world_id) 
        references game.worlds (id)
        on delete cascade,
    constraint fk_capabilities_capabilities 
        foreign key (parent_id) 
        references game.capabilities (id)
        on delete set null,
    constraint capabilities_type_check check (type in ('skill', 'feat'))
);

create index idx_capabilities_world on game.capabilities (world_id);
create index idx_capabilities_parents on game.capabilities (parent_id);
create index idx_capabilities_tags_gin on game.capabilities using GIN (tags);
create unique index uidx_capabilities_code on game.capabilities (code);
create index idx_capabilities_type on game.capabilities (type);

-- drop skills
drop table game.skills;
drop table game.feats;