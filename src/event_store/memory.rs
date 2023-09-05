use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::events::AccountEvent;

use super::{EventStore, EventStoreError};

#[derive(Default)]
pub struct MemoryEventStore {
    events: Arc<Mutex<HashMap<String, Vec<AccountEvent>>>>,
}

impl EventStore for MemoryEventStore {
    fn get_events(&self, aggregate_id: &str) -> Result<Vec<AccountEvent>, EventStoreError> {
        let events_map = self.events.lock().unwrap();
        if let Some(events) = events_map.get(aggregate_id) {
            Ok(events.clone())
        } else {
            Err(EventStoreError::AggregateNotFound(aggregate_id.to_string()))
        }
    }

    fn persist(&self, aggregate_id: &str, events: &[AccountEvent]) -> Result<(), EventStoreError> {
        let mut events_map = self.events.lock().unwrap();
        let aggregate_events = events_map.entry(aggregate_id.to_string()).or_default();
        aggregate_events.extend_from_slice(events);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{date_utils::fixtures::iphone_launched_at, events::AccountEvent};

    use super::*;

    #[test]
    fn test_memory_persist() {
        let event_store = MemoryEventStore::default();
        let events = vec![AccountEvent::new_stocks_bought(
            iphone_launched_at(),
            10.0,
            "100.00 USD".to_string(),
            "AAPL".to_string(),
        )];
        event_store.persist("123", &events).unwrap();
        let events = event_store.get_events("123").unwrap();
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_memory_get_events() {
        let event_store = MemoryEventStore::default();
        let events = vec![AccountEvent::new_stocks_bought(
            iphone_launched_at(),
            10.0,
            "100.00 USD".to_string(),
            "AAPL".to_string(),
        )];
        event_store.persist("123", &events).unwrap();
        let events = event_store.get_events("123").unwrap();
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_memory_get_events_not_found() {
        let event_store = MemoryEventStore::default();
        let events = event_store.get_events("123");
        assert!(events.is_err());
    }
}
