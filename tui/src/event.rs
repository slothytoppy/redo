use crossterm::event::Event;

pub trait EventHandler {
    type Event;
    type Input;

    fn handle_event(&mut self, event: &Event, input: &Self::Input) -> Option<Self::Event>;
}
