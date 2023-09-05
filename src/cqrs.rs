use crate::event_store::EventStore;

pub struct CqrsFramework<E>
where
    E: EventStore,
{
    pub store: E,
    // queries, services?
}

impl<E> CqrsFramework<E>
where
    E: EventStore,
{
    pub fn new(store: E) -> Self {
        Self { store }
    }
}
