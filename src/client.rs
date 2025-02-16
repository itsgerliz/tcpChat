use log::{debug, info, error};
use crate::Stream;
use std::{net::TcpStream, process::exit};

impl Stream {
	fn new(address: &(&str, u16)) -> Stream {
		debug!("Trying to connect to {}:{}", (*address).0, (*address).1);
		match TcpStream::connect(*address) {
			Ok(stream) => {
				info!("Connected to {}:{}", (*address).0, (*address).1);
				Stream {
					inner: stream,
					address: None
				}
			}
			Err(errno) => {
				error!("Cannot connect to {}:{}", (*address).0, (*address).1);
				error!("OS sent: {}", errno);
				exit(1);
			}
		}
	}
}

pub(crate) fn init(socket_address: &(&str, u16)) {
	let session_stream = Stream::new(socket_address);
}