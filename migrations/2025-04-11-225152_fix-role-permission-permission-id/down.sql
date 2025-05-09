-- This file should undo anything in `up.sql`
ALTER TABLE system.role_permissions
ALTER COLUMN permission_id TYPE int USING permission_id::int;