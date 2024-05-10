use crate::byte_view::ByteView;
use crate::IndexPointer;
use std::fmt::Debug;
use std::mem::size_of;
use std::ops::{BitAnd, Shr};
use std::sync::Arc;

pub struct BST<K> {
    ptr: IndexPointer,
    _val: K,
}

pub fn maskLowerThan(v: Arc<Vec<u8>>, position: u8) -> [u64; 4] {
    let mut ar: [u64; 4] = [0, 0, 0, 0];
    ar[0] = u64::from_le_bytes(v.clone().as_slice()[0..8].try_into().unwrap());
    ar[1] = u64::from_le_bytes(v.clone().as_slice()[8..16].try_into().unwrap());
    ar[2] = u64::from_le_bytes(v.clone().as_slice()[16..24].try_into().unwrap());
    ar[3] = u64::from_le_bytes(v.clone().as_slice()[24..32].try_into().unwrap());

    let bit_selected = position % 64;
    let byte_selected = position / 64;

    ar[byte_selected as usize] = ar[byte_selected as usize]
        & u64::try_from(((1 << bit_selected) - 1) << (64 - bit_selected)).unwrap();

    for i in byte_selected + 1..4 {
        ar[i as usize] = 0;
    }

    ar
}

macro_rules! pass_down_binary_search {
    ($high: tt, $low: tt, $for_highest: tt, $size: tt, $zero: tt, $t2: tt, $x: tt, $pass: tt) => {
        if ($for_highest || $low == $zero) || $high != $zero {
            if $pass {
                return 0;
            }
            binary_search_inner::<$t2>($high, $for_highest, $size / 2)
        } else {
            if $pass {
                return 1;
            }
            $x + binary_search_inner::<$t2>($low, $for_highest, $size / 2)
        }
    };
}

macro_rules! pass_down_higher {
    ($word: tt, $for_highest: tt, $size: tt, $T: tt, $t2: tt) => {{
        let low: $t2 = $T::try_into($T::from($word & $T::try_from($t2::MAX).unwrap())).unwrap();
        let shr = $T::try_from(size_of::<$t2>() * 8).unwrap();
        let high: $t2 = $T::try_into(
            $T::try_from($T::try_from($word >> shr).unwrap() & $T::try_from($t2::MAX).unwrap())
                .unwrap(),
        )
        .unwrap();
        let zero = 0 as $t2;
        let x = i32::from(i32::try_from(size_of::<$t2>()).unwrap() * 8);
        pass_down_binary_search!(high, low, $for_highest, $size, zero, $t2, x, false)
    }};
}

pub fn binary_search(high: u128, low: u128, for_highest: bool) -> i32 {
    let x = i32::try_from(size_of::<u128>() * 8).unwrap();
    let zero = u128::try_from(0).unwrap();
    pass_down_binary_search!(high, low, for_highest, 256, zero, u128, x, false)
}

pub fn binary_search_inner<T>(word: T, for_highest: bool, size: u16) -> i32
where
    T: ByteView + Shr + BitAnd + Eq + PartialEq + Copy + Debug,
    T: From<<T as Shr>::Output>,
    T: From<<T as BitAnd>::Output>,
    T: TryInto<u8> + TryInto<u16> + TryInto<u32> + TryInto<u64> + Into<u128>,
    T: From<u8> + TryFrom<usize> + TryFrom<u16> + TryFrom<u32> + TryFrom<u64> + TryFrom<u128>,
    <T as TryFrom<usize>>::Error: Debug,
    <T as TryFrom<u16>>::Error: Debug,
    <T as TryInto<u16>>::Error: Debug,
    <T as TryFrom<u8>>::Error: Debug,
    <T as TryFrom<u32>>::Error: Debug,
    <T as TryInto<u8>>::Error: Debug,
    <T as TryInto<u32>>::Error: Debug,
    <T as TryInto<u64>>::Error: Debug,
    <T as TryFrom<u64>>::Error: Debug,
{
    match size {
        128 => {
            pass_down_higher!(word, for_highest, size, T, u64)
        }
        64 => {
            pass_down_higher!(word, for_highest, size, T, u32)
        }
        32 => {
            pass_down_higher!(word, for_highest, size, T, u16)
        }
        16 => {
            pass_down_higher!(word, for_highest, size, T, u8)
        }
        2 | 4 | 8 => {
            let rhs = if size > 4 { 0x0f } else { size - 1 };
            let high = T::from(
                T::from(word >> T::try_from(size / 2).unwrap()) & T::try_from(rhs).unwrap(),
            );
            let low = T::from(word & T::try_from(rhs).unwrap());
            let zero = T::from(0);
            let condition = size == 2;
            let x = i32::from(size / 2);
            pass_down_binary_search!(high, low, for_highest, size, zero, T, x, condition)
        }
        _ => 0,
    }
}

impl<K: ByteView + Sized> BST<K> {
    pub fn new(ptr: IndexPointer) -> Self {
        BST {
            ptr,
            _val: K::from_bytes(vec![0]),
        }
    }
    pub fn mark_path(&self, key: K) {}
    pub fn get_mask_pointer(&self, partial_key: Vec<u8>) -> IndexPointer {
        self.ptr
    }
    pub fn unmark_path(&self, key: K) {}
    fn _findBoundaryFromPartial(key_bytes: Vec<u8>, seek_higher: bool) -> K {
        K::from_bytes(vec![0])
    }
    pub fn seek_lower(start: K) -> K {
        K::from_bytes(vec![0])
    }
    pub fn seek_greater(start: K) -> K {
        K::from_bytes(vec![0])
    }
    pub fn set(k: K, v: Vec<u8>) {}
    pub fn get(k: K) -> Vec<u8> {
        vec![0]
    }
    pub fn nullify(key: K) {}
}
