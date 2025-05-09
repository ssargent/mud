-- This file should undo anything in `up.sql`
-- Revert the changes made in `up.sql`
alter table system.permissions alter column updated_at set null; 