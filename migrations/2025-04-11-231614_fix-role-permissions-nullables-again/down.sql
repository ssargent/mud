-- This file should undo anything in `up.sql`
-- Your SQL goes here
alter table system.permissions alter column created_at set null;
alter table system.permissions alter column description set null;

