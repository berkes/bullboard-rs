use bullboard::{dashboard::Dashboard, events::Event};

fn main() {
    // Simulating events
    let events = vec![
        Event::new_stocks_bought(10.0, "150.0 USD".to_string(), "AAPL".to_string()),
        Event::new_price_obtained(
            chrono::NaiveDate::from_ymd_opt(2020, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            "170.0 USD".to_string(),
            "AAPL".to_string(),
        ),
        Event::new_stocks_bought(5.0, "160.0 USD".to_string(), "AAPL".to_string()),
        Event::new_price_obtained(
            chrono::NaiveDate::from_ymd_opt(2020, 2, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            "160.0 USD".to_string(),
            "AAPL".to_string(),
        ),
        Event::new_stocks_bought(8.0, "100.0 USD".to_string(), "MSFT".to_string()),
        Event::new_price_obtained(
            chrono::NaiveDate::from_ymd_opt(2020, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            "110.0 USD".to_string(),
            "MSFT".to_string(),
        ),
    ];

    let dashboard = Dashboard::new(events);
    println!("{}", dashboard);
}
