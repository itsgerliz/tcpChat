use clap::Parser;
use env_logger::Env;
use log::debug;

#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
    #[arg(help = "Are we running as server?", short = 'l', long = "listening", required = false)]
    listening: bool,
    #[arg(help = "Address to connect/listen to/on", required = true)]
    address: String,
    #[arg(help = "Port to connect/listen to/on", required = true)]
    port: u16
}

fn main() {
    // Default logging level to INFO
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    // Parse arguments
    let args = CliArgs::parse();

    debug!("Starting tcpChat...");
}