use serde::{Deserialize, Serialize};

use super::TypeSignature;

/*
    Leaving the gameplay things as serde_json::Value for now, as we don't know what they will look like yet.
    Once these structures are defined, we can change them to the appropriate types.
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub id: Option<i64>,
    pub world_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub capability_type: String,
    pub code: String,
    pub name: String,
    pub description: String,
    pub requirements: serde_json::Value,
    pub actions: Option<serde_json::Value>,
    pub access_requirements: serde_json::Value,
    pub gameplay_definition: Option<serde_json::Value>,
    pub tags: Vec<Option<String>>,
}

impl TypeSignature for Capability {
    fn signature(&self) -> Vec<u8> {
        let mut signature = Vec::new();
        //signature.extend_from_slice(&self.world_id.unwrap_or(0).to_be_bytes());
        signature.extend_from_slice(self.code.as_bytes());
        signature.extend_from_slice(self.name.as_bytes());
        signature.extend_from_slice(self.description.as_bytes());
        signature.extend_from_slice(self.capability_type.as_bytes());
        signature.extend_from_slice(&self.parent_id.unwrap_or(0).to_be_bytes());
        signature.extend_from_slice(self.requirements.to_string().as_bytes());
        signature.extend_from_slice(self.access_requirements.to_string().as_bytes());

        if let Some(actions) = &self.actions {
            signature.extend_from_slice(actions.to_string().as_bytes());
        }
        if let Some(gameplay_definition) = &self.gameplay_definition {
            signature.extend_from_slice(gameplay_definition.to_string().as_bytes());
        }
        self.tags.iter().for_each(|tag| {
            if let Some(tag) = tag {
                signature.extend_from_slice(tag.as_bytes());
            }
        });

        Self::as_hashed(signature)
    }
}
