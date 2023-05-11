pub mod event;
pub mod dao {
    pub mod event_entity;
}
pub type Event = event::Event;
pub type EventsRepository = event::EventsRepository;