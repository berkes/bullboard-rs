use std::{error::Error, fmt::Display};

use crate::events::AccountEvent;

mod memory;
pub mod sqlite;

pub trait EventStore {
    fn get_events(&self, aggregate_id: &str) -> Result<Vec<AccountEvent>, EventStoreError>;
    fn persist(&self, aggregate_id: &str, events: &[AccountEvent]) -> Result<(), EventStoreError>;
}

#[derive(Debug)]
pub enum EventStoreError {
    AggregateNotFound(String),
    StorageError(String),
    Unknown,
}
impl Error for EventStoreError {}

impl Display for EventStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventStoreError::AggregateNotFound(id) => {
                write!(f, "Aggregate with id {} not found", id)
            }
            EventStoreError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            EventStoreError::Unknown => write!(f, "Unknown error"),
        }
    }
}
