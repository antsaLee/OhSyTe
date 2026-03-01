mod category;
mod event;
mod providers;

use chrono::Datelike;

use crate::providers::EventProvider;
use crate::providers::historical_provider::HistoricalProvider;

fn main() {
    let provider = HistoricalProvider::new();

    let mut events = vec![];
    provider.get_events(&mut events);

    for day in 15..=29 {
        println!("\n{}.1.", day);

        for event in &events {
            let d = event.date();
            if d.month() == 1 && d.day() == day {
                println!("{} ({}, {:?})", event.description, d.year(), event.category);
            }
        }
    }
}