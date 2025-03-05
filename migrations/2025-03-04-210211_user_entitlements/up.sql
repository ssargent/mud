-- Your SQL goes here
create table player.entitlements (
    id bigserial not null,
    name varchar(255) not null,
    code varchar(64) not null,
    description varchar(255) not null,
    world_id bigint not null,
    entitlement_type varchar(255) not null,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    constraint pk_entitlements_id primary key (id),
    constraint fk_entitlements_world_id foreign key (world_id) references game.worlds (id) on delete cascade
);

create index idx_entitlements_world_id on player.entitlements (world_id);

create table player.entitlement_mappings (
    id bigserial not null,
    entitlement_id bigint not null,
    user_id bigint not null,
    is_consumable boolean not null default false,
    is_consumed boolean not null default false,
    start_date timestamp with time zone not null default now(),
    end_date timestamp with time zone null default now(),
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    constraint pk_entitlement_mappings_id primary key (id),
    constraint fk_entitlement_mappings_entitlement_id foreign key (entitlement_id) references player.entitlements (id) on delete cascade,
    constraint fk_entitlement_mappings_user_id foreign key (user_id) references system.users (id) on delete cascade
);

create index idx_entitlement_mappings_entitlement_id on player.entitlement_mappings (entitlement_id);
create index idx_entitlement_mappings_user_access on player.entitlement_mappings (user_id, start_date, end_date);