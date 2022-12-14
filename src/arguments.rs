use clap::{Args, Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub(crate) enum Actions {
    Play(Index),
    #[clap(subcommand)]
    Peek(GameObject),
    EnterFight,
    EndTurn,
    #[clap(subcommand)]
    Save(Save),
}

#[derive(Subcommand, Debug)]
pub(crate) enum Save {
    Delete,
    Load(SaveRef),
}

#[derive(Args, Debug)]
pub(crate) struct SaveRef {
    pub(crate) file: String,
}

#[derive(Subcommand, Debug)]
pub(crate) enum GameObject {
    Player,
    Enemy,
    DrawPile,
    Hand,
    DiscardPile,
}

#[derive(Args, Debug)]
pub(crate) struct Index {
    pub(crate) index: usize,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Main {
    #[clap(subcommand)]
    pub(crate) command: Actions,
}
