use crate::value_objects::{Amount, Currency, StockIdentifier};
use chrono::NaiveDateTime;

/// A stock was bought
#[derive(Debug, Clone)]
pub struct StocksBought {
    /// The amount of stocks of this type. Fractional, because some assets allow fractions
    pub amount: f64,
    /// The price paid for each stock
    pub price: Amount,
    /// The ticker of the stock
    pub identifier: StockIdentifier,
}

impl StocksBought {
    pub fn new(amount: f64, price: String, ticker: String) -> Self {
        let price = Amount::from(price);
        let identifier = StockIdentifier::from(ticker);
        Self {
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
#[derive(Debug, Clone)]
pub struct PriceObtained {
    /// The amount of stocks of this type. Fractional, because some assets allow fractions
    pub obtained_at: NaiveDateTime,
    /// The price paid for each stock
    pub price: Amount,
    /// The ticker of the stock
    pub identifier: StockIdentifier,
}

impl PriceObtained {
    pub fn new(obtained_at: NaiveDateTime, price: String, identifier: String) -> Self {
        let price = Amount::from(price);
        let identifier = StockIdentifier::from(identifier);
        Self {
            obtained_at,
            price,
            identifier,
        }
    }

    pub(crate) fn currency(&self) -> Currency {
        self.price.currency.clone()
    }
}

/// A dividend was paid for a stock
#[derive(Debug, Clone)]
pub struct DividendPaid {
    /// The amount of dividend paid per stock on hand
    pub price: Amount,
    /// The ticker of the stock
    pub identifier: StockIdentifier,
}

impl DividendPaid {
    pub fn new(price: String, identifier: String) -> Self {
        let price = Amount::from(price);
        let identifier = StockIdentifier::from(identifier);
        Self { price, identifier }
    }

    pub(crate) fn currency(&self) -> Currency {
        self.price.currency.clone()
    }
}

// Your existing code above...

// Now, the refactored enum Event:
#[derive(Debug, Clone)]
pub enum Event {
    StocksBought(StocksBought),
    PriceObtained(PriceObtained),
    DividendPaid(DividendPaid),
}

impl Event {
    pub fn new_stocks_bought(amount: f64, price: String, ticker: String) -> Self {
        let stocks_bought = StocksBought::new(amount, price, ticker);
        Event::StocksBought(stocks_bought)
    }

    pub fn new_price_obtained(
        obtained_at: NaiveDateTime,
        price: String,
        identifier: String,
    ) -> Self {
        let price_obtained = PriceObtained::new(obtained_at, price, identifier);
        Event::PriceObtained(price_obtained)
    }

    pub fn new_dividend_paid(price: String, identifier: String) -> Self {
        let dividend_paid = DividendPaid::new(price, identifier);
        Event::DividendPaid(dividend_paid)
    }
}
