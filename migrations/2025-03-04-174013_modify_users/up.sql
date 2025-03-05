-- Your SQL goes here
alter table player.characters drop constraint fk_characters_users;
drop index player.idx_characters_user_id;
alter table player.characters drop column user_id;

alter table player.characters add column user_id bigint not null;

create index idx_characters_user_id on player.characters (user_id);

alter table system.users drop constraint pk_users_id;
alter table system.users drop column id;

alter table system.users add column id bigserial not null;
alter table system.users add constraint pk_users_id primary key (id);


