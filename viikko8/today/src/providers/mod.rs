use crate::event::Event;

pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, events: &mut Vec<Event>);
}

pub mod historical_provider;
pub mod sqlite_provider;