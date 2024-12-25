// Code generated by sqlc. DO NOT EDIT.
// versions:
//   sqlc v1.26.0

package repository

import (
	"time"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5/pgtype"
)

type GameAttribute struct {
	ID          int64     `json:"id"`
	Name        string    `json:"name"`
	Description string    `json:"description"`
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
}

type GameFeat struct {
	ID          int64     `json:"id"`
	WorldID     int64     `json:"world_id"`
	Code        string    `json:"code"`
	Name        string    `json:"name"`
	Description string    `json:"description"`
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
}

type GameItem struct {
	ID             int64     `json:"id"`
	CategoryID     int64     `json:"category_id"`
	Name           string    `json:"name"`
	Description    string    `json:"description"`
	ItemProperties []byte    `json:"item_properties"`
	CreatedAt      time.Time `json:"created_at"`
	UpdatedAt      time.Time `json:"updated_at"`
	ItemType       string    `json:"item_type"`
	Code           string    `json:"code"`
	BasePrice      int64     `json:"base_price"`
	WorldID        int64     `json:"world_id"`
}

type GameItemCategory struct {
	ID          int64       `json:"id"`
	ParentID    pgtype.Int8 `json:"parent_id"`
	Name        string      `json:"name"`
	Description string      `json:"description"`
	CreatedAt   time.Time   `json:"created_at"`
	UpdatedAt   time.Time   `json:"updated_at"`
}

type GameNpcSpawnRule struct {
	ID               uuid.UUID `json:"id"`
	NpcTemplateID    int64     `json:"npc_template_id"`
	WorldNodeID      int64     `json:"world_node_id"`
	SpawnChance      int32     `json:"spawn_chance"`
	SpawnQuantityMin int32     `json:"spawn_quantity_min"`
	SpawnQuantityMax int32     `json:"spawn_quantity_max"`
	CreatedAt        time.Time `json:"created_at"`
	UpdatedAt        time.Time `json:"updated_at"`
}

type GameNpcTemplate struct {
	ID               int64     `json:"id"`
	Name             string    `json:"name"`
	Description      string    `json:"description"`
	NpcProperties    []byte    `json:"npc_properties"`
	CanSpawnMultiple bool      `json:"can_spawn_multiple"`
	CanRespawn       bool      `json:"can_respawn"`
	CreatedAt        time.Time `json:"created_at"`
	UpdatedAt        time.Time `json:"updated_at"`
}

type GameRace struct {
	ID          int64     `json:"id"`
	WorldID     int64     `json:"world_id"`
	Code        string    `json:"code"`
	Name        string    `json:"name"`
	Description string    `json:"description"`
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
}

type GameSkill struct {
	ID          int64     `json:"id"`
	WorldID     int64     `json:"world_id"`
	Code        string    `json:"code"`
	Name        string    `json:"name"`
	Description string    `json:"description"`
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
}

type GameWorld struct {
	Name        string    `json:"name"`
	Description string    `json:"description"`
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
	ID          int64     `json:"id"`
}

type GameWorldNode struct {
	ID          int64       `json:"id"`
	ParentID    pgtype.Int8 `json:"parent_id"`
	Name        string      `json:"name"`
	Description string      `json:"description"`
	CreatedAt   time.Time   `json:"created_at"`
	UpdatedAt   time.Time   `json:"updated_at"`
	WorldID     int64       `json:"world_id"`
}

type GameWorldNodeFeature struct {
	ID                uuid.UUID `json:"id"`
	WorldNodeID       int64     `json:"world_node_id"`
	FeatureName       string    `json:"feature_name"`
	FeatureValue      string    `json:"feature_value"`
	FeatureProperties []byte    `json:"feature_properties"`
	CreatedAt         time.Time `json:"created_at"`
	UpdatedAt         time.Time `json:"updated_at"`
}

type PlayerCharacter struct {
	ID                  uuid.UUID `json:"id"`
	UserID              uuid.UUID `json:"user_id"`
	CharacterName       string    `json:"character_name"`
	Class               string    `json:"class"`
	CharacterLevel      int32     `json:"character_level"`
	CharacterDefinition []byte    `json:"character_definition"`
	CreatedAt           time.Time `json:"created_at"`
	UpdatedAt           time.Time `json:"updated_at"`
}

type SchemaMigration struct {
	Version int64 `json:"version"`
	Dirty   bool  `json:"dirty"`
}

type SystemSetting struct {
	ID        uuid.UUID `json:"id"`
	Name      string    `json:"name"`
	DataType  string    `json:"data_type"`
	Value     string    `json:"value"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
}

type SystemUser struct {
	ID        uuid.UUID `json:"id"`
	Username  string    `json:"username"`
	Password  string    `json:"password"`
	Email     string    `json:"email"`
	FullName  string    `json:"full_name"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
}
