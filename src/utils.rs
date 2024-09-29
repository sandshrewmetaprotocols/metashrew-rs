pub fn ptr_to_vec(ptr: i32) -> Vec<u8> {
  unsafe {
    let len = *((ptr - 4) as usize as *const usize);
    Box::leak(Box::new(Vec::<u8>::from_raw_parts(ptr as usize as *mut u8, len, len))).clone()
  }
}
