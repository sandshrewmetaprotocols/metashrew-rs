extern crate alloc;
use std::collections::HashMap;
use std::panic;
use std::sync::Arc;
use protobuf::Message;

pub mod byte_view;
pub mod compat;
pub mod index_pointer;
pub mod stdio;
pub mod proto;
pub mod imports;
use crate::compat::{panic_hook, to_arraybuffer_layout, to_ptr};
pub use crate::stdio::stdout;
use crate::proto::metashrew::{KeyValueFlush};
use crate::imports::{
  __get,
  __get_len,
  __flush,
  __host_len,
  __load_input
};


pub fn ptr_to_vec(ptr: i32) -> Vec<u8> {
  unsafe {
    let len = *((ptr - 4) as usize as *const usize);
    Vec::<u8>::from_raw_parts(ptr as usize as *mut u8, len, len)
  }
}



static mut CACHE: Option<HashMap<Arc<Vec<u8>>, Arc<Vec<u8>>>> = None;
static mut TO_FLUSH: Option<Vec<Arc<Vec<u8>>>> = None;

pub fn get(v: Arc<Vec<u8>>) -> Arc<Vec<u8>> {
    unsafe {
        if CACHE.as_ref().unwrap().contains_key(&v.clone()) {
            return CACHE.as_ref().unwrap().get(&v.clone()).unwrap().clone();
        }
        let mut key = to_arraybuffer_layout(v.clone());
        let mut value = to_arraybuffer_layout(Arc::new(Vec::<u8>::with_capacity(__get_len(
            to_ptr(&mut key) + 4,
        )
            as usize)));
        __get(to_ptr(&mut key) + 4, to_ptr(&mut value) + 4);
        let result = Arc::<Vec<u8>>::new(value[4..].to_vec());
        CACHE.as_mut().unwrap().insert(v.clone(), result.clone());
        return result.clone();
    }
}

pub fn set(k: Arc<Vec<u8>>, v: Arc<Vec<u8>>) {
    unsafe {
        CACHE.as_mut().unwrap().insert(k.clone(), v.clone());
        TO_FLUSH.as_mut().unwrap().push(k.clone());
    }
}

pub fn flush() {
    unsafe {
        let mut to_encode: Vec<Vec<u8>> = Vec::<Vec<u8>>::new();
        for item in TO_FLUSH.as_ref().unwrap() {
            to_encode.push((*item.clone()).clone());
            to_encode.push((*(CACHE.as_ref().unwrap().get(item).unwrap().clone())).clone());
        }
        TO_FLUSH = Some(Vec::<Arc<Vec<u8>>>::new());
        let mut buffer = KeyValueFlush::new();
        buffer.list = to_encode;
        let serialized = buffer.write_to_bytes().unwrap();
        __flush(to_ptr(&mut to_arraybuffer_layout(Arc::new(serialized.to_vec()))) + 4);
    }
}

pub fn input() -> Vec<u8> {
    unsafe {
        let length: i32 = __host_len().into();
        let mut buffer = Vec::<u8>::new();
        buffer.extend_from_slice(&length.to_le_bytes());
        buffer.resize((length as usize) + 4, 0);
        __load_input(to_ptr(&mut buffer) + 4);
        return buffer[4..].to_vec();
    }
}

pub fn initialize() -> () {
    unsafe {
        if CACHE.is_none() {
            CACHE = Some(HashMap::<Arc<Vec<u8>>, Arc<Vec<u8>>>::new());
            panic::set_hook(Box::new(panic_hook));
        }
        TO_FLUSH = Some(Vec::<Arc<Vec<u8>>>::new());
    }
}
