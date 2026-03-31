use crate::event::Event;
use crate::filters::EventFilter;

pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>);
}

pub mod historical_provider;
pub mod sqlite_provider;