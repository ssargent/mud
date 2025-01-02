use clap::{arg, command, ArgMatches, Command};
use protocol::types::item::Item;
use serde::{Deserialize, Serialize};
use serde_json;
use walkdir::WalkDir;

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
            load_resources(data);
        }
        _ => println!("No subcommand was used"),
    }
}

fn load_game_objects(data: String) -> Vec<GameObject<Item>> {
    let items = serde_json::from_str(&data).unwrap();
    items
}

// load_resources recursively loads all yaml files from a given path and converts them to items.
fn load_resources(path: &String) {
    println!("Loading resources from {}", path);
    let items = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().unwrap_or_default() == "json")
        .map(|e| std::fs::read_to_string(e.path()).unwrap())
        .flat_map(|e| load_game_objects(e))
        .collect::<Vec<GameObject<Item>>>();

    println!("Loaded {} items", items.len());
    for item in items {
        println!("{}\t{}", item.spec.name, item.spec.description);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameObject<T> {
    apiVersion: String,
    kind: String,
    spec: T,
}
