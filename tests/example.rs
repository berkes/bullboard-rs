use bullboard::{Dashboard, Event};
use chrono::{NaiveDate, NaiveDateTime};
use cucumber::{gherkin::Step, given, then, when, World};

#[derive(Debug, Default, World)]
pub struct BullBoardWorld {
    events: Vec<Event>,
    output: String,
}

#[given("I have the following stock transactions")]
fn i_have_the_following_stock_stransactions(world: &mut BullBoardWorld, step: &Step) {
    if let Some(table) = step.table() {
        for row in table.rows.iter().skip(1) {
            let ticker: String = row[0].to_string();
            let currency: String = row[1].parse().unwrap();
            let amount: f64 = row[2].parse().unwrap();
            let price: f64 = row[3].parse().unwrap();
            world.events.push(Event::StocksBought {
                ticker,
                amount,
                price,
                currency,
            });
        }
    }
}

#[when("I check my dashboard")]
fn i_check_my_dashboard(world: &mut BullBoardWorld) {
    let dashboard = Dashboard::new(world.events.clone());
    world.output = dashboard.to_string();
}

#[when(expr = "the prices change to the following values on {string}")]
fn the_prices_change_to_the_following_values_on(
    world: &mut BullBoardWorld,
    date: String,
    step: &Step,
) {
    if let Some(table) = step.table() {
        let obtained_at: NaiveDateTime = NaiveDate::parse_from_str(&date, "%d-%m-%Y")
            .expect("parse date")
            .and_hms_opt(0, 0, 0)
            .expect("convert to datetime");

        for row in table.rows.iter().skip(1) {
            let ticker: String = row[0].to_string();
            let price: f64 = row[1].parse().unwrap();
            let currency: String = "USD".to_string();

            world.events.push(Event::PriceObtained {
                obtained_at,
                ticker,
                price,
                currency,
            });
        }
    }
}

#[when(expr = "{string} pays {string} dividend per share on {string}")]
fn pays_dividend_per_share_on(
    world: &mut BullBoardWorld,
    ticker: String,
    dividend: String,
    _date: String,
) {
    let mut dividend_parts = dividend.split_whitespace();
    let amount = dividend_parts.next().unwrap().parse::<f64>().unwrap();
    let currency = dividend_parts.next().unwrap().to_string();

    // let paid_at: NaiveDateTime = NaiveDate::parse_from_str(&date, "%d-%m-%Y")
    //     .expect("parse date")
    //     .and_hms_opt(0, 0, 0)
    //     .expect("convert to datetime");

    world.events.push(Event::DividendPaid {
        amount,
        ticker,
        currency,
    });
}

#[when("I have the following stock transactions")]
fn i_have_the_following_stock_transactions(world: &mut BullBoardWorld, step: &Step) {
    if let Some(table) = step.table() {
        for row in table.rows.iter().skip(1) {
            let ticker: String = row[0].parse().unwrap();
            let currency: String = row[1].parse().unwrap();
            let amount: f64 = row[2].parse().unwrap();
            let price: f64 = row[3].parse().unwrap();

            world.events.push(Event::StocksBought {
                ticker,
                amount,
                price,
                currency,
            });
        }
    }
}

#[then(expr = "I should see {string}")]
fn i_should_see(world: &mut BullBoardWorld, state: String) {
    assert!(
        world.output.contains(&state),
        "expected to find {} in {}",
        state,
        &world.output
    );
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(BullBoardWorld::run("tests/features"));
}
