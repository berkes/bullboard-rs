use std::{process::Command, path, env};

use bullboard::value_objects::Amount;
use cucumber::{gherkin::Step, given, then, when, World};
use pretty_assertions::assert_eq;
use tempfile::TempDir;

#[derive(Debug, Default, World)]
pub struct BullboardWorld {
    last_command_output: String,

    bin: path::PathBuf,

    db_path: String,
    db_dir: Option<TempDir>,
}

impl BullboardWorld {
    fn run_command(&mut self, args: &str) {
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_string()).collect();

        env::set_var("BULLBOARD_DB_PATH", &self.db_path);
        self.bin = path::PathBuf::from(env!("CARGO_BIN_EXE_bullboard"));

        let output = Command::new(&self.bin)
            .args(&args)
            .output()
            .expect(&format!("Failed to execute command: {} {}", &self.bin.to_string_lossy(), args.join(" ")));

        if !output.status.success() {
            panic!(
                "Command {} failed with status code {}.\nOutput: {}",
                format!("{} {}", self.bin.to_string_lossy(), args.join(" ")),
                output.status,
                String::from_utf8_lossy(&output.stderr)
            );
        }
        self.last_command_output = String::from_utf8_lossy(&output.stdout).to_string();
    }
}
#[given("a database file to store events")]
fn a_database_file_to_store_events(world: &mut BullboardWorld) {
    let temp_dir = tempfile::tempdir().expect("Failed to create tmp directory");
    let db_path = temp_dir.path().join("test.db");
    let db_path_str = db_path.to_str().expect("Failed to convert path to string");

    world.db_path = db_path_str.to_string();
    world.db_dir = Some(temp_dir);
    world.run_command("init");
}

#[given("I have the following stock transactions")]
fn i_have_the_following_stock_stransactions(world: &mut BullboardWorld, step: &Step) {
    if let Some(table) = step.table() {
        for row in table.rows.iter().skip(1) {
            let ticker: String = row[0].to_string();
            let currency: String = row[1].parse().unwrap();
            let amount: f64 = row[2].parse().unwrap();
            let price: f64 = row[3].parse().unwrap();
            world.run_command(&format!(
                "add --type buy --amount {} --price {} --currency {} --identifier {}",
                amount, price, currency, ticker
            ));
        }
    }
}

#[given("I have an empty journal")]
fn i_have_an_empty_journal(_world: &mut BullboardWorld) {
    // TODO: remove the journal file
}

#[when("I check my dashboard")]
fn i_check_my_dashboard(world: &mut BullboardWorld) {
    world.run_command("dashboard");
}

#[when(expr = "the prices change to the following values on {string}")]
fn the_prices_change_to_the_following_values_on(
    world: &mut BullboardWorld,
    date: String,
    step: &Step,
) {
    if let Some(table) = step.table() {
        for row in table.rows.iter().skip(1) {
            let ticker: String = row[0].to_string();
            let currency: String = row[1].to_string();
            let price: f64 = row[2].parse().unwrap();

            world.run_command(&format!(
                "add --type price --price {} --currency {} --identifier {} --date {}",
                price, currency, ticker, date
            ));
        }
    }
}

#[when(expr = "{string} pays {string} dividend per share on {string}")]
fn pays_dividend_per_share_on(
    world: &mut BullboardWorld,
    ticker: String,
    dividend: String,
    _date: String,
) {
    let amount: Amount = dividend.into();
    let price = amount.num;
    let currency = amount.currency.to_string();

    world.run_command(&format!(
        "add --type dividend --price {} --currency {} --identifier {}",
        price, currency, ticker
    ));
}

#[when("I have the following stock transactions")]
fn i_have_the_following_stock_transactions(world: &mut BullboardWorld, step: &Step) {
    if let Some(table) = step.table() {
        for row in table.rows.iter().skip(1) {
            let ticker: String = row[0].parse().unwrap();
            let currency: String = row[1].parse().unwrap();
            let amount: f64 = row[2].parse().unwrap();
            let price: f64 = row[3].parse().unwrap();

            world.run_command(&format!(
                "add --type buy --amount {} --price {} --currency {} --identifier {}",
                amount, price, currency, ticker
            ));
        }
    }
}

#[when("I add a journal entry")] // TODO: merge with "I have the following stock transactions"
fn i_add_a_journal_entry(world: &mut BullboardWorld) {
    world.run_command("add --type buy --amount 10 --price 100 --currency USD --identifier AAPL");
}

#[when("I restart the application")]
fn i_restart_the_application(_world: &mut BullboardWorld) {
    // no-op.  TODO: remove once we have the CLI setup, which is stateless by design
}

#[then("I should see the entry in my journal")]
fn i_should_see_the_entry_in_my_journal(world: &mut BullboardWorld) {
    world.run_command("journal");
    let actual = &world.last_command_output;

    let expected = r#"My Journal
     Date       Type    Ticker    Amount      Price          Total 
  2020-01-01    Buy     AAPL        10.0    100.00 USD    1000.00 USD 
"#;

    assert_eq!(actual, expected);
}

#[then(expr = "I should see {string}")]
fn i_should_see(world: &mut BullboardWorld, state: String) {
    assert!(
        normalize_whitespace(&world.last_command_output).contains(&state),
        "expected to find {} in {}",
        state,
        &world.last_command_output
    );
}

#[then(expr = "I should see the following text")]
fn i_should_see_following(world: &mut BullboardWorld, step: &Step) {
    if let Some(content) = step.docstring() {
        assert_eq!(&world.last_command_output, content);
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
    futures::executor::block_on(BullboardWorld::run("tests/features"));
}
