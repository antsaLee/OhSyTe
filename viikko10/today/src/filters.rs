use std::collections::HashSet;

use crate::category::Category;
use crate::event::Event;
use crate::month_day::MonthDay;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilterOption {
    MonthDay(MonthDay),
    Category(Category),
    Text(String),
}

pub struct EventFilter {
    options: HashSet<FilterOption>,
}

impl EventFilter {
    pub fn new() -> Self {
        Self {
            options: HashSet::new(),
        }
    }

    pub fn accepts(&self, event: &Event) -> bool {
        if self.options.is_empty() {
            return true;
        }

        let mut results: Vec<bool> = Vec::new();

        for option in &self.options {
            let result = match option {
                FilterOption::MonthDay(month_day) => *month_day == event.month_day(),
                FilterOption::Category(category) => *category == event.category(),
                FilterOption::Text(text) => event.description().contains(text),
            };
            results.push(result);
        }

        results.iter().all(|&result| result)
    }

    pub fn contains_month_day(&self) -> bool {
        self.options
            .iter()
            .any(|option| matches!(option, FilterOption::MonthDay(_)))
    }

    pub fn contains_category(&self) -> bool {
        self.options
            .iter()
            .any(|option| matches!(option, FilterOption::Category(_)))
    }

    pub fn contains_text(&self) -> bool {
        self.options
            .iter()
            .any(|option| matches!(option, FilterOption::Text(_)))
    }

    pub fn month_day(&self) -> Option<MonthDay> {
        for option in &self.options {
            if let FilterOption::MonthDay(month_day) = option {
                return Some(*month_day);
            }
        }
        None
    }

    pub fn category(&self) -> Option<Category> {
        for option in &self.options {
            if let FilterOption::Category(category) = option {
                return Some(*category);
            }
        }
        None
    }

    pub fn text(&self) -> Option<String> {
        for option in &self.options {
            if let FilterOption::Text(text) = option {
                return Some(text.clone());
            }
        }
        None
    }
}

pub struct FilterBuilder {
    options: HashSet<FilterOption>,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self {
            options: HashSet::new(),
        }
    }

    pub fn month_day(mut self, month_day: MonthDay) -> Self {
        self.options.insert(FilterOption::MonthDay(month_day));
        self
    }

    pub fn category(mut self, category: Category) -> Self {
        self.options.insert(FilterOption::Category(category));
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.options.insert(FilterOption::Text(text.into()));
        self
    }

    pub fn build(self) -> EventFilter {
        EventFilter {
            options: self.options,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn sample_event() -> Event {
        Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 19).unwrap(),
            "Rust 1.94.0 released",
            Category::Technology,
        )
    }

    #[test]
    fn filter_accepts_anything_when_no_options() {
        let event = sample_event();
        let filter = FilterBuilder::new().build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn build_filter_no_options() {
        let filter = FilterBuilder::new().build();
        let contains = [
            filter.contains_month_day(),
            filter.contains_category(),
            filter.contains_text(),
        ];
        assert_eq!(contains, [false, false, false]);
    }

    #[test]
    fn filter_accepts_matching_month_day() {
        let event = sample_event();
        let filter = FilterBuilder::new()
            .month_day(MonthDay::new(3, 19))
            .build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_rejects_wrong_month_day() {
        let event = sample_event();
        let filter = FilterBuilder::new()
            .month_day(MonthDay::new(3, 20))
            .build();
        assert!(!filter.accepts(&event));
    }

    #[test]
    fn filter_accepts_matching_category() {
        let event = sample_event();
        let filter = FilterBuilder::new().category(Category::Technology).build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_rejects_wrong_category() {
        let event = sample_event();
        let filter = FilterBuilder::new().category(Category::History).build();
        assert!(!filter.accepts(&event));
    }

    #[test]
    fn filter_accepts_matching_text() {
        let event = sample_event();
        let filter = FilterBuilder::new().text("released").build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_rejects_missing_text() {
        let event = sample_event();
        let filter = FilterBuilder::new().text("Python").build();
        assert!(!filter.accepts(&event));
    }

    #[test]
    fn filter_accepts_when_all_combined_options_match() {
        let event = sample_event();
        let filter = FilterBuilder::new()
            .month_day(MonthDay::new(3, 19))
            .category(Category::Technology)
            .text("released")
            .build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_rejects_when_one_combined_option_fails() {
        let event = sample_event();
        let filter = FilterBuilder::new()
            .month_day(MonthDay::new(3, 19))
            .category(Category::Technology)
            .text("Python")
            .build();
        assert!(!filter.accepts(&event));
    }
}