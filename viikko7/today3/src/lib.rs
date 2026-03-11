use serde::Deserialize;
use std::error::Error;
use std::path::{Path, PathBuf};

pub mod category;
pub mod event;
pub mod providers;

use chrono::{Datelike, Local};
use providers::historical_provider::HistoricalProvider;
use providers::EventProvider;

#[derive(Debug, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub kind: String,
    pub resource: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub providers: Vec<ProviderConfig>,
}

fn resolve_resource(config_dir: &Path, resource: &str) -> PathBuf {
    if resource.contains("://") {
        PathBuf::from(resource)
    } else {
        config_dir.join(resource)
    }
}

fn create_providers(
    config: &Config,
    config_dir: &Path,
) -> Result<Vec<Box<dyn EventProvider>>, Box<dyn Error>> {
    let mut providers: Vec<Box<dyn EventProvider>> = Vec::new();

    for provider in &config.providers {
        match provider.kind.as_str() {
            "historical" => {
                let resource_path = resolve_resource(config_dir, &provider.resource);
                let p = HistoricalProvider::new(provider.name.clone(), resource_path);
                providers.push(Box::new(p));
            }
            _ => {
                return Err(format!("Unknown provider kind: {}", provider.kind).into());
            }
        }
    }

    Ok(providers)
}

pub fn run(config: &Config, config_dir: &Path) -> Result<(), Box<dyn Error>> {
    let providers = create_providers(config, config_dir)?;
    let mut events = vec![];

    for provider in providers {
        provider.get_events(&mut events);
    }

    let today = Local::now().date_naive();

    println!("{}.{}.", today.day(), today.month());

    let mut found = false;

    for event in &events {
        let d = event.date();
        if d.month() == today.month() && d.day() == today.day() {
            println!("{} ({}, {:?})", event.description, d.year(), event.category);
            found = true;
        }
    }

    if !found {
        println!("No events found for today.");
    }

    Ok(())
}