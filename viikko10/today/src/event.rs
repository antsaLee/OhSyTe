use chrono::{Datelike, NaiveDate};

use crate::category::Category;
use crate::month_day::MonthDay;

#[derive(Debug, Copy, Clone)]
pub enum EventKind {
    Singular(NaiveDate),
}

#[derive(Debug, Clone)]
pub struct Event {
    pub kind: EventKind,
    pub description: String,
    pub category: Category,
}

impl Event {
    pub fn new_singular(date: NaiveDate, description: impl Into<String>, category: Category) -> Self {
        Self {
            kind: EventKind::Singular(date),
            description: description.into(),
            category,
        }
    }

    pub fn date(&self) -> NaiveDate {
        match self.kind {
            EventKind::Singular(d) => d,
        }
    }

    pub fn month_day(&self) -> MonthDay {
        let date = self.date();
        MonthDay::new(date.month(), date.day())
    }

    pub fn category(&self) -> Category {
        self.category
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}