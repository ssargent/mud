use std::collections::HashMap;

mod game;
use clap::{arg, command, Command};
use game::game_object::GameObject;
use game::Spec;
use walkdir::WalkDir;

use serde_json::{self, Value};
use std::fs;
use std::path::Path;

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("load-resources")
                .about("Load resources from yaml files")
                .arg(arg!(--data <VALUE>).default_value("~/source/github/ssargent/mud/data")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("load-resources", sub_m)) => {
            let data = sub_m.get_one::<String>("data").unwrap();
            let hash_map = match load_all_game_objects(data) {
                Ok(o) => o,
                Err(e) => {
                    println!("Error loading game objects: {}", e);
                    HashMap::new()
                }
            };
            let game_objects = hash_map;
            for (_, objects) in game_objects {
                for object in objects {
                    match object.spec {
                        Spec::Item(item) => {
                            println!("Item: {} Description: {}", item.name, item.description);
                        }
                        Spec::Enemy(enemy) => {
                            println!("Enemy: {} Level: {}", enemy.name, enemy.level);
                        }
                        Spec::World(world) => {
                            println!(
                                "World: {} ({})",
                                world.name,
                                world
                                    .code
                                    .unwrap_or(world.name.to_lowercase().replace(" ", "-"))
                            );
                        }
                        Spec::CharacterClass(character_class) => {
                            println!(
                                "Character Class: {} HP: {} Description: {}",
                                character_class.name,
                                character_class.hit_points,
                                character_class.description
                            );
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
