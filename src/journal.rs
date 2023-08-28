use std::fmt::Display;

use prettytable::{row, format::FormatBuilder};

use crate::events::Event;

// enum JournalEntry {
    // Buy, Sell, Dividend,
    // TODO: Split, Merge, Top-up, Withdraw, Tax, Interest, Fee, claim-event, etc.
// }

pub struct Journal {
    // entries: Vec<JournalEntry>,
}
impl Journal {
    pub fn new(_entries: Vec<Event>) -> Self {
        Self { 
            //entries: vec![]
        }
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
        table.set_titles(row![c->"Date", c->"Type", c->"Ticker", c->"Amount", c->"Price", c->"Total"]);
        table.add_row(row![l->"2020-01-01", l->"Buy", l->"AAPL", r->"10.0", r->"100.00 USD", r->"1000.00 USD"]);
        write!(f, "My Journal\n{}", table)
    }
}
