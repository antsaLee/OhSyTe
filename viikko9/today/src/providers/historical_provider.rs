use std::path::PathBuf;

use chrono::NaiveDate;

use crate::category::Category;
use crate::event::Event;
use crate::filters::EventFilter;
use crate::providers::EventProvider;

pub struct HistoricalProvider {
    name: String,
    resource: PathBuf,
}

impl HistoricalProvider {
    pub fn new(name: String, resource: PathBuf) -> Self {
        Self { name, resource }
    }
}

impl EventProvider for HistoricalProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let _ = &self.resource;

        let source_events = vec![
            Event::new_singular(
                NaiveDate::from_ymd_opt(1996, 1, 10).unwrap(),
                "JDK 1.0 released",
                Category::Technology,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(2008, 12, 3).unwrap(),
                "Python 3.0 released",
                Category::Technology,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1929, 1, 15).unwrap(),
                "Martin Luther King Jr. was born",
                Category::People,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1962, 1, 17).unwrap(),
                "Jim Carrey was born",
                Category::People,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1963, 1, 16).unwrap(),
                "James May was born",
                Category::People,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1942, 1, 17).unwrap(),
                "Muhammad Ali was born",
                Category::People,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1980, 1, 18).unwrap(),
                "Pink Floyd's double album The Wall hits #1",
                Category::Music,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(2013, 1, 19).unwrap(),
                "Lance Armstrong admits to doping in all seven Tour de France victories",
                Category::Sports,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(2008, 1, 20).unwrap(),
                "Breaking Bad premieres",
                Category::Entertainment,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1869, 1, 21).unwrap(),
                "Rasputin was born",
                Category::People,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1879, 1, 22).unwrap(),
                "Battle of Rorke's Drift",
                Category::History,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1973, 1, 23).unwrap(),
                "Vietnam War ends",
                Category::History,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1987, 1, 24).unwrap(),
                "Luis Suárez was born",
                Category::People,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1554, 1, 25).unwrap(),
                "City of São Paulo founded in Brazil",
                Category::History,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1924, 1, 25).unwrap(),
                "1st Winter Olympic Games open in Chamonix, France",
                Category::Sports,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1978, 1, 25).unwrap(),
                "Volodymyr Zelensky was born",
                Category::People,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1998, 1, 26).unwrap(),
                "President Bill Clinton says he did not have sexual relations with that woman",
                Category::Politics,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1820, 1, 27).unwrap(),
                "Russian Antarctic expedition discovers Antarctica",
                Category::History,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1756, 1, 27).unwrap(),
                "Wolfgang Amadeus Mozart was born",
                Category::People,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1986, 1, 28).unwrap(),
                "Space Shuttle Challenger explodes",
                Category::History,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1935, 1, 28).unwrap(),
                "Iceland becomes the first Western country to legalize abortion",
                Category::History,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1954, 1, 29).unwrap(),
                "Oprah Winfrey was born",
                Category::People,
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1999, 3, 11).unwrap(),
                "Test for today",
                Category::People,
            ),
        ];

        for event in source_events {
            if filter.accepts(&event) {
                events.push(event);
            }
        }
    }
}