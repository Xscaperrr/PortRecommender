use clap::Parser;

use port_recommender::cli::Cli;
use port_recommender::{NetstatPortUsageProvider, recommend_port};

fn main() {
    let cli = Cli::parse();
    let provider = NetstatPortUsageProvider;

    match recommend_port(&cli.name, cli.protocol, &provider) {
        Ok(port) => println!("{port}"),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
