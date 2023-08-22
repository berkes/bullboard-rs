use crate::events::{DividendPaid, Event, PriceObtained, StocksBought};
use crate::value_objects::{Amount, Asset, Currency, StockIdentifier};
use chrono::NaiveDateTime;
use prettytable::{format::FormatBuilder, row, Table};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Dashboard {
    events: Vec<Event>,
    pub number_of_positions: f64,
    pub total_dividend: Amount,
    pub total_buying_price: Amount,
    total_value_at: HashMap<NaiveDateTime, Amount>,
    assets: HashMap<StockIdentifier, Asset>,
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
            assets: HashMap::new(),
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
        self.assets
            .get(identifier)
            .unwrap_or(&Asset::zero(identifier))
            .amount
    }

    pub fn assets(&self) -> Vec<Asset> {
        self.assets.values().cloned().collect()
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

        let asset = Asset {
            identifier: event.identifier.clone(),
            amount: event.amount,
            value: None,
        };

        self.total_buying_price += event.price * event.amount;

        self.upsert_assets(asset, event.amount);
    }

    fn handle_price_obtained(&mut self, event: PriceObtained) {
        if self.currency.is_empty() {
            self.currency = event.currency().clone();
        }

        if let Some(asset) = self.assets.get_mut(&event.identifier) {
            asset.value = Some(event.price.clone() * asset.amount)
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

    fn upsert_assets(&mut self, asset: Asset, amount: f64) {
        let identifier = asset.identifier.clone();
        if self.assets.get(&identifier).is_none() {
            self.number_of_positions += 1.0;
        }

        let new_asset = if let Some(current_asset) = self.assets.get(&identifier) {
            Asset {
                identifier: identifier.clone(),
                amount: current_asset.amount + amount,
                value: asset.value,
            }
        } else {
            asset
        };
        self.assets.insert(identifier, new_asset);
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

        let mut assets: Vec<Asset> = self.assets();
        assets.sort_by(|a, b| a.value.cmp(&b.value));
        assets.reverse();

        write!(
            f,
            "\nDashboard\n{}\n\n{}",
            format_aligned_key_value_pairs(meta),
            format_portfolio_table(assets)
        )
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
        .map(|(key, value)| {
            format!(
                "{:<width$} {}",
                format!("{}:", key),
                value,
                width = max_key_length + 1
            )
        }) // +1 for the colon
        .collect();

    formatted_lines.join("\n")
}

fn format_portfolio_table(assets: Vec<Asset>) -> String {
    let mut table = Table::new();
    let clean_more_padding = FormatBuilder::new()
        .column_separator(' ')
        .padding(2, 1)
        .build();

    table.set_format(clean_more_padding);
    table.set_titles(row![c->"Ticker", c->"Amount", c->"Value"]);

    for asset in assets {
        table.add_row(row![
            d->asset.identifier,
            r->asset.amount,
            r->asset
                .value
                .map(|v| v.to_string())
                .unwrap_or("??.?? ???".to_string())
        ]);
    }

    table.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that_stocks_bought_adds_asset() {
        let mut dashboard = Dashboard::new(vec![]);
        let event = Event::new_stocks_bought(10.0, "13.37 USD".to_string(), "AAPL".to_string());
        if let Event::StocksBought(event) = event {
            dashboard.handle_stocks_bought(event);
        }

        assert_eq!(dashboard.assets.len(), 1);
    }
}
