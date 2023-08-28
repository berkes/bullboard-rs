use std::fmt::Display;

use prettytable::{format::FormatBuilder, row};

use crate::events::Event;

#[derive(PartialEq, Debug)]
enum JournalEntry {
    Buy,
    Dividend,
    // TODO: Split, Merge, Top-up, Withdraw, Tax, Interest, Fee, claim-event, etc.
}

pub struct Journal {
    entries: Vec<JournalEntry>,
}
impl Journal {
    pub fn new(events: Vec<Event>) -> Self {
        let entries = events
            .iter()
            .map(|event| match event {
                Event::StocksBought { .. } => Some(JournalEntry::Buy),
                Event::DividendPaid(_) => Some(JournalEntry::Dividend),
                Event::PriceObtained { .. } => None,
            })
            .filter_map(|entry| entry)
            .collect::<Vec<JournalEntry>>()
            .into();

        Self { entries }
    }
}

// TODO: Move to a view layer
impl Display for Journal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = prettytable::Table::new();
        let clean_more_padding = FormatBuilder::new()
            .column_separator(' ')
            .padding(2, 1)
            .build();

        table.set_format(clean_more_padding);
        table.set_titles(
            row![c->"Date", c->"Type", c->"Ticker", c->"Amount", c->"Price", c->"Total"],
        );
        self.entries.iter().for_each(|entry| {
            match entry {
                JournalEntry::Buy => { table.add_row(row![l->"2020-01-01", l->"Buy", l->"AAPL", r->"10.0", r->"100.00 USD", r->"1000.00 USD"]); },
                _ => {}
            }
        });
        write!(f, "My Journal\n{}", table)
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
