use log::{debug, info, error};
use crate::{OpCode, Stream, H_SIZE, H_VERSION};
use std::{net::TcpStream, process::exit, io::Write};

impl Stream {
	// Open TCP stream to endpoint
	fn new(address: &(&str, u16)) -> Self {
		debug!("Trying to connect to {}:{}", (*address).0, (*address).1);
		// Try to open a TCP stream to the specified address
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

	// Send client hello
	fn client_hello(&mut self) -> bool {
		debug!("Trying to send client hello");

		let mut buffer: [u8; H_SIZE as usize] = [0; H_SIZE as usize];
		buffer[0] = H_VERSION;
		buffer[1] = OpCode::Hello as u8;
		for i in 2..4 {
			buffer[i] = 0;
		}

		match (&mut(*self).inner).write(&buffer) {
			Ok(bytes_sent) => {
				(&mut(*self).inner).flush().expect("Cannot flush TCP stream!");
				assert_eq!(bytes_sent, buffer.len());
				debug!("Sent client hello");
				true
			}
			Err(errno) => {
				error!("Cannot send client hello");
				error!("OS sent: {}", errno);
				false
			}
		}
	}
}

pub(crate) fn init(socket_address: &(&str, u16)) {
	let session_stream = Stream::new(socket_address);
}