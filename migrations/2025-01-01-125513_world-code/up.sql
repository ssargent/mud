-- Your SQL goes here
alter table game.worlds add code varchar(32) null;
update game.worlds set code = lower(name);
alter table game.worlds alter column code set not null;
create unique index uq_worlds_code on game.worlds (code);
