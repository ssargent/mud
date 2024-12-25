
.PHONY: export-schema
export-schema:
	@pg_dump --schema-only -d mud-game-development > schema.sql

.PHONY: generate
generate: export-schema
	@sqlc generate