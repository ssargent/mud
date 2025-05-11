use serde::{Deserialize, Serialize};

use super::Spec;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameObject {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub spec: Spec,
}
