pub trait Aggregate {
    type Event;
    type Command;
    type Error;

    fn handle(&mut self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error>;
    fn apply(&mut self, event: Self::Event);
}
