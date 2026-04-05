use clap::Parser;

use crate::ports::ProtocolScope;

#[derive(Debug, Parser)]
#[command(
    name = "port-recommender",
    version,
    about = "Recommend a deterministic available port on the current machine."
)]
pub struct Cli {
    /// Name to map into a recommended port.
    pub name: String,

    /// Protocols to consider when excluding occupied local ports.
    #[arg(long, value_enum, default_value = "both")]
    pub protocol: ProtocolScope,
}
