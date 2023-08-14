use bullboard::{Dashboard, Event, StocksBought};

fn main() {
    // Simulating events
    let events = vec![
        Event::StocksBought(StocksBought::new(
            10.0,
            150.0,
            "AAPL".to_string(),
            "USD".to_string(),
        )),
        Event::StocksBought(StocksBought::new(
            5.0,
            160.0,
            "AAPL".to_string(),
            "USD".to_string(),
        )),
    ];

    let dashboard = Dashboard::new(events);

    println!("Total Buying Price: {} {}", dashboard.total_buying_price, dashboard.currency);
    println!("Total Value: {} {}", dashboard.total_value(), dashboard.currency);
    println!("Number of Positions: {}", dashboard.number_of_positions);
    println!("Total Dividend: {} {}", dashboard.total_dividend, dashboard.currency);
}
