use std::{collections::HashMap, mem, os::raw::c_void};

use libc::memcpy;

pub struct DataMap {
	int_map: HashMap<String, i64>,
	string_map: HashMap<String, String>,
	any_map: HashMap<String, Vec<u8>>,
}

impl DataMap {
	pub fn set_int(&mut self, key: String, value: i64) -> Option<i64> {
		self.int_map.insert(key, value)
	}
	
	pub fn set_string(&mut self, key: String, value: String) -> Option<String> {
		self.string_map.insert(key, value)
	}

	pub unsafe fn set_any<T: Copy>(&mut self, key: String, value: &T) {
		let ptr = value as *const T as *const u8;
		let raw = std::slice::from_raw_parts(ptr, mem::size_of::<T>());
		let vec = Vec::from(raw);
		self.any_map.insert(key, vec);
	}

	pub fn get_int(&mut self, key: &String) -> Option<&i64> {
		self.int_map.get(key)
	}

	pub fn get_string(&mut self, key: &String) -> Option<&String> {
		self.string_map.get(key)
	}

	pub unsafe fn get_any<T: Copy>(&mut self, key: &String) -> Option<T> {
		match self.any_map.get(key) {
			Some(vec) => {
				if vec.len() != mem::size_of::<T>() { return None; }
				let ptr = libc::malloc(mem::size_of::<T>() as libc::size_t) as *mut c_void;
				memcpy(ptr, vec.as_ptr() as *const c_void, mem::size_of::<T>());
				Some(*(ptr as *const T))
			},
			None => None,
		}
	}
}
