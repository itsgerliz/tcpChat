use clap::Parser;
use env_logger::Env;
use log::debug;
use std::net::{SocketAddr, TcpListener, TcpStream};

mod client;

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

#[repr(u8)]
enum OpCode {
    Hello = 0x01,
    HelloAck = 0x02,
    Login = 0x03,
    SessionInfo = 0x04,
    LoginError = 0x05,
    Synchronize = 0x06,
    SessionTerminate = 0x07
}

struct TcpChatMessage {
    version: u8,
    opcode: OpCode,
    payload_size: u16,
    payload: Option<[u8; 512]>
}

struct Listener {
    inner: TcpListener
}

struct Stream {
    inner: TcpStream,
    address: SocketAddr
}

fn main() {
    // Default logging level to INFO
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    // Parse arguments
    let args = CliArgs::parse();
    let socket = (args.address.as_str(), args.port);

    debug!("Starting tcpChat...");
    if args.listening == false {
        // Launch client side
        client::init(&socket);
    } else {
        // Launch server side
    }
}