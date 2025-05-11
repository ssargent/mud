/*
{
        "id": 1,
        "code": "devgalaxy",
        "name": "devgalaxy",
        "description": "A galaxy for development"
    }
*/

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSpec {
    pub id: Option<i64>,
    pub code: Option<String>,
    pub name: String,
    pub description: String,
}
