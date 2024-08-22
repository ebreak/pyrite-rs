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
	client_data: HashMap<SocketAddr, ClientData>,
}

impl Server {
	#[allow(unused)]
	pub fn new(ip: [u8; 4], port: u16) -> Self {
		Server { 
			addr: SocketAddr::from((ip, port)),
			client_data: HashMap::new(),
		}
	}

	#[allow(unused)]
	pub fn start(&'static self) -> ! {
		let socket = UdpSocket::bind(self.addr).unwrap();
		
		loop {
			let mut buf = [0 as u8; MAX_TRANSMIT_SIZE];
			let (recv_size, recv_addr) = socket.recv_from(&mut buf).unwrap();
			if recv_size == 0 { continue; }

			let mut data = vec![0 as u8; recv_size];
			for i in 0..recv_size { data[i] = buf[i]; }
			thread::spawn(move || self.process(data, recv_addr));
		}
	}

	#[allow(unused)]
	pub fn process(&'static self, mut data: Vec<u8>, recv_addr: SocketAddr) {
		todo!()
	}
}
