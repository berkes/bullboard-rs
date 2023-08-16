use crate::events::{DividendPaid, Event, PriceObtained, StocksBought};
use crate::value_objects::{Amount, Currency, StockIdentifier};
use chrono::NaiveDateTime;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Dashboard {
    events: Vec<Event>,
    pub number_of_positions: f64,
    pub total_dividend: Amount,
    pub total_buying_price: Amount,
    total_value_at: HashMap<NaiveDateTime, Amount>,
    tickers: HashMap<StockIdentifier, f64>,
    pub currency: Currency,
}

impl Dashboard {
    pub fn new(events: Vec<Event>) -> Self {
        let currency = Currency("USD".to_string());
        let mut dashboard = Dashboard {
            events,
            number_of_positions: 0.0,
            total_dividend: Amount::zero(currency.clone()),
            total_buying_price: Amount::zero(currency.clone()),
            total_value_at: HashMap::new(),
            tickers: HashMap::new(),
            currency,
        };

        for event in &dashboard.events.clone() {
            dashboard.handle_event(event);
        }

        dashboard
    }

    pub fn total_value(&self, currency: Currency) -> Amount {
        self.total_value_at
            .values()
            .last()
            .unwrap_or(&Amount::zero(currency))
            .to_owned()
    }

    pub fn total_value_at(&self, date: &NaiveDateTime, currency: Currency) -> Amount {
        self.total_value_at
            .get(date)
            .unwrap_or(&Amount::zero(currency))
            .to_owned()
    }

    pub fn amount_of(&self, identifier: &StockIdentifier) -> f64 {
        *self.tickers.get(identifier).unwrap_or(&0.0)
    }

    fn handle_event(&mut self, generic_event: &Event) {
        match generic_event {
            Event::StocksBought(event) => self.handle_stocks_bought(event.clone()),
            Event::PriceObtained(event) => self.handle_price_obtained(event.clone()),
            Event::DividendPaid(event) => self.handle_dividend_paid(event.clone()),
        };
    }

    fn handle_stocks_bought(&mut self, event: StocksBought) {
        if self.currency.is_empty() {
            self.currency = event.currency().clone();
        }

        self.total_buying_price += event.price * event.amount;

        self.upsert_tickers(event.identifier.clone(), event.amount);
    }

    fn handle_price_obtained(&mut self, event: PriceObtained) {
        if self.currency.is_empty() {
            self.currency = event.currency().clone();
        }

        // Add the value of the stock at the time of the price obtained event
        let total_stock_value = (event.price.clone() * self.amount_of(&event.identifier))
            + self.total_value_at(&event.obtained_at, event.currency().clone());

        self.total_value_at
            .insert(event.obtained_at, total_stock_value);
    }

    fn handle_dividend_paid(&mut self, event: DividendPaid) {
        if self.currency.is_empty() {
            self.currency = event.currency().clone();
        }

        self.total_dividend += event.price * self.amount_of(&event.identifier);
    }

    fn upsert_tickers(&mut self, identifier: StockIdentifier, amount: f64) {
        if self.tickers.get(&identifier).is_none() {
            self.number_of_positions += 1.0;
        }

        let current_amount = self.tickers.get(&identifier).unwrap_or(&0.0);
        self.tickers.insert(identifier, current_amount + amount);
    }
}

// TODO: Move into a view layer.
impl std::fmt::Display for Dashboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let meta = vec![
            ("Amount of positions", self.number_of_positions.to_string()),
            ("Total Buying Price", self.total_buying_price.to_string()),
            (
                "Total Value",
                self.total_value(self.currency.clone()).to_string(),
            ),
            ("Total dividend", self.total_dividend.to_string()),
        ];
        write!(f, "Dashboard\n{}", format_aligned_key_value_pairs(meta))
    }
}

fn format_aligned_key_value_pairs(key_value_pairs: Vec<(&str, String)>) -> String {
    let max_key_length = key_value_pairs
        .iter()
        .map(|(key, _)| key.len())
        .max()
        .unwrap_or(0);

    let formatted_lines: Vec<String> = key_value_pairs
        .into_iter()
        .map(|(key, value)| format!("{:<width$} {}", format!("{}:", key), value, width = max_key_length + 1)) // +1 for the colon
        .collect();

    formatted_lines.join("\n")
}
