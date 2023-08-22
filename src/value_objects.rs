use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul},
};

use rust_decimal::{prelude::Zero, Decimal};

/// A financial asset (stock, ETF, etc.) held by the user
#[derive(Debug, Clone, PartialEq)]
pub struct Asset {
    /// The identifier of the asset
    pub identifier: StockIdentifier,

    /// The amount of the asset held
    pub amount: f64,

    /// The total value of the asset based on last price obtained
    /// TODO: This should be a hashmap of dates and amounts but that's a large refactor.
    /// None means that the price has not been obtained yet.
    pub value: Option<Amount>,
}

impl Asset {
    pub fn zero(identifier: &StockIdentifier) -> Self {
        Self {
            identifier: identifier.clone(),
            amount: 0.0,
            value: None,
        }
    }
}

/// A number of units of certain commodity
#[derive(Default, Debug, Clone, PartialEq, Ord, PartialOrd, Eq)]
pub struct Amount {
    /// The value of the amount
    pub num: Decimal,

    /// The commodity of the amount
    pub currency: Currency,
}

impl Amount {
    pub fn zero(currency: Currency) -> Self {
        Self {
            num: Decimal::zero(),
            currency,
        }
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.currency.is_empty() {
            write!(f, "{:.2}", self.num)
        } else {
            write!(f, "{:.2} {}", self.num, self.currency)
        }
    }
}

impl From<String> for Amount {
    fn from(s: String) -> Self {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            panic!("Amount must be in the form of '123.45 EUR' - number[whitespace]commodity");
        }

        let num = Decimal::from_str_exact(parts[0]).expect("Amount is not a valid Decimal number");
        let currency = Currency(parts[1].to_string());

        Self { num, currency }
    }
}

impl<T> Mul<T> for Amount
where
    Decimal: TryFrom<T>,
    <Decimal as TryFrom<T>>::Error: std::fmt::Debug,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            num: self.num * Decimal::try_from(rhs).unwrap(),
            currency: self.currency,
        }
    }
}

impl Add for Amount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.currency != rhs.currency {
            panic!("Cannot add amounts of different currencies");
        }

        Self {
            num: self.num + rhs.num,
            currency: self.currency,
        }
    }
}

impl<T> Add<T> for Amount
where
    Decimal: TryFrom<T>,
    <Decimal as TryFrom<T>>::Error: std::fmt::Debug,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            num: self.num + Decimal::try_from(rhs).unwrap(),
            currency: self.currency,
        }
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, rhs: Self) {
        if self.currency != rhs.currency {
            panic!(
                "Cannot add amounts of different currencies. Got {} and {}",
                self.currency, rhs.currency
            );
        }

        self.num += rhs.num;
    }
}

/// A currency string
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Currency(pub String);
impl Currency {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// A Stock Identifier
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct StockIdentifier {
    /// The ticker of the stock
    pub ticker: String,
}
impl From<String> for StockIdentifier {
    fn from(s: String) -> Self {
        Self { ticker: s }
    }
}
impl Display for StockIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.ticker)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_amount_display() {
        let amount = Amount::from("123.45 EUR".to_string());
        assert_eq!(amount.to_string(), "123.45 EUR");
    }

    #[test]
    fn test_amount_display_no_currency() {
        let amount = Amount {
            currency: Currency("".to_string()),
            ..Default::default()
        };
        assert_eq!(amount.to_string(), "0.00");
    }

    #[test]
    fn test_amount_zero() {
        let amount = Amount::zero(Currency("USD".to_string()));
        assert_eq!(amount.num, Decimal::from_str_exact("0.00").unwrap());
        assert_eq!(amount.currency.0, "USD".to_string());
    }

    #[test]
    fn test_amount_from_string() {
        let good_strings = HashMap::from([
            ("123.45 EUR", (Decimal::from_str_exact("123.45"), "EUR")),
            ("200 USD", (Decimal::from_str_exact("200"), "USD")),
            ("0.0045 BTC", (Decimal::from_str_exact("0.0045"), "BTC")),
        ]);

        for (s, (num, currency)) in good_strings {
            let amount = Amount::from(s.to_string());
            assert_eq!(amount.num, num.unwrap());
            assert_eq!(amount.currency.0, currency.to_string());
        }
    }

    #[test]
    fn test_amount_mul() {
        let amount = Amount::from("123.45 EUR".to_string());
        let amount = amount * 2.0;
        assert_eq!(amount.num, Decimal::from_str_exact("246.90").unwrap());
        assert_eq!(amount.currency.0, "EUR".to_string());
    }

    #[test]
    fn test_amount_add() {
        let amount = Amount::from("123.45 EUR".to_string());
        let amount = amount + 123.45;
        assert_eq!(amount.num, Decimal::from_str_exact("246.90").unwrap());
        assert_eq!(amount.currency.0, "EUR".to_string());
    }

    #[test]
    fn test_amount_add_assign() {
        let mut amount = Amount::from("123.45 EUR".to_string());
        amount += Amount::from("123.45 EUR".to_string());
        assert_eq!(amount.num, Decimal::from_str_exact("246.90").unwrap());
        assert_eq!(amount.currency.0, "EUR".to_string());
    }

    #[test]
    #[should_panic]
    fn test_amount_add_assign_different_currencies() {
        let mut amount = Amount::from("123.45 EUR".to_string());
        amount += Amount::from("123.45 USD".to_string());
    }

    #[test]
    fn test_stock_identifier_display() {
        let stock = StockIdentifier::from("AAPL".to_string());
        assert_eq!(stock.to_string(), "AAPL");
    }
}
