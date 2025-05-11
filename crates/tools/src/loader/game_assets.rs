use std::{collections::HashMap, fs, path::Path};

use serde_json::Value;
use walkdir::WalkDir;

use crate::game::{
    game_object::GameObject, CapabilitySpec, CharacterClassSpec, EnemySpec, ItemSpec, Spec,
    WorldSpec,
};

pub async fn load_assets(
    data_path: &str,
    server_address: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let hash_map = match load_all_game_objects(data_path) {
        Ok(o) => o,
        Err(e) => {
            println!("Error loading game objects: {}", e);
            HashMap::new()
        }
    };

    // convert our hash map to a vector of game objects
    let mut game_objects: Vec<GameObject> = Vec::new();
    for (_, objects) in hash_map {
        for object in objects {
            game_objects.push(object);
        }
    }

    let client = reqwest::Client::new();
    let assets = GameAssets::from_object_array(game_objects.clone());

    if assets.world.is_none() {
        return Err("No world found in assets".into());
    }

    let world_code = assets.world.clone().unwrap().code.unwrap();
    if assets.world.is_some() {
        let url = format!("{}/game/{}", server_address, world_code);
        match client
            .put(url)
            .body(serde_json::to_string(&assets.world).unwrap())
            .header("Content-Type", "application/json")
            .send()
            .await
        {
            Ok(response) => {
                if response.status().as_u16() != 304 {
                    println!(
                        "CREATED World: {} - {}",
                        world_code,
                        assets.world.clone().unwrap().description
                    );
                }
            }
            Err(e) => return Err(format!("Error: {}", e).into()),
        }
    }

    if !assets.capabilities.is_empty() {
        for capability in assets.capabilities {
            let capability_code = capability.clone().code.unwrap();
            let capability_type = capability.clone().capability_type;
            let url = format!(
                "{}/game/{}/capabilities/{}/{}",
                server_address, world_code, capability_type, capability_code
            );
            match client
                .put(url)
                .body(serde_json::to_string(&capability).unwrap())
                .header("Content-Type", "application/json")
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().as_u16() != 304 {
                        println!(
                            "CREATED Capability: {} - {}",
                            capability_code,
                            capability.clone().description
                        );
                    }
                }
                Err(e) => return Err(format!("Error: {}", e).into()),
            }
        }
    }

    if !assets.character_classes.is_empty() {
        for character_class in assets.character_classes {
            let character_class_code = character_class.clone().code.unwrap();
            let url = format!(
                "{}/game/{}/classes/{}",
                server_address, world_code, character_class_code
            );
            match client
                .put(url)
                .body(serde_json::to_string(&character_class).unwrap())
                .header("Content-Type", "application/json")
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().as_u16() != 304 {
                        println!(
                            "CREATED Character Class: {} - {}",
                            character_class_code,
                            character_class.clone().description
                        );
                    }
                }
                Err(e) => return Err(format!("Error: {}", e).into()),
            }
        }
    }

    if !assets.items.is_empty() {
        for item in assets.items {
            let item_code = item.clone().code.unwrap();
            let url = format!("{}/game/{}/items/{}", server_address, world_code, item_code);
            match client
                .put(url)
                .body(serde_json::to_string(&item).unwrap())
                .header("Content-Type", "application/json")
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().as_u16() != 304 {
                        println!("CREATED Item: {} - {}", item_code, item.clone().description);
                    }
                }
                Err(e) => return Err(format!("Error: {}", e).into()),
            }
        }
    }

    if !assets.enemies.is_empty() {
        for enemy in assets.enemies {
            let enemy_code = enemy.clone().code.unwrap();
            let url = format!(
                "{}/game/{}/enemies/{}",
                server_address, world_code, enemy_code
            );
            match client
                .put(url)
                .body(serde_json::to_string(&enemy).unwrap())
                .header("Content-Type", "application/json")
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().as_u16() != 304 {
                        println!(
                            "CREATED Enemy: {} - {}",
                            enemy_code,
                            enemy.clone().description
                        );
                    }
                }
                Err(e) => return Err(format!("Error: {}", e).into()),
            }
        }
    }
    Ok(())
}

fn load_all_game_objects(
    path: &str,
) -> Result<HashMap<String, Vec<GameObject>>, Box<dyn std::error::Error>> {
    println!("Loading resources from {}", path);
    let mut game_objects: HashMap<String, Vec<GameObject>> = HashMap::new();

    // Recursively traverse the directory structure
    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        if entry.path().is_file()
            && entry.path().extension().and_then(|ext| ext.to_str()) == Some("json")
        {
            // Process JSON file
            process_json_file(entry.path(), &mut game_objects)?;
        }
    }

    Ok(game_objects)
}

fn process_json_file(
    file_path: &Path,
    game_objects: &mut HashMap<String, Vec<GameObject>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(file_path)?;
    let v: Value = serde_json::from_str(&file_content)?;
    if v.is_array() {
        for item in v.as_array().unwrap() {
            match debug_parse_single_value(item) {
                Ok(o) => {
                    game_objects.entry(o.kind.clone()).or_default().push(o);
                }
                Err(e) => {
                    return Err(format!("Error parsing file {:?}: {}", file_path, e).into());
                }
            }
        }
    } else {
        match debug_parse_single_value(&v) {
            Ok(o) => {
                game_objects.entry(o.kind.clone()).or_default().push(o);
            }
            Err(e) => {
                return Err(format!("Error parsing file {:?}: {}", file_path, e).into());
            }
        }
    }

    Ok(())
}

fn debug_parse_single_value(item_value: &Value) -> Result<GameObject, Box<dyn std::error::Error>> {
    let spec = serde_json::from_value::<Spec>(item_value.clone())?;
    Ok(GameObject {
        kind: item_value["kind"].as_str().unwrap().to_string(),
        api_version: item_value["apiVersion"].as_str().unwrap().to_string(),
        spec,
    })
}

#[derive(Debug, Clone)]
struct GameAssets {
    world: Option<WorldSpec>,
    items: Vec<ItemSpec>,
    enemies: Vec<EnemySpec>,
    character_classes: Vec<CharacterClassSpec>,
    capabilities: Vec<CapabilitySpec>,
}

impl GameAssets {
    fn from_object_array(data: Vec<GameObject>) -> GameAssets {
        let mut m_world: Option<WorldSpec> = None;
        let mut m_items: Vec<ItemSpec> = Vec::new();
        let mut m_enemies = Vec::new();
        let mut m_character_classes = Vec::new();
        let mut m_capabilities = Vec::new();

        for object in data {
            match object.clone().spec {
                Spec::World(w) => {
                    m_world = Some(w);
                }
                Spec::Item(i) => {
                    m_items.push(i);
                }
                Spec::Enemy(e) => {
                    m_enemies.push(e);
                }
                Spec::CharacterClass(c) => {
                    m_character_classes.push(c);
                }
                Spec::Capability(c) => {
                    m_capabilities.push(c);
                }
            }
        }

        GameAssets {
            world: m_world,
            items: m_items,
            enemies: m_enemies,
            character_classes: m_character_classes,
            capabilities: m_capabilities,
        }
    }
}
