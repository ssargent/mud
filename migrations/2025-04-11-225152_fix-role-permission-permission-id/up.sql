-- Your SQL goes here
-- change system.role_permissions.permission_id from integer to bigint

ALTER TABLE system.role_permissions
ALTER COLUMN permission_id TYPE bigint USING permission_id::bigint;
