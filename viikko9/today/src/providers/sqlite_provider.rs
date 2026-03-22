use std::collections::HashMap;
use std::path::PathBuf;

use chrono::NaiveDate;
use sqlite::{Connection, State};

use crate::category::Category;
use crate::event::Event;
use crate::filters::EventFilter;
use crate::providers::EventProvider;

pub struct SqliteProvider {
    name: String,
    path: PathBuf,
}

impl SqliteProvider {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }

    fn parse_category(primary_name: &str) -> Option<Category> {
        match primary_name.to_lowercase().as_str() {
            "technology" => Some(Category::Technology),
            "people" => Some(Category::People),
            "history" => Some(Category::History),
            "entertainment" => Some(Category::Entertainment),
            "music" => Some(Category::Music),
            "politics" => Some(Category::Politics),
            "sports" => Some(Category::Sports),
            _ => None,
        }
    }

    fn get_categories(&self, connection: &Connection) -> HashMap<i64, Category> {
        let mut category_map: HashMap<i64, Category> = HashMap::new();

        let query = "SELECT category_id, primary_name FROM category";
        let mut statement = connection.prepare(query).unwrap();

        while let Ok(State::Row) = statement.next() {
            let category_id = statement.read::<i64, _>("category_id").unwrap();
            let primary_name = statement.read::<String, _>("primary_name").unwrap();

            if let Some(category) = Self::parse_category(&primary_name) {
                category_map.insert(category_id, category);
            }
        }

        category_map
    }

    fn make_date_part(filter: &EventFilter) -> Option<String> {
        filter.month_day().map(|month_day| {
            let md = format!("{:02}-{:02}", month_day.month(), month_day.day());
            format!("strftime('%m-%d', event_date) = '{}'", md)
        })
    }

    fn make_category_part(
        filter: &EventFilter,
        category_map: &HashMap<i64, Category>,
    ) -> Option<String> {
        filter.category().map(|filter_category| {
            for (category_id, category) in category_map {
                if *category == filter_category {
                    return format!("category_id = {}", category_id);
                }
            }
            "1 = 0".to_string()
        })
    }

    fn make_text_part(filter: &EventFilter) -> Option<String> {
        filter.text().map(|text| {
            let escaped = text.replace('\'', "''");
            format!("event_description LIKE '%{}%'", escaped)
        })
    }

    fn make_where_clause(filter: &EventFilter, category_map: &HashMap<i64, Category>) -> String {
        let mut parts: Vec<String> = Vec::new();

        if let Some(date_part) = Self::make_date_part(filter) {
            parts.push(date_part);
        }
        if let Some(category_part) = Self::make_category_part(filter, category_map) {
            parts.push(category_part);
        }
        if let Some(text_part) = Self::make_text_part(filter) {
            parts.push(text_part);
        }

        if parts.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", parts.join(" AND "))
        }
    }
}

impl EventProvider for SqliteProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let connection = Connection::open(&self.path).expect("Failed to open SQLite database");
        let categories = self.get_categories(&connection);

        let where_clause = Self::make_where_clause(filter, &categories);
        let mut query = "SELECT event_date, event_description, category_id FROM event".to_string();

        if !where_clause.is_empty() {
            query.push(' ');
            query.push_str(&where_clause);
        }

        let mut statement = connection.prepare(query).unwrap();

        while let Ok(State::Row) = statement.next() {
            let event_date = statement.read::<String, _>("event_date").unwrap();
            let event_description = statement.read::<String, _>("event_description").unwrap();
            let category_id = statement.read::<i64, _>("category_id").unwrap();

            let date =
                NaiveDate::parse_from_str(&event_date, "%Y-%m-%d").expect("Invalid date in database");

            if let Some(category) = categories.get(&category_id) {
                let event = Event::new_singular(date, &event_description, *category);
                events.push(event);
            }
        }
    }
}