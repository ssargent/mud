-- Your SQL goes here
create table system.roles (
    id bigserial not null,
    name varchar(255) not null,
    description varchar(255) not null,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    constraint pk_roles_id primary key (id)
);

create table system.user_roles (
    id bigserial not null,
    user_id bigint not null,
    role_id bigint not null,
    is_read_only boolean not null default false,
    start_date timestamp with time zone not null default now(),
    end_date timestamp with time zone null default null,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    constraint pk_user_roles_id primary key (id),
    constraint fk_user_roles_user_id foreign key (user_id) references system.users (id) on delete cascade,
    constraint fk_user_roles_role_id foreign key (role_id) references system.roles (id) on delete cascade
)