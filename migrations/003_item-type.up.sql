alter table game.items add item_type varchar(32) not null;

create index ix_items_type on game.items (item_type);