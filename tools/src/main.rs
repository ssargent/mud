mod game;
mod loader;
use clap::{arg, command, Command};
use loader::load_assets;

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
            match load_assets(data, server).await {
                Ok(_) => println!("Assets loaded successfully"),
                Err(e) => println!("Error loading assets: {}", e),
            }
        }
        _ => println!("No subcommand was used"),
    }
}
