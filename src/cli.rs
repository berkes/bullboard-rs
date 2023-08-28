use clap::{command, Command};

pub(crate) fn build_cli() -> Command {
    command!("bullboard")
}
