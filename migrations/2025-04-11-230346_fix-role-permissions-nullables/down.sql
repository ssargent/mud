-- This file should undo anything in `up.sql`
drop table system.role_permissions;
drop table system.permissions;
drop table system.user_api_keys;

-- Your SQL goes here
create table system.permissions (
    id bigserial not null,
    code varchar(64) not null,
    name varchar(255) not null,
    description text,
    created_at timestamp default now(),
    created_by varchar(64) not null,
    updated_at timestamp default now(),
    updated_by varchar(64) not null,
    constraint pk_permissions_id primary key (id)
);

insert into system.permissions (code, name, description, created_by, updated_by) values
('user.read', 'Read User', 'Permission to read user details', 'system', 'system'),
('user.manage', 'Manage User Access', 'Permission to perform all actions on users', 'system', 'system'),
('apikeys.read', 'Read API Key', 'Permission to read API key details', 'system', 'system'),
('apikeys.manage', 'Manage API Key Access', 'Permission to perform all actions on API keys', 'system', 'system'),
('login.external.apikey', 'Login with API Key', 'Permission to login using an API key', 'system', 'system'),
('game.world.read', 'Read Game World', 'Permission to read game world details', 'system', 'system'),
('game.world.manage', 'Manage Game World Access', 'Permission to perform all actions on game worlds', 'system', 'system'),
('game.characterclass.read', 'Read Character Class', 'Permission to read character class details', 'system', 'system'),
('game.characterclass.manage', 'Manage Character Class Access', 'Permission to perform all actions on character classes', 'system', 'system'),
('game.item.read', 'Read Item', 'Permission to read item details', 'system', 'system'),
('game.item.manage', 'Manage Item Access', 'Permission to perform all actions on items', 'system', 'system'),
('game.enemy.read', 'Read Enemy', 'Permission to read enemy details', 'system', 'system'),
('game.enemy.manage', 'Manage Enemy Access', 'Permission to perform all actions on enemies', 'system', 'system'),
('game.quest.read', 'Read Quest', 'Permission to read quest details', 'system', 'system'),
('game.quest.manage', 'Manage Quest Access', 'Permission to perform all actions on quests', 'system', 'system'),
('game.npc.read', 'Read NPC', 'Permission to read NPC details', 'system', 'system'),
('game.npc.manage', 'Manage NPC Access', 'Permission to perform all actions on NPCs', 'system', 'system'),
('game.event.read', 'Read Event', 'Permission to read event details', 'system', 'system'),
('game.event.manage', 'Manage Event Access', 'Permission to perform all actions on events', 'system', 'system'),
('game.capability.read', 'Read Capability', 'Permission to read capability details', 'system', 'system'),
('game.capability.manage', 'Manage Capability Access', 'Permission to perform all actions on capabilities', 'system', 'system');

create table system.role_permissions (
    id bigserial not null,
    role_id bigint not null,
    permission_id integer not null,
    created_at timestamp default now(),
    created_by varchar(64) not null,
    updated_at timestamp default now(),
    updated_by varchar(64) not null,
    constraint pk_rolepermissions_id primary key (id),
    constraint fk_rolepermissions_roles foreign key (role_id) references system.roles(id),
    constraint fk_rolepermissions_permissions foreign key (permission_id) references system.permissions(id)
);

create unique index idx_role_permissions_role_id_permission_id on system.role_permissions (role_id, permission_id);
create index idx_role_permissions_permission_id on system.role_permissions (permission_id); 

DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM system.roles WHERE id = 1 AND name = 'admin') THEN
        INSERT INTO system.role_permissions (role_id, permission_id, created_by, updated_by)
        SELECT 1, p.id, 'system', 'system'
        FROM system.permissions p
        ON CONFLICT DO NOTHING;
    END IF;
END $$;

create table system.user_api_keys (
    id bigserial not null,
    user_id bigint not null,
    key_type varchar(64) not null,
    api_key varchar(255) not null,
    private_key varchar(255) null,
    expiration timestamp null,
    created_at timestamp default now(),
    created_by varchar(64) not null,
    updated_at timestamp default now(),
    updated_by varchar(64) not null,
    constraint pk_user_api_keys_id primary key (id),
    constraint fk_user_api_keys_user_id foreign key (user_id) references system.users(id)
);

create index idx_user_api_keys_user_id on system.user_api_keys (user_id);