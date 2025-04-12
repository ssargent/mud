-- Your SQL goes here
alter table system.permissions alter column created_at set not null;
alter table system.permissions alter column description set not null;

