use crossterm::event::Event;

pub trait EventHandler<Input, Output> {
    fn handle_event(&mut self, _event: &Event, _input: Input) -> Option<Output> {
        None
    }
}
