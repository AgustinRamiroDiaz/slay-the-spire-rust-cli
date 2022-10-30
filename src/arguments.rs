use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = ("peke".to_string()))]
    pub(crate) name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    pub(crate) count: u8,
}
