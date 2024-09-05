use std::{collections::HashMap, net::{SocketAddr, UdpSocket}, sync::mpsc::Sender, thread};

use crate::{package::Package, MAX_TRANSMIT_SIZE};

type ServerHandlerPtr = fn(&SocketAddr, Vec<u8>) -> Option<Vec<u8>>;

pub struct ClientData {
	pub sequence: u64,
	pub promise_buf: HashMap<i32, Sender<Package>>,
	// pub data: Arc<Mutex<DataMap>>,
}

impl ClientData {
	#[allow(unused)]
	pub fn new() -> Self {
		ClientData {
			sequence: 0,
			promise_buf: HashMap::new(),
			// data: Arc::new(Mutex::new(DataMap::new())),
		}
	}
}

pub struct Server {
	pub socket: UdpSocket,
	router: HashMap<String, ServerHandlerPtr>,
	client_data: HashMap<SocketAddr, ClientData>,
}

impl Server {
	#[allow(unused)]
	pub fn new(ip: [u8; 4], port: u16) -> Self {
		let addr = SocketAddr::from((ip, port));
		let socket = UdpSocket::bind(addr).unwrap();
		Server {
			socket,
			router: HashMap::new(),
			client_data: HashMap::new(),
		}
	}

	#[allow(unused)]
	pub fn set_handler(&mut self, identifier: &'static str, handler: ServerHandlerPtr) -> bool {
		if identifier.starts_with("prt-") {
			return false;
		}

		self.router.insert(String::from(identifier), handler);
		return true;
	}

	#[allow(unused)]
	pub fn start(&'static mut self) -> ! {
		let mut buf = [0 as u8; MAX_TRANSMIT_SIZE];
		loop {
			let (recv_size, recv_addr) = self.socket.recv_from(&mut buf).unwrap();
			if recv_size == 0 { continue; }

			let recv_pkg = match Package::from(Vec::from(&buf[0..recv_size])) {
				Some(pkg) => pkg,
				None => continue,
			};

			if recv_pkg.identifier == "prt-ack" {
				// 处理 ack 数据包
				// 先取出对应 ClientData
				// 然后从 ClientData.promise_buf 中移除 sequence 对应的 Receiver
				// 通过 Receiver 发送 pkg，然后在 Drop 时自动回收相关资源
				match match self.client_data.get_mut(&recv_addr) {
					Some(s) => s,
					None => continue,
				}.promise_buf.remove(&recv_pkg.sequence) {
					Some(v) => v,
					None => continue,
				}.send(recv_pkg);
				// 处理 ack 的逻辑会更改所有客户端数据，
				// 所以为了并发安全，ack 的初步处理必须在 Server 主线程中进行
				// 此处理 ack 的逻辑并不会过度占用计算资源
				// 因为耗时的数据处理操作在 Channel 另一端的线程中进行
				// 此处仅有一些基本的 HashMap 操作，时间复杂度为 O(1)
				continue;
			}

			let mut handler = self.router.get(&recv_pkg.identifier);
			// 如果找不到 identifier 对应的处理函数，则查找默认处理函数
			if handler == None {
				handler = self.router.get("*");
			}

			if handler == None {
				continue;
			}

			let handler = handler.unwrap();
			let this_socket = &self.socket;
			thread::spawn(move || {
				let resp_data = handler(&recv_addr, recv_pkg.body);
				if resp_data.is_none() { return; }
				let resp_data = resp_data.unwrap();
				let resp_pkg = Package{
					sequence: recv_pkg.sequence,
					identifier: "prt-ack".to_string(),
					body: resp_data,
				};
				resp_pkg.send_to(&this_socket, &recv_addr);
			});
		}
	}
}
