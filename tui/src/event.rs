use crossterm::event::Event;

pub trait EventHandler {
    type Event;
    fn handle_event(&mut self, event: &Event) -> Option<Self::Event>;
}
