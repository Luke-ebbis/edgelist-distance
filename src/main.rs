use clap::ValueEnum;
use clap::{Parser, Subcommand};
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum OutputFormat {
    Ttl,
    Json,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Subcommands for rdfbio
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Query OmicsDI endpoint
    Query {
        /// The search query (required)
        #[arg()]
        query: String,

        /// The size of the results to fetch
        #[arg(short = 'S', long, default_value = "10")]
        size: usize,

        /// The start index for the search results
        #[arg(short = 's', long, default_value = "0")]
        start: usize,

        /// Output format: RDF (ttl) or JSON (json). The JSON retrieves the raw database hits, the ttl retrieves the linked data as interepreted by this program.
        #[arg(value_enum, short, long, default_value_t = OutputFormat::Json)]
        format: OutputFormat,

        /// Output file (optional). If not provided, prints to stdout.
        #[arg(short, long)]
        output: Option<String>,
    },
}
use log;
fn main() {
    env_logger::init();
    let cli = Cli::parse();
}
