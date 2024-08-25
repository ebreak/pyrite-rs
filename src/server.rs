use std::{collections::HashMap, net::{SocketAddr, UdpSocket}, thread};

use crate::{data::DataMap, MAX_TRANSMIT_SIZE};

pub struct ClientData {
	pub sequence: u64,
	// promise channel
	pub data: DataMap,
}

impl ClientData {
	#[allow(unused)]
	pub fn new() -> Self {
		ClientData { sequence: 0, data: DataMap::new() }
	}
}

pub struct Server {
	pub addr: SocketAddr,
	pub socket: UdpSocket,
	client_data: HashMap<SocketAddr, ClientData>,
}

impl Server {
	#[allow(unused)]
	pub fn new(ip: [u8; 4], port: u16) -> Self {
		let addr = SocketAddr::from((ip, port));
		let socket = UdpSocket::bind(addr).unwrap();
		Server { 
			addr,
			socket,
			client_data: HashMap::new(),
		}
	}

	#[allow(unused)]
	pub fn start(&'static self) -> ! {
		let mut buf = [0 as u8; MAX_TRANSMIT_SIZE];
		loop {
			let (recv_size, recv_addr) = self.socket.recv_from(&mut buf).unwrap();
			if recv_size == 0 { continue; }

			let data = Vec::from(&buf[0..recv_size]);
			thread::spawn(move || self.process(data, recv_addr));
		}
	}

	#[allow(unused)]
	pub fn process(&'static self, mut data: Vec<u8>, recv_addr: SocketAddr) {
		todo!()
	}
}
