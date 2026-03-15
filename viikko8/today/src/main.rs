use std::fs;
use std::path::PathBuf;

use today2::{run, Config};

fn get_config_path(app_name: &str) -> Option<PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        let app_config_dir = config_dir.join(app_name);

        if !app_config_dir.exists() {
            if let Err(e) = fs::create_dir_all(&app_config_dir) {
                eprintln!("Could not create config directory: {}", e);
                return None;
            }
        }

        Some(app_config_dir)
    } else {
        None
    }
}

fn main() {
    let app_name = "today";

    let Some(config_dir) = get_config_path(app_name) else {
        eprintln!("Could not find config directory.");
        return;
    };

    let config_file = config_dir.join("today.toml");

    println!("Looking for config file: {}", config_file.display());

    if !config_file.exists() {
        eprintln!("Config file not found.");
        return;
    }

    let config_str = match fs::read_to_string(&config_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Could not read config file: {}", e);
            return;
        }
    };

    let config: Config = match toml::from_str(&config_str) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Invalid TOML file: {}", e);
            return;
        }
    };

    if let Err(e) = run(&config, &config_dir) {
        eprintln!("Error: {}", e);
    }
}