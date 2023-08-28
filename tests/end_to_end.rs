use bullboard::{dashboard::Dashboard, events::Event, journal::Journal, event_store::{EventStore, MemoryEventStore}};
use chrono::{NaiveDate, NaiveDateTime};
use cucumber::{gherkin::Step, given, then, when, World};
use pretty_assertions::assert_eq;

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
            let money = format!("{} {}", price, currency);
            world
                .events
                .push(Event::new_stocks_bought(amount, money, ticker))
        }
    }
}

#[given("I have an empty journal")]
fn i_have_an_empty_journal(world: &mut BullBoardWorld) {
    world.events = vec![];
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
            let currency: String = row[1].to_string();
            let price: f64 = row[2].parse().unwrap();
            let money = format!("{} {}", price, currency);

            world
                .events
                .push(Event::new_price_obtained(obtained_at, money, ticker));
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
    let money = format!("{} {}", amount, currency);

    world.events.push(Event::new_dividend_paid(money, ticker));
}

#[when("I have the following stock transactions")]
fn i_have_the_following_stock_transactions(world: &mut BullBoardWorld, step: &Step) {
    if let Some(table) = step.table() {
        for row in table.rows.iter().skip(1) {
            let ticker: String = row[0].parse().unwrap();
            let currency: String = row[1].parse().unwrap();
            let amount: f64 = row[2].parse().unwrap();
            let price: f64 = row[3].parse().unwrap();
            let money = format!("{} {}", price, currency);

            world
                .events
                .push(Event::new_stocks_bought(amount, money, ticker));
        }
    }
}

#[when("I add a journal entry")]
fn i_add_a_journal_entry(world: &mut BullBoardWorld) {
    let event = Event::new_stocks_bought(10.0, "100.00 USD".to_string(), "AAPL".to_string());
    world.events.push(event);
}

#[when("I restart the application")]
fn i_restart_the_application(world: &mut BullBoardWorld) {
    let event_store = MemoryEventStore::new();
    event_store.persist("ber", &world.events).unwrap();
    world.events = event_store.get_events("ber").unwrap();
}

#[then("I should see the entry in my journal")]
fn i_should_see_the_entry_in_my_journal(world: &mut BullBoardWorld) {
    let journal = Journal::new(world.events.clone());
    let expected = r#"My Journal
     Date       Type    Ticker    Amount      Price          Total 
  2020-01-01    Buy     AAPL        10.0    100.00 USD    1000.00 USD 
"#;

    assert_eq!(journal.to_string(), expected);
}

#[then(expr = "I should see {string}")]
fn i_should_see(world: &mut BullBoardWorld, state: String) {
    assert!(
        normalize_whitespace(&world.output).contains(&state),
        "expected to find {} in {}",
        state,
        &world.output
    );
}

#[then(expr = "I should see the following text")]
fn i_should_see_following(world: &mut BullBoardWorld, step: &Step) {
    if let Some(content) = step.docstring() {
        assert_eq!(&world.output, content);
    }
}

fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(BullBoardWorld::run("tests/features"));
}
