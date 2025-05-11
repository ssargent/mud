use serde::{Deserialize, Serialize};

use super::TypeSignature;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
}

impl TypeSignature for World {
    fn signature(&self) -> Vec<u8> {
        let mut signature = Vec::new();
        signature.extend_from_slice(self.name.as_bytes());
        signature.extend_from_slice(self.description.as_bytes());

        Self::as_hashed(signature)
    }
}
