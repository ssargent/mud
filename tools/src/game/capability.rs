use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitySpec {
    pub id: Option<i64>,
    pub world_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub capability_type: String,
    pub code: Option<String>,
    pub name: String,
    pub description: String,
    pub requirements: serde_json::Value,
    pub actions: Option<serde_json::Value>,
    pub access_requirements: serde_json::Value,
    pub gameplay_definition: Option<serde_json::Value>,
    pub tags: Vec<Option<String>>,
}
