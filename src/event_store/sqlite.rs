use crate::{event_store::EventStoreError, events::AccountEvent};
use rusqlite::{params, Connection};
use std::rc::Rc;

use super::EventStore;

pub struct SqliteEventStore {
    db: Rc<Connection>,
}

impl SqliteEventStore {
    pub fn new(path: &str) -> Result<Self, EventStoreError> {
        let db = Connection::open(path)?;
        Ok(Self { db: Rc::new(db) })
    }

    pub fn init(&self) -> Result<(), EventStoreError> {
        self.db
            .execute(
                "CREATE TABLE IF NOT EXISTS events (
                    id INTEGER PRIMARY KEY,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    aggregate_id TEXT NOT NULL,
                    event TEXT NOT NULL
                )",
                params![],
            )
            .map_err(|_| EventStoreError::Unknown)?;
        Ok(())
    }
}

impl EventStore for SqliteEventStore {
    fn get_events(&self, aggregate_id: &str) -> Result<Vec<AccountEvent>, EventStoreError> {
        let mut stmt = self
            .db
            .prepare("SELECT event FROM events WHERE aggregate_id = ? ORDER BY created_at ASC")?;
        let events = stmt
            .query_map([&aggregate_id], |row| {
                let event: String = row.get(0)?;
                Ok(event)
            })
            .map_err(|_| EventStoreError::Unknown)?
            .collect::<Result<Vec<String>, rusqlite::Error>>()?;

        if events.is_empty() {
            Err(EventStoreError::AggregateNotFound(aggregate_id.to_string()))
        } else {
            Ok(events
                .iter()
                .map(|event| serde_json::from_str(event).unwrap())
                .collect())
        }
    }

    fn persist(&self, aggregate_id: &str, events: &[AccountEvent]) -> Result<(), EventStoreError> {
        // TODO: we now store the created_at in both the serialized event and in the database.
        let mut stmt = self
            .db
            .prepare("INSERT INTO events (aggregate_id, created_at, event) VALUES (?, ?, ?)")?;
        for event in events {
            let dt = event.created_at();
            let event = serde_json::to_string(event)?;

            stmt.execute(params![&aggregate_id, &dt, &event])
                .expect("Failed to insert");
        }
        Ok(())
    }
}

impl From<rusqlite::Error> for EventStoreError {
    fn from(err: rusqlite::Error) -> Self {
        EventStoreError::StorageError(err.to_string())
    }
}

impl From<serde_json::Error> for EventStoreError {
    fn from(err: serde_json::Error) -> Self {
        EventStoreError::StorageError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use crate::{date_utils::fixtures::iphone_launched_at, events::StocksBought};

    use super::*;
    #[test]
    fn test_sqlite_persist() {
        let (db_file, event_store) = setup_db();
        let events = vec![AccountEvent::new_stocks_bought(
            iphone_launched_at(),
            10.0,
            "100.00 USD".to_string(),
            "AAPL".to_string(),
        )];
        event_store.persist("123", &events).unwrap();
        let events = event_store.get_events("123").unwrap();

        assert_eq!(events.len(), 1);

        db_file.close().unwrap();
    }

    #[test]
    fn test_sqlite_get_events() {
        let (db_file, event_store) = setup_db();
        let events = vec![AccountEvent::new_stocks_bought(
            iphone_launched_at(),
            10.0,
            "100.00 USD".to_string(),
            "AAPL".to_string(),
        )];
        event_store.persist("123", &events).unwrap();
        let events = event_store.get_events("123").unwrap();

        assert_eq!(events.len(), 1);

        db_file.close().unwrap();
    }

    #[test]
    fn test_sqlite_get_events_sorts_by_created_at() {
        // Insert MSFT event first, then AAPL event
        // But post-date the AAPL event by 1 second, so it should be first
        let (db_file, event_store) = setup_db();
        let events = vec![
            AccountEvent::new_stocks_bought(
                iphone_launched_at() + chrono::Duration::seconds(1),
                10.0,
                "100.00 USD".to_string(),
                "MSFT".to_string(),
            ),
            AccountEvent::new_stocks_bought(
                iphone_launched_at(),
                10.0,
                "100.00 USD".to_string(),
                "AAPL".to_string(),
            ),
        ];
        event_store.persist("123", &events).unwrap();
        let events = event_store.get_events("123").unwrap();

        assert_eq!(events.len(), 2);
        let tickers = events
            .iter()
            .map(|e| match e {
                AccountEvent::StocksBought(StocksBought { identifier, .. }) => {
                    identifier.ticker.clone()
                }
                _ => panic!("Unexpected event type"),
            })
            .collect::<Vec<String>>();

        assert_eq!(tickers, vec!["AAPL", "MSFT"]);

        db_file.close().unwrap();
    }

    #[test]
    fn test_sqlite_get_events_not_found() {
        let (db_file, event_store) = setup_db();
        let events = event_store.get_events("123");

        assert!(events.is_err());

        db_file.close().unwrap();
    }

    fn setup_db() -> (TempDir, SqliteEventStore) {
        let temp_dir = tempfile::tempdir().expect("Failed to create tmp directory");
        let db_path = temp_dir.path().join("test.db");
        let db_path_str = db_path.to_str().expect("Failed to convert path to string");

        let event_store = SqliteEventStore::new(db_path_str).expect("Failed to create event store");
        event_store
            .init()
            .expect("Failed to initialize event store");

        (temp_dir, event_store)
    }
}
