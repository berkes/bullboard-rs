use chrono::NaiveDate;

use crate::{
    events::Event,
    value_objects::{Amount, StockIdentifier},
};

#[derive(PartialEq, Debug)]
pub enum JournalEntry {
    Buy(JournalRow),
    Dividend(JournalRow),
    // TODO: Split, Merge, Top-up, Withdraw, Tax, Interest, Fee, claim-event, etc.
}

pub struct Journal {
    pub entries: Vec<JournalEntry>,
}

#[derive(PartialEq, Debug)]
pub enum JournalRowType {
    Buy,
    Dividend,
}

#[derive(PartialEq, Debug)]
pub struct JournalRow {
    pub date: Option<NaiveDate>,
    pub rtype: JournalRowType,
    pub identifier: StockIdentifier,
    pub amount: f64,
    pub price: Amount,
    pub total: Amount,
}

impl Journal {
    pub fn new(events: Vec<Event>) -> Self {
        let entries = events
            .iter()
            .filter_map(|event| match event {
                Event::StocksBought(props) => Some(JournalEntry::Buy(JournalRow {
                    date: Some(props.created_at.date()),
                    rtype: JournalRowType::Buy,
                    identifier: props.identifier.clone(),
                    amount: props.amount,
                    price: props.price.clone(),
                    total: (props.price.clone() * props.amount),
                })),
                Event::DividendPaid(props) => Some(JournalEntry::Dividend(JournalRow {
                    date: Some(props.created_at.date()),
                    rtype: JournalRowType::Dividend,
                    identifier: props.identifier.clone(),
                    amount: 1.0, // TODO: Change dividend to have price per share instead of total
                    price: props.price.clone(),
                    total: props.price.clone(),
                })),
                Event::PriceObtained { .. } => None,
            })
            .collect::<Vec<JournalEntry>>();

        Self { entries }
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use crate::date_utils::fixtures::iphone_launched_at;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn journal_from_stocks_bought_events() {
        let events = vec![
            Event::new_stocks_bought(
                iphone_launched_at(),
                10.0,
                "100.00 USD".to_string(),
                "AAPL".to_string(),
            ),
            Event::new_stocks_bought(
                iphone_launched_at(),
                20.0,
                "200.00 USD".to_string(),
                "AAPL".to_string(),
            ),
        ];
        let journal = Journal::new(events);
        assert_eq!(
            journal.entries,
            vec![
                JournalEntry::Buy(JournalRow {
                    date: Some(iphone_launched_at().date()),
                    rtype: JournalRowType::Buy,
                    identifier: StockIdentifier::from("AAPL"),
                    amount: 10.0,
                    price: Amount::from("100.00 USD"),
                    total: Amount::from("1000.00 USD")
                }),
                JournalEntry::Buy(JournalRow {
                    date: Some(iphone_launched_at().date()),
                    rtype: JournalRowType::Buy,
                    identifier: StockIdentifier::from("AAPL"),
                    amount: 20.0,
                    price: Amount::from("200.00 USD"),
                    total: Amount::from("4000.00 USD")
                })
            ]
        );
    }

    #[test]
    fn journal_from_dividend_paid_events() {
        let events = vec![
            Event::new_dividend_paid(
                iphone_launched_at(),
                "100.00 USD".to_string(),
                "AAPL".to_string(),
            ),
            Event::new_dividend_paid(
                iphone_launched_at(),
                "200.00 USD".to_string(),
                "AAPL".to_string(),
            ),
        ];
        let journal = Journal::new(events);
        assert_eq!(
            journal.entries,
            vec![
                JournalEntry::Dividend(JournalRow {
                    date: Some(iphone_launched_at().date()),
                    rtype: JournalRowType::Dividend,
                    identifier: StockIdentifier::from("AAPL"),
                    amount: 1.0,
                    price: Amount::from("100.00 USD"),
                    total: Amount::from("100.00 USD")
                }),
                JournalEntry::Dividend(JournalRow {
                    date: Some(iphone_launched_at().date()),
                    rtype: JournalRowType::Dividend,
                    identifier: StockIdentifier::from("AAPL"),
                    amount: 1.0,
                    price: Amount::from("200.00 USD"),
                    total: Amount::from("200.00 USD")
                })
            ]
        );
    }

    #[test]
    fn journal_from_price_obtained_events() {
        let events = vec![
            Event::new_price_obtained(
                iphone_launched_at(),
                "100.00 USD".to_string(),
                "AAPL".to_string(),
            ),
            Event::new_price_obtained(
                iphone_launched_at(),
                "200.00 USD".to_string(),
                "AAPL".to_string(),
            ),
        ];
        let journal = Journal::new(events);
        assert_eq!(journal.entries, vec![]);
    }
}
