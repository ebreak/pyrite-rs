use std::net::{SocketAddr, UdpSocket};

pub struct Package {
	pub sequence: i32,
	pub identifier: String,
	pub body: Vec<u8>,
}

impl Package {
	#[allow(unused)]
	pub fn new(sequence: i32, identifier: String, body: Vec<u8>) -> Self {
		Package{sequence, identifier, body}
	}

	#[allow(unused)]
	pub fn from(raw: Vec<u8>) -> Option<Self> {
		if raw.len() < 4 { return None; }
		// 小端存储
		let mut sequence: i32 = 0;
		for i in 0..4 {
			sequence = raw[i] as i32;
			sequence = sequence << 8;
		}

		let mut end_of_identifier: usize = 4;
		for i in 4..raw.len() {
			if raw[i] == 0 {
				end_of_identifier = i;
				break;
			}
		}

		// 如果 end_of_identifier 未更新，说明 raw 长度为 4，或者 raw 根本就没有 \0
		// 此时认为 raw 的剩余部分全都是 identifier
		if end_of_identifier == 4 {
			end_of_identifier = raw.len();
		}

		let identifier = String::from_utf8(
			raw[4..end_of_identifier].to_vec()
		).unwrap();
		let body = raw[end_of_identifier..raw.len()].to_vec();
		Some(Package {
			sequence,
			identifier,
			body,
		})
	}

	#[allow(unused)]
	pub fn to_bytes(&self) -> Vec<u8> {
		let mut ret: Vec<u8> = Vec::new();

		// sequence
		for i in 0..4 {
			ret.push((self.sequence >> i*8) as u8);
		}

		// identifier
		for c in self.identifier.chars() {
			ret.push(c as u8);
		}
		ret.push(0);

		// body
		ret.append(&mut self.body.clone());

		ret
	}

	#[allow(unused)]
	pub fn send_to(
		&self,
		socket: &UdpSocket,
		addr: &SocketAddr
	) -> Result<usize, std::io::Error> {
		socket.send_to(&self.to_bytes(), addr)
	}
}
