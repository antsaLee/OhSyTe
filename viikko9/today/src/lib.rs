use serde::Deserialize;
use std::error::Error;
use std::path::{Path, PathBuf};

pub mod category;
pub mod event;
pub mod filters;
pub mod month_day;
pub mod providers;

use chrono::{Datelike, Local};
use filters::FilterBuilder;
use month_day::MonthDay;
use providers::historical_provider::HistoricalProvider;
use providers::sqlite_provider::SqliteProvider;
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
            "sqlite" => {
                let resource_path = resolve_resource(config_dir, &provider.resource);
                let p = SqliteProvider::new(provider.name.clone(), resource_path);
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
    let today = Local::now().date_naive();
    let month_day = MonthDay::new(today.month(), today.day());
    let filter = FilterBuilder::new().month_day(month_day).build();
    let mut events = vec![];

    for provider in providers {
        provider.get_events(&filter, &mut events);
    }

    println!("{}.{}.", today.day(), today.month());

    if events.is_empty() {
        println!("No events found for today.");
    } else {
        for event in &events {
            println!("{} ({}, {:?})", event.description, event.date().year(), event.category);
        }
    }

    Ok(())
}