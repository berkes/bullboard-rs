use std::collections::HashMap;

use crate::events::{DividendPaid, Event, PriceObtained, StocksBought};
use crate::value_objects::{Amount, Amounts, Asset, StockIdentifier};

#[derive(Debug)]
pub struct Dashboard {
    events: Vec<Event>,
    pub number_of_positions: f64,
    pub total_dividend: Amounts,
    pub total_buying_price: Amounts,
    pub total_value: Amounts,
    assets: HashMap<StockIdentifier, Asset>,
}

impl Dashboard {
    pub fn new(events: Vec<Event>) -> Self {
        let mut dashboard = Dashboard {
            events,
            number_of_positions: 0.0,
            total_dividend: Amounts::zero(),
            total_buying_price: Amounts::zero(),
            total_value: Amounts::zero(),
            assets: HashMap::new(),
        };

        for event in &dashboard.events.clone() {
            dashboard.handle_event(event);
        }

        dashboard
    }

    pub fn assets(&self) -> Vec<Asset> {
        self.assets.values().cloned().collect()
    }

    fn amount_of(&self, identifier: &StockIdentifier) -> f64 {
        self.assets
            .get(identifier)
            .unwrap_or(&Asset::zero(identifier))
            .amount
    }

    fn handle_event(&mut self, generic_event: &Event) {
        match generic_event {
            Event::StocksBought(event) => self.handle_stocks_bought(event.clone()),
            Event::PriceObtained(event) => self.handle_price_obtained(event.clone()),
            Event::DividendPaid(event) => self.handle_dividend_paid(event.clone()),
        };
    }

    fn handle_stocks_bought(&mut self, event: StocksBought) {
        let asset = Asset {
            identifier: event.identifier.clone(),
            amount: event.amount,
            dividends: Amount::zero(event.currency().clone()),
            value: None,
        };

        self.total_buying_price.upsert(event.price * event.amount);

        self.upsert_assets(asset, event.amount);
    }

    fn handle_price_obtained(&mut self, event: PriceObtained) {
        // Guard against the case where we have not bought any of this stock yet
        if self.assets.get(&event.identifier).is_none() {
            return;
        }

        // Update the value of the asset
        let amount_at_hand = self.amount_of(&event.identifier);
        let asset = Asset {
            identifier: event.identifier.clone(),
            amount: amount_at_hand,
            dividends: Amount::zero(event.currency().clone()),
            value: Some(event.price.clone() * amount_at_hand),
        };
        self.assets.insert(event.identifier.clone(), asset);

        // Update the total value
        self.total_value.upsert(event.price * amount_at_hand);
    }

    fn handle_dividend_paid(&mut self, event: DividendPaid) {
        if let Some(asset) = self.assets.get_mut(&event.identifier) {
            asset.dividends += event.price.clone() * asset.amount;
        }

        let dividend = event.price * self.amount_of(&event.identifier);
        let new_amount = self.total_dividend.for_currency(&dividend.currency) + dividend;
        self.total_dividend.upsert(new_amount);
    }

    fn upsert_assets(&mut self, asset: Asset, amount: f64) {
        let identifier = asset.identifier.clone();
        if self.assets.get(&identifier).is_none() {
            self.number_of_positions += 1.0;
        }

        let new_asset = if let Some(current_asset) = self.assets.get(&identifier) {
            Asset {
                identifier: identifier.clone(),
                amount: current_asset.amount + amount,
                dividends: current_asset.dividends.clone(),
                value: asset.value,
            }
        } else {
            asset
        };
        self.assets.insert(identifier, new_asset);
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime};

    use super::*;

    #[test]
    fn test_that_stocks_bought_adds_asset() {
        let dashboard = Dashboard::new(vec![Event::new_stocks_bought(
            10.0,
            "13.37 USD".to_string(),
            "AAPL".to_string(),
        )]);
        assert_eq!(dashboard.assets.len(), 1);
    }

    #[test]
    fn test_that_stocks_bought_sets_dividend_to_zero() {
        let dashboard = Dashboard::new(vec![Event::new_stocks_bought(
            10.0,
            "13.37 USD".to_string(),
            "AAPL".to_string(),
        )]);

        let id: StockIdentifier = "AAPL".into();
        assert_eq!(
            dashboard.assets.get(&id).unwrap().dividends,
            Amount::new(0.into(), "USD".to_string())
        );
    }

    #[test]
    fn test_stocks_bought_with_multiple_currencies() {
        let events = vec![
            Event::new_stocks_bought(1.0, "42.00 USD".to_string(), "AAPL".to_string()),
            Event::new_stocks_bought(1.0, "13.37 EUR".to_string(), "ASR-AS".to_string()),
        ];
        let dashboard = Dashboard::new(events);

        assert_eq!(
            dashboard.total_buying_price,
            Amounts::new(vec![
                "13.37 EUR".to_string().into(),
                "42.00 USD".to_string().into(),
            ])
        );
    }

    #[test]
    fn test_stocks_bought_has_default_totals() {
        let dashboard = Dashboard::new(vec![]);
        assert_eq!(dashboard.total_value, Amounts::zero());
    }

    #[test]
    fn test_stocks_bought_adds_totals() {
        let events = vec![
            Event::new_stocks_bought(1.0, "42.00 USD".to_string(), "AAPL".to_string()),
            Event::new_price_obtained(
                date_time(2020, 1, 1),
                "42.00 USD".to_string(),
                "AAPL".to_string(),
            ),
        ];
        let dashboard = Dashboard::new(events);

        assert_eq!(
            dashboard.total_value,
            Amounts::new(vec!["42.00 USD".to_string().into(),])
        );
    }

    #[test]
    fn test_price_obtained_with_multiple_currencies() {
        let date = NaiveDate::from_ymd_opt(2020, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let events = vec![
            Event::new_stocks_bought(1.0, "42.00 USD".to_string(), "AAPL".to_string()),
            Event::new_stocks_bought(1.0, "13.37 EUR".to_string(), "ASR-AS".to_string()),
            Event::new_price_obtained(date, "42.00 USD".to_string(), "AAPL".to_string()),
            Event::new_price_obtained(date, "13.37 EUR".to_string(), "ASR-AS".to_string()),
        ];
        let dashboard = Dashboard::new(events);

        assert_eq!(
            dashboard.total_value,
            Amounts::new(vec![
                "13.37 EUR".to_string().into(),
                "42.00 USD".to_string().into(),
            ])
        );
    }

    #[test]
    fn test_that_dividend_paid_adds_dividend_to_asset() {
        let events = vec![
            // first we need to buy some stocks before getting dividend on them
            Event::new_stocks_bought(10.0, "13.37 USD".to_string(), "AAPL".to_string()),
            Event::new_dividend_paid("13.37 USD".to_string(), "AAPL".to_string()),
        ];
        let dashboard = Dashboard::new(events);

        let id: StockIdentifier = "AAPL".into();
        assert_eq!(
            dashboard.assets.get(&id).unwrap().dividends,
            "133.70 USD".to_string().into()
        );
    }
    // TODO: test that stocks bought does not set dividend to zero if it already has a value

    fn date_time(year: i32, month: u32, day: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }
}
