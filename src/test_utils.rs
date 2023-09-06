use bullboard::aggregate::Aggregate;

/// Test framework for testing aggregates in unit tests. Where the unit is the aggregate.
/// It tests the aggregate by applying commands to it and asserting that the resulting events are
/// emitted.
/// It can also be used to test the state of the aggregate after applying a series of events.
#[allow(dead_code)]
pub struct TestFramework<'a, T>
where
    T: Aggregate,
    T::Event: Clone + PartialEq + std::fmt::Debug,
{
    pub aggregate: &'a mut T,

    pub events: Vec<T::Event>,
}

#[allow(dead_code)]
impl<'a, T> TestFramework<'a, T>
where
    T: Aggregate,
    T::Event: Clone + PartialEq + std::fmt::Debug,
    T::Error: PartialEq + std::fmt::Debug,
{
    pub fn new(aggregate: &'a mut T) -> Self {
        Self {
            aggregate,
            events: vec![],
        }
    }

    pub fn given(&mut self, events: Vec<T::Event>) {
        events
            .iter()
            .for_each(|event| self.aggregate.apply(event.clone()));
    }

    /// This is a convenience method for when you expect the command to succeed.
    /// If you expect the command to fail, use `when_err` instead.
    /// This method panics if the command fails.
    pub fn when(&mut self, command: T::Command) -> &mut Self {
        self.events = self.aggregate.handle(command).unwrap();

        self.events
            .iter()
            .for_each(|event| self.aggregate.apply(event.clone()));

        self
    }

    /// This is a convenience method for when you expect the command to fail.
    /// If you expect the command to succeed, use `when` instead.
    /// This method panics if the command succeeds.
    pub fn when_err(&mut self, command: T::Command) -> &mut Self {
        self.aggregate.handle(command).unwrap_err();

        self
    }

    pub fn then(&mut self, events: Vec<T::Event>) {
        assert_eq!(
            self.events, events,
            "Events do not match: {:?} != {:?}",
            self.events, events
        );
    }

    pub fn then_err(&mut self, err: T::Error) {
        assert_eq!(self.events, vec![], "Events should be empty");
        assert_eq!(err, err, "Error does not match");
    }
}
