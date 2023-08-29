use clap::{command, Command, arg};

pub(crate) fn build_cli() -> Command {
    command!("bullboard")
        .propagate_version(true)
        .subcommand_required(true)
        .subcommand(Command::new("demo").about("Show a demo of the dashboard"))
        .subcommand(
            Command::new("add").about("Add a new event").arg(
                arg!(--type <TYPE> "the type of event to add")
            ).arg(
                arg!(--date <DATE> "the date of the event")
            ).arg(
                arg!(--price <PRICE> "the price of the event")
            ).arg(
                arg!(--currency <CURRENCY> "the currency of the event")
            ).arg(
                arg!(--identifier <IDENTIFIER> "the ID (symbol) of the asset")
            ).arg(
                arg!(--amount <AMOUNT> "the amount of the event").default_value("1")
            )
        )
        .subcommand(Command::new("journal").about("Show the journal"))
        .subcommand(Command::new("dashboard").about("Show the dashboard"))
        .subcommand(Command::new("init").about("Initialize the event store"))
}
