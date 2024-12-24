-- Your SQL goes here

-- ensure devgalaxy world exists
insert into game.worlds 
(id, name, description, created_at, updated_at)
values
(1, 'devgalaxy', 'The development galaxy', now(), now())
on conflict (id) do update set name = excluded.name, description = excluded.description, updated_at = now();

alter table game.items add code varchar(32) null;
alter table game.items add base_price bigint not null default(0);
alter table game.items add world_id bigint not null default(1);

update game.items set code = lower(name);

alter table game.items alter column code set not null;
alter table game.items add constraint fk_items_world_id foreign key (world_id) references game.worlds (id);

create unique index uq_items_code on game.items (code);
create index idx_items_world on game.items (world_id);

drop table game.armors;
drop table game.weapons;