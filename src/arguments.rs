use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub(crate) enum Actions {
    Attack,
    Status,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Main {
    #[clap(subcommand)]
    pub(crate) command: Actions,
}
