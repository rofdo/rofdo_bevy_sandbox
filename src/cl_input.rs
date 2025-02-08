pub use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct ClInput {
    #[clap(short, long, default_value = "0.0.0.0")]
    address: String,
    #[clap(short, long, default_value = "8080")]
    port: u16,
}

