use std::collections::HashMap;

mod game;
use clap::{arg, command, Command};
use game::game_object::GameObject;
use game::{world, ItemSpec, Spec, WorldSpec};
use walkdir::WalkDir;

use serde_json::{self, Value};
use std::fs;
use std::path::Path;
use tokio;

#[tokio::main]
async fn main() {
    let matches =
        command!()
            .subcommand(
                Command::new("load-resources")
                    .about("Load resources from yaml files")
                    .arg(arg!(--data <VALUE>).default_value(
                        "/Users/scott/source/github/ssargent/mud/data/worlds/devgalaxy",
                    ))
                    .arg(arg!(--server <VALUE>).default_value("http://localhost:2900")),
            )
            .get_matches();

    match matches.subcommand() {
        Some(("load-resources", sub_m)) => {
            let data = sub_m.get_one::<String>("data").unwrap();
            let server = sub_m.get_one::<String>("server").unwrap();
            let hash_map = match load_all_game_objects(data) {
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
                println!("No world found in assets");
                return;
            }

            let world_code = assets.world.clone().unwrap().code.unwrap();
            if assets.world.is_some() {
                let url = format!("{}/game/{}", server, world_code);
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
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }

            if !assets.items.is_empty() {
                for item in assets.items {
                    let item_code = item.clone().code.unwrap();
                    let url = format!("{}/game/{}/items/{}", server, world_code, item_code);
                    match client
                        .put(url)
                        .body(serde_json::to_string(&item).unwrap())
                        .header("Content-Type", "application/json")
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if response.status().as_u16() != 304 {
                                println!(
                                    "CREATED Item: {} - {}",
                                    item_code,
                                    item.clone().description
                                );
                            }
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                }
            }
        }
        _ => println!("No subcommand was used"),
    }
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
    enemies: Vec<GameObject>,
    character_classes: Vec<GameObject>,
}

impl GameAssets {
    fn from_object_array(data: Vec<GameObject>) -> GameAssets {
        let mut m_world: Option<WorldSpec> = None;
        let mut m_items: Vec<ItemSpec> = Vec::new();
        let mut m_enemies = Vec::new();
        let mut m_character_classes = Vec::new();

        for object in data {
            match object.clone().spec {
                Spec::World(w) => {
                    m_world = Some(w);
                }
                Spec::Item(i) => {
                    m_items.push(i);
                }
                Spec::Enemy(e) => {
                    m_enemies.push(object);
                }
                Spec::CharacterClass(c) => {
                    m_character_classes.push(object);
                }
            }
        }

        GameAssets {
            world: m_world,
            items: m_items,
            enemies: m_enemies,
            character_classes: m_character_classes,
        }
    }
}
