use crate::value_objects::{Amount, Currency, StockIdentifier};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// A stock was bought
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StocksBought {
    /// Event creation time
    pub created_at: NaiveDateTime,
    /// The amount of stocks of this type. Fractional, because some assets allow fractions
    pub amount: f64,
    /// The price paid for each stock
    pub price: Amount,
    /// The ticker of the stock
    pub identifier: StockIdentifier,
}

impl StocksBought {
    pub fn new(created_at: NaiveDateTime, amount: f64, price: String, ticker: String) -> Self {
        let price = Amount::from(price);
        let identifier = StockIdentifier::from(ticker);
        Self {
            created_at,
            amount,
            price,
            identifier,
        }
    }

    pub(crate) fn currency(&self) -> Currency {
        self.price.currency.clone()
    }
}

/// A price was obtained for a stock
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceObtained {
    /// The time the price was obtained
    pub created_at: NaiveDateTime,
    /// The price paid for each stock
    pub price: Amount,
    /// The ticker of the stock
    pub identifier: StockIdentifier,
}

impl PriceObtained {
    pub fn new(created_at: NaiveDateTime, price: String, identifier: String) -> Self {
        let price = Amount::from(price);
        let identifier = StockIdentifier::from(identifier);
        Self {
            created_at,
            price,
            identifier,
        }
    }

    pub(crate) fn currency(&self) -> Currency {
        self.price.currency.clone()
    }
}

/// A dividend was paid for a stock
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DividendPaid {
    /// The time the dividend was paid
    pub created_at: NaiveDateTime,
    /// The amount of dividend paid per stock on hand
    pub price: Amount,
    /// The ticker of the stock
    pub identifier: StockIdentifier,
}

impl DividendPaid {
    pub fn new(created_at: NaiveDateTime, price: String, identifier: String) -> Self {
        let price = Amount::from(price);
        let identifier = StockIdentifier::from(identifier);
        Self { created_at, price, identifier }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    StocksBought(StocksBought),
    PriceObtained(PriceObtained),
    DividendPaid(DividendPaid),
}

impl Event {
    pub fn new_stocks_bought(
        created_at: NaiveDateTime,
        amount: f64,
        price: String,
        ticker: String,
    ) -> Self {
        let stocks_bought = StocksBought::new(created_at, amount, price, ticker);
        Event::StocksBought(stocks_bought)
    }

    pub fn new_price_obtained(
        created_at: NaiveDateTime,
        price: String,
        identifier: String,
    ) -> Self {
        let price_obtained = PriceObtained::new(created_at, price, identifier);
        Event::PriceObtained(price_obtained)
    }

    pub fn new_dividend_paid(created_at: NaiveDateTime, price: String, identifier: String) -> Self {
        let dividend_paid = DividendPaid::new(created_at, price, identifier);
        Event::DividendPaid(dividend_paid)
    }

    pub(crate) fn created_at(&self) -> NaiveDateTime {
        match self {
            Event::StocksBought(event) => event.created_at,
            Event::PriceObtained(event) => event.created_at,
            Event::DividendPaid(event) => event.created_at,
        }
    }
}
