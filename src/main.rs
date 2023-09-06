use std::env;

use bullboard::{
    cqrs::CqrsFramework,
    dashboard::Dashboard,
    date_utils::{now, parse_datetime_or},
    event_store::{sqlite::SqliteEventStore, EventStore},
    events::AccountEvent,
    journal::Journal,
};

mod cli;
mod demo;

mod portfolio;

#[cfg(test)]
pub mod test_utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::build_cli().get_matches();

    let db_file = env::var("BULLBOARD_DB_PATH").unwrap_or("bullboard.db".to_string());
    let cqrs = CqrsFramework::new(SqliteEventStore::new(&db_file)?);

    let output: String = match matches.subcommand() {
        Some(("demo", _)) => demo::demo().to_string(),
        Some(("add", sub_cmd)) => {
            handle_add(sub_cmd, cqrs);
            "".to_string() // TODO: decide what we want to show to the user.
        }
        Some(("journal", _)) => {
            let events = cqrs.store.get_events("ber")?;
            Journal::new(events).to_string()
        }
        Some(("dashboard", _)) => {
            let events = cqrs.store.get_events("ber")?;
            Dashboard::new(events).to_string()
        }
        Some(("init", _)) => {
            cqrs.store.init().unwrap();
            "".to_string()
        }
        Some((&_, _)) => todo!(),
        None => unreachable!(),
    };

    print!("{}", output);

    Ok(())
}

fn handle_add<T>(sub_cmd: &clap::ArgMatches, cqrs: CqrsFramework<T>)
where
    T: EventStore,
{
    let etype = sub_cmd.get_one::<String>("type").unwrap();

    let date = sub_cmd.get_one::<String>("date");
    let date_time = parse_datetime_or(date.cloned(), now).expect("Failed to parse date");

    let price = sub_cmd.get_one::<String>("price").unwrap();
    let currency = sub_cmd.get_one::<String>("currency").unwrap();
    let identifier = sub_cmd.get_one::<String>("identifier").unwrap();
    let amount = sub_cmd.get_one::<String>("amount").unwrap();

    let event = match etype.as_str() {
        "buy" => AccountEvent::new_stocks_bought(
            date_time,
            amount.parse::<f64>().unwrap(),
            format!("{} {}", price, currency),
            identifier.to_string(),
        ),
        "dividend" => AccountEvent::new_dividend_paid(
            date_time,
            format!("{} {}", price, currency),
            identifier.to_string(),
        ),
        "price" => AccountEvent::new_price_obtained(
            date_time,
            format!("{} {}", price, currency),
            identifier.to_string(),
        ),
        _ => panic!("Unknown event type"),
    };

    cqrs.store
        .persist("ber", &[event])
        .expect("Failed to persist event");
}
