use chrono::NaiveDateTime;
use std::collections::HashMap; // You'll need to add the chrono crate to your Cargo.toml

#[derive(Debug, Clone)]
pub enum Event {
    StocksBought(StocksBought),
    PriceObtained(PriceObtained),
    DividendPaid(DividendPaid),
}

#[derive(Debug, Clone)]
pub struct StocksBought {
    amount: f64,
    price: f64,
    ticker: String,
    currency: String,
}
impl StocksBought {
    pub fn new(amount: f64, price: f64, ticker: String, currency: String) -> Self {
        StocksBought {
            amount,
            price,
            ticker,
            currency,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PriceObtained {
    obtained_at: NaiveDateTime,
    price: f64,
    ticker: String,
    currency: String,
}
impl PriceObtained {
    pub fn new(price: f64, ticker: String, obtained_at: NaiveDateTime) -> Self {
        PriceObtained {
            price,
            ticker,
            obtained_at,
            currency: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DividendPaid {
    amount: f64,
    ticker: String,
    currency: String,
}
impl DividendPaid {
    pub fn new(amount: f64, ticker: String, currency: String) -> Self {
        DividendPaid {
            amount,
            ticker,
            currency,
        }
    }
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
            Event::StocksBought(event) => self.handle_stocks_bought(event),
            Event::PriceObtained(event) => self.handle_price_obtained(event),
            Event::DividendPaid(event) => self.handle_dividend_paid(event),
        }
    }

    fn handle_stocks_bought(&mut self, event: &StocksBought) {
        if self.currency.is_empty() {
            self.currency = event.currency.clone();
        }

        self.total_buying_price += event.amount * event.price;

        if !self.tickers.contains_key(&event.ticker) {
            self.number_of_positions += 1;
        }
        self.tickers.entry(event.ticker.clone()).or_insert(0.0);
        *self.tickers.get_mut(&event.ticker).unwrap() += event.amount;
    }

    fn handle_price_obtained(&mut self, event: &PriceObtained) {
        if self.currency.is_empty() {
            self.currency = event.currency.clone();
        }

        // Add the value of the stock at the time of the price obtained event
        let total_stock_value = self.total_value_at.get(&event.obtained_at).unwrap_or(&0.0)
            + self.tickers.get(&event.ticker).unwrap_or(&0.0) * event.price;

        self.total_value_at
            .insert(event.obtained_at, total_stock_value);
    }

    fn handle_dividend_paid(&mut self, event: &DividendPaid) {
        if self.currency.is_empty() {
            self.currency = event.currency.clone();
        }

        self.total_dividend += event.amount * self.tickers.get(&event.ticker).unwrap_or(&0.0);
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
