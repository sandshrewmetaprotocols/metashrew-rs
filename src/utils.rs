pub fn ptr_to_vec(ptr: i32) -> Vec<u8> {
    unsafe {
        let len = *((ptr - 4) as usize as *const usize);
        Box::leak(Box::new(Vec::<u8>::from_raw_parts(
            ptr as usize as *mut u8,
            len,
            len,
        )))
        .clone()
    }
}

pub fn format_key(v: &Vec<u8>) -> String {
    v.clone()
        .split(|c| *c == 47)
        .map(|bytes| {
            let r = String::from_utf8(bytes.to_vec());
            let is_ascii = match r {
              Ok(ref s) => s.is_ascii(),
              Err(_) => false
            };
            if is_ascii {
                "/".to_owned() + r.unwrap().as_str()
            } else {
                "/".to_owned() + hex::encode(bytes).as_str()
            }
        })
        .collect::<Vec<String>>()
        .join("")
}
