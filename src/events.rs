use chrono::NaiveDateTime;

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


