use std::sync::Arc;
//use std::io::{Write, Result};
use crate::compat::{to_arraybuffer_layout, to_ptr};
use crate::imports::__log;
pub use std::fmt::{Error, Write};

pub struct Stdout(());

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        let data = Arc::new(s.to_string().as_bytes().to_vec());
        log(data.clone());
        return Ok(());
    }
}

pub fn stdout() -> Stdout {
    Stdout(())
}

#[macro_export]
macro_rules! println {
  ( $( $x:expr ),* ) => {
    {
      writeln!(stdout(), $($x),*).unwrap();
    }
  }
}

/*
#[cfg(not(test))]
#[link(wasm_import_module = "env")]
extern "C" {
    fn __log(ptr: i32);
}
*/

#[allow(unused_unsafe)]
pub fn log(v: Arc<Vec<u8>>) -> () {
    unsafe {
        __log(to_ptr(&mut to_arraybuffer_layout(v)) + 4);
    }
}
