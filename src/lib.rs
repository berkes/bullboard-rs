use chrono::NaiveDateTime;
use std::collections::HashMap; // You'll need to add the chrono crate to your Cargo.toml

#[derive(Debug, Clone)]
pub enum Event {
    StocksBought {
        amount: f64,
        price: f64,
        ticker: String,
        currency: String,
    },
    PriceObtained {
        obtained_at: NaiveDateTime,
        price: f64,
        ticker: String,
        currency: String,
    },
    DividendPaid {
        amount: f64,
        ticker: String,
        currency: String,
    },
}

#[derive(Debug)]
pub struct Dashboard {
    events: Vec<Event>,
    pub number_of_positions: usize,
    pub total_dividend: f64,
    pub total_buying_price: f64,
    total_value_at: HashMap<NaiveDateTime, f64>,
    tickers: HashMap<String, f64>,
    pub currency: String,
}

impl Dashboard {
    pub fn new(events: Vec<Event>) -> Self {
        let mut dashboard = Dashboard {
            events,
            number_of_positions: 0,
            total_dividend: 0.0,
            total_buying_price: 0.0,
            total_value_at: HashMap::new(),
            tickers: HashMap::new(),
            currency: "".to_string(),
        };

        for event in &dashboard.events.clone() {
            dashboard.handle_event(event);
        }

        dashboard
    }

    pub fn total_value(&self) -> f64 {
        *self.total_value_at.values().last().unwrap_or(&0.0)
    }

    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::StocksBought {
                ticker,
                amount,
                price,
                currency,
            } => self.handle_stocks_bought(ticker, amount, price, currency),
            Event::PriceObtained {
                ticker,
                price,
                obtained_at,
                currency,
            } => self.handle_price_obtained(ticker, price, obtained_at, currency),
            Event::DividendPaid {
                ticker,
                amount,
                currency,
            } => self.handle_dividend_paid(ticker, amount, currency),
        }
    }

    fn handle_stocks_bought(&mut self, ticker: &String, amount: &f64, price: &f64, currency: &String) {
        if self.currency.is_empty() {
            self.currency = currency.clone();
        }

        self.total_buying_price += amount * price;

        if !self.tickers.contains_key(ticker) {
            self.number_of_positions += 1;
        }
        self.tickers.entry(ticker.clone()).or_insert(0.0);
        *self.tickers.get_mut(ticker).unwrap() += amount;
    }

    fn handle_price_obtained(
        &mut self,
        ticker: &String,
        price: &f64,
        obtained_at: &NaiveDateTime,
        currency: &String,
    ) {
        if self.currency.is_empty() {
            self.currency = currency.clone();
        }

        // Add the value of the stock at the time of the price obtained event
        let total_stock_value = self.total_value_at.get(obtained_at).unwrap_or(&0.0)
            + self.tickers.get(ticker).unwrap_or(&0.0) * price;

        self.total_value_at.insert(*obtained_at, total_stock_value);
    }

    fn handle_dividend_paid(&mut self, ticker: &String, amount: &f64, currency: &String) {
        if self.currency.is_empty() {
            self.currency = currency.clone();
        }

        self.total_dividend += amount * self.tickers.get(ticker).unwrap_or(&0.0);
    }
}

impl std::fmt::Display for Dashboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Total Buying Price: {} {}\nTotal Value: {} {}\nAmount of positions: {}\nTotal dividend: {} {}",
            format!("{:.2}", self.total_buying_price),
            self.currency,
            format!("{:.2}", self.total_value()),
            self.currency,
            self.number_of_positions,
            format!("{:.2}", self.total_dividend),
            self.currency
        )
    }
}
