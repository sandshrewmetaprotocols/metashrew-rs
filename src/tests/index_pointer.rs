mod tests {
  use crate::index_pointer::{KeyValuePointer, IndexPointer};
  use anyhow::{Result};
  use wasm_bindgen_test::*;
  use std::sync::{Arc};
  #[wasm_bindgen_test]
  pub fn test_index_pointer() -> Result<()> {
    let ptr = IndexPointer::from_keyword("/test");
    ptr.append(Arc::new(vec![0x01, 0x02, 0x03, 0x04]));
    assert_eq!(ptr.select_index(0).get().as_ref().clone(), vec![0x01, 0x02, 0x03, 0x04]);
    Ok(())
  }
}
