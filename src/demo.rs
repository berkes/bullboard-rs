use std::fmt::Display;

use bullboard::{events::AccountEvent, dashboard::Dashboard, date_utils::fixtures::iphone_launched_at};

pub fn demo() -> impl Display {
    // Simulating events
    let events = vec![
        AccountEvent::new_stocks_bought(iphone_launched_at(), 10.0, "150.0 USD".to_string(), "AAPL".to_string()),
        AccountEvent::new_price_obtained(
            chrono::NaiveDate::from_ymd_opt(2020, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            "170.0 USD".to_string(),
            "AAPL".to_string(),
        ),
        AccountEvent::new_stocks_bought(iphone_launched_at(), 5.0, "160.0 USD".to_string(), "AAPL".to_string()),
        AccountEvent::new_price_obtained(
            chrono::NaiveDate::from_ymd_opt(2020, 2, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            "160.0 USD".to_string(),
            "AAPL".to_string(),
        ),
        AccountEvent::new_stocks_bought(iphone_launched_at(), 4.0, "13.37 EUR".to_string(), "ASR.AS".to_string()),
        AccountEvent::new_price_obtained(
            chrono::NaiveDate::from_ymd_opt(2020, 2, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            "14.20 EUR".to_string(),
            "ASR.AS".to_string(),
        ),
        AccountEvent::new_stocks_bought(iphone_launched_at(), 8.0, "100.0 USD".to_string(), "MSFT".to_string()),
        AccountEvent::new_price_obtained(
            chrono::NaiveDate::from_ymd_opt(2020, 2, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            "110.0 USD".to_string(),
            "MSFT".to_string(),
        ),
    ];

    Dashboard::new(events)
}
