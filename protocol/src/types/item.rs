use serde::{Deserialize, Serialize};

use super::TypeSignature;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: Option<i64>,
    pub world_id: i64,
    pub category_id: i64,
    pub code: String,
    pub item_type: String,
    pub name: String,
    pub description: String,
    pub item_properties: serde_json::Value,
    pub base_price: i64,
    pub created_at: Option<i64>,
}

impl TypeSignature for Item {
    fn signature(&self) -> Vec<u8> {
        let mut signature = Vec::new();
        signature.extend_from_slice(&self.world_id.to_be_bytes());
        signature.extend_from_slice(&self.category_id.to_be_bytes());
        signature.extend_from_slice(self.code.as_bytes());
        signature.extend_from_slice(self.item_type.as_bytes());
        signature.extend_from_slice(self.name.as_bytes());
        signature.extend_from_slice(self.description.as_bytes());
        signature.extend_from_slice(&self.base_price.to_be_bytes());

        Self::as_hashed(signature)
    }
}
