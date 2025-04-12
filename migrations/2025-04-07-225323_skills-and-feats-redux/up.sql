-- Your SQL goes here

-- 1. Drop the old check constraint (it's tied to the old column name)
ALTER TABLE game.capabilities
DROP CONSTRAINT capabilities_type_check;

-- 2. Rename the column
ALTER TABLE game.capabilities
RENAME COLUMN type TO capability_type;

-- 3. Recreate the check constraint on the new column
ALTER TABLE game.capabilities
ADD CONSTRAINT capabilities_capability_type_check
CHECK (capability_type IN ('skill', 'feat', 'ability', 'power', 'action', 'feature'));

-- 4. (Optional) Rename index if you want name consistency
DROP INDEX IF EXISTS idx_capabilities_type;

CREATE INDEX idx_capabilities_capability_type
ON game.capabilities (capability_type);
