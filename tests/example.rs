use cucumber::{gherkin::Step, given, when, World, then};

use bullboard::{StocksBought, Event, Dashboard};

#[derive(Debug, Default, World)]
pub struct BullBoardWorld {
    events: Vec<Event>,
    output: String,
}

#[given("I have the following stock transactions")]
fn i_have_the_following_stock_stransactions(world: &mut BullBoardWorld, step: &Step) {
    if let Some(table) = step.table() {
        for row in table.rows.iter().skip(1) {
            let symbol = &row[0];
            let currency = &row[1];
            let amount = &row[2];
            let price = &row[3];
            world.events.push(Event::StocksBought(StocksBought::new(
                amount.parse().unwrap(),
                price.parse().unwrap(),
                symbol.to_string(),
                currency.to_string(),
            )));
        }
    }
}

#[when("I check my dashboard")]
fn i_check_my_dashboard(world: &mut BullBoardWorld) {
    let dashboard = Dashboard::new(world.events.clone());
    world.output = dashboard.to_string();
}

#[then(expr = "I should see {string}")]
fn i_should_see(world: &mut BullBoardWorld, state: String) {
    assert!(world.output.contains(&state));
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(BullBoardWorld::run("tests/features"));
}
