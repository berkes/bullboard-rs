use crate::events::Event;

#[derive(PartialEq, Debug)]
pub enum JournalEntry {
    Buy,
    Dividend,
    // TODO: Split, Merge, Top-up, Withdraw, Tax, Interest, Fee, claim-event, etc.
}

pub struct Journal {
    pub entries: Vec<JournalEntry>,
}

impl Journal {
    pub fn new(events: Vec<Event>) -> Self {
        let entries = events
            .iter()
            .filter_map(|event| match event {
                Event::StocksBought { .. } => Some(JournalEntry::Buy),
                Event::DividendPaid(_) => Some(JournalEntry::Dividend),
                Event::PriceObtained { .. } => None,
            })
            .collect::<Vec<JournalEntry>>();

        Self { entries }
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;

    #[test]
    fn journal_from_stocks_bought_events() {
        let events = vec![
            Event::new_stocks_bought(10.0, "100.00 USD".to_string(), "AAPL".to_string()),
            Event::new_stocks_bought(20.0, "200.00 USD".to_string(), "AAPL".to_string()),
        ];
        let journal = Journal::new(events);
        assert_eq!(journal.entries, vec![JournalEntry::Buy, JournalEntry::Buy]);
    }

    #[test]
    fn journal_from_dividend_paid_events() {
        let events = vec![
            Event::new_dividend_paid("100.00 USD".to_string(), "AAPL".to_string()),
            Event::new_dividend_paid("200.00 USD".to_string(), "AAPL".to_string()),
        ];
        let journal = Journal::new(events);
        assert_eq!(
            journal.entries,
            vec![JournalEntry::Dividend, JournalEntry::Dividend]
        );
    }

    #[test]
    fn journal_from_price_obtained_events() {
        let events = vec![
            Event::new_price_obtained(
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                "100.00 USD".to_string(),
                "AAPL".to_string(),
            ),
            Event::new_price_obtained(
                NaiveDate::from_ymd_opt(2020, 1, 2).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                "200.00 USD".to_string(),
                "AAPL".to_string(),
            ),
        ];
        let journal = Journal::new(events);
        assert_eq!(journal.entries, vec![]);
    }
}
