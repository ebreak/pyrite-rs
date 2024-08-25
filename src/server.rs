use core::panic;
use std::{collections::HashMap, net::{SocketAddr, UdpSocket}, sync::{mpsc::{Receiver, Sender}, Arc, Mutex}, thread};

use crate::{data::DataMap, package::Package, MAX_TRANSMIT_SIZE};

pub struct ClientData {
	pub sequence: u64,
	pub promise_buf: HashMap<i32, Sender<Package>>,
	pub data: DataMap,
}

impl ClientData {
	#[allow(unused)]
	pub fn new() -> Self {
		ClientData {
			sequence: 0,
			promise_buf: HashMap::new(),
			data: DataMap::new(),
		}
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
	pub fn start(&mut self) -> ! {
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
			}

			todo!()
		}
	}
}
