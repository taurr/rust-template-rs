use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    pub name: Option<String>,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 10)]
    pub count: u8,
}
