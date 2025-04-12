use serde::{Deserialize, Serialize};

use super::{CapabilitySpec, CharacterClassSpec, EnemySpec, ItemSpec, WorldSpec};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase", content = "spec")]
pub enum Spec {
    Item(ItemSpec),
    Enemy(EnemySpec),
    World(WorldSpec),
    CharacterClass(CharacterClassSpec),
    Capability(CapabilitySpec),
}
