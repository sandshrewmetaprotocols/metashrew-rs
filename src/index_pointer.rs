use crate::byte_view::ByteView;
use crate::{flush, get, input, set};
use std::mem::{size_of, size_of_val};
use std::sync::Arc;

pub struct IndexPointer(Arc<Vec<u8>>);

impl IndexPointer {
    pub fn from_keyword(keyword: &str) -> IndexPointer {
        IndexPointer::wrap(&keyword.to_string().clone().into_bytes())
    }
    pub fn wrap(word: &Vec<u8>) -> IndexPointer {
        IndexPointer(Arc::<Vec<u8>>::new(word.clone()))
    }
    pub fn unwrap(&self) -> Arc<Vec<u8>> {
        self.0.clone()
    }
    pub fn set(&self, v: Arc<Vec<u8>>) {
        set(self.unwrap(), v)
    }
    pub fn get(&self) -> Arc<Vec<u8>> {
        get(self.unwrap())
    }
    pub fn select(&self, word: &Vec<u8>) -> IndexPointer {
        let mut key = (*self.unwrap()).clone();
        key.extend(word);
        return IndexPointer::wrap(&key);
    }
    pub fn keyword(&self, word: &String) -> IndexPointer {
        let mut key = (*self.unwrap()).clone();
        key.extend(&word.clone().into_bytes());
        return IndexPointer::wrap(&key);
    }

    pub fn set_value<T: ByteView>(&self, v: T) {
        self.set(Arc::new(T::to_bytes(v)));
    }

    pub fn get_value<T: ByteView>(&self) -> T {
        T::from_bytes(self.get().as_ref().clone())
    }

    pub fn select_value<T: ByteView>(&self, key: T) -> Self {
        self.select(T::to_bytes(key).as_ref())
    }
    pub fn length_key(&self) -> Self {
        self.keyword(&"/length".to_string())
    }
    pub fn length(&self) -> u32 {
        self.length_key().get_value()
    }
}
