use bullboard::{Dashboard, Event};

fn main() {
    // Simulating events
    let events = vec![
        Event::StocksBought {
            amount: 10.0,
            price: 150.0,
            ticker: "AAPL".to_string(),
            currency: "USD".to_string(),
        },
        Event::StocksBought {
            amount: 5.0,
            price: 160.0,
            ticker: "AAPL".to_string(),
            currency: "USD".to_string(),
        }
    ];

    let dashboard = Dashboard::new(events);

    println!(
        "Total Buying Price: {} {}",
        dashboard.total_buying_price, dashboard.currency
    );
    println!(
        "Total Value: {} {}",
        dashboard.total_value(),
        dashboard.currency
    );
    println!("Number of Positions: {}", dashboard.number_of_positions);
    println!(
        "Total Dividend: {} {}",
        dashboard.total_dividend, dashboard.currency
    );
}
