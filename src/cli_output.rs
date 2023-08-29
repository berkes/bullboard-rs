use std::fmt::Display;

use prettytable::{format::FormatBuilder, row, Table};

use crate::{
    dashboard::Dashboard,
    journal::{Journal, JournalEntry},
    value_objects::{Amounts, Asset},
};

impl Display for Dashboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut assets: Vec<Asset> = self.assets();
        assets.sort_by(|a, b| a.value.cmp(&b.value));
        assets.reverse();

        write!(
            f,
            "\nDashboard\n\n{}\n{}",
            format_meta_table(self),
            format_portfolio_table(assets)
        )
    }
}

impl Display for Journal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = prettytable::Table::new();
        let clean_more_padding = FormatBuilder::new()
            .column_separator(' ')
            .padding(2, 1)
            .build();

        table.set_format(clean_more_padding);
        table.set_titles(
            row![c->"Date", c->"Type", c->"Ticker", c->"Amount", c->"Price", c->"Total"],
        );
        self.entries.iter().for_each(|entry| {
            if entry == &JournalEntry::Buy { table.add_row(row![l->"2020-01-01", l->"Buy", l->"AAPL", r->"10.0", r->"100.00 USD", r->"1000.00 USD"]); }
        });
        write!(f, "My Journal\n{}", table)
    }
}

fn format_meta_table(dashboard: &Dashboard) -> String {
    let mut table = Table::new();
    let clean_more_padding = FormatBuilder::new()
        .column_separator(' ')
        .padding(2, 1)
        .build();
    table.set_format(clean_more_padding);

    let meta = vec![
        (
            "Number of positions",
            dashboard.number_of_positions.to_string(),
        ),
        (
            "Total buying price",
            fmt_amounts(&dashboard.total_buying_price),
        ),
        ("Total value", fmt_amounts(&dashboard.total_value)),
        ("Total dividend", fmt_amounts(&dashboard.total_dividend)),
    ];

    for (key, value) in meta {
        table.add_row(row![key, r->value]);
    }

    table.to_string()
}

fn format_portfolio_table(assets: Vec<Asset>) -> String {
    let mut table = Table::new();
    let clean_more_padding = FormatBuilder::new()
        .column_separator(' ')
        .padding(2, 1)
        .build();

    table.set_format(clean_more_padding);
    table.set_titles(row![c->"Ticker", c->"Amount", c->"Dividend", c->"Value"]);

    for asset in assets {
        table.add_row(row![
            d->asset.identifier,
            r->asset.amount,
            r->asset.dividends,
            r->asset
                .value
                .map(|v| v.to_string())
                .unwrap_or("??.?? ???".to_string())
        ]);
    }

    table.to_string()
}

fn fmt_amounts(amounts: &Amounts) -> String {
    amounts
        .sorted()
        .iter()
        .map(|amt| amt.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}
