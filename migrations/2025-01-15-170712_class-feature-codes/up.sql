-- Your SQL goes here
alter table game.character_class_features add column code varchar(32) not null;
create unique index unq_character_class_features_code on game.character_class_features (code);
