use clap::{ValueEnum, Parser};

#[derive(Parser, Debug, Clone)]
struct ClInput {
    #[clap(short, long, default_value = "Server")]
    role: Role,
    #[clap(short, long, default_value = "0.0.0.0")]
    address: String,
    #[clap(short, long, default_value = "8080")]
    port: u16,
}

#[derive(ValueEnum, Debug, Clone)]
enum Role {
    Server,
    Client,
}
