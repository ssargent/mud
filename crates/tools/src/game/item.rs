/*
{
            "world_id": 1,
            "code": "sniper-blaster-rifle",
            "item_type": "weapon",
            "category_id": 1,
            "name": "Sniper Blaster Rifle",
            "description": "A sniper longarm energy weapon",
            "item_properties": {
                "damage": "1d12",
                "damage_type": "energy",
                "properties": [
                    "blast"
                ],
                "range": 400
            },
            "base_price": 1375
        }
*/

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSpec {
    pub id: Option<i64>,
    pub code: Option<String>,
    pub world_id: Option<i64>,
    pub item_type: String,
    pub category_id: Option<i64>,
    pub name: String,
    pub description: String,
    pub item_properties: serde_json::Value,
    pub base_price: i32,
}
