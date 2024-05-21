use crate::byte_view::{shrink_back, ByteView};
use crate::IndexPointer;
use std::fmt::Debug;
use std::mem::size_of;
use std::ops::{BitAnd, Shr};
use std::sync::Arc;

//TODO: redo byteLen parts

#[derive(Debug, Clone)]
pub struct BST<K> {
    ptr: IndexPointer,
    _val: K,
}

pub fn maskLowerThan(v: Arc<Vec<u8>>, position: u8) -> Arc<Vec<u8>> {
    let mut ar: [u64; 4] = [0, 0, 0, 0];
    ar[0] = u64::from_le_bytes(v.clone().as_slice()[0..8].try_into().unwrap());
    ar[1] = u64::from_le_bytes(v.clone().as_slice()[8..16].try_into().unwrap());
    ar[2] = u64::from_le_bytes(v.clone().as_slice()[16..24].try_into().unwrap());
    ar[3] = u64::from_le_bytes(v.clone().as_slice()[24..32].try_into().unwrap());

    let bit_selected = position % 64;
    let byte_selected = position / 64;

    ar[byte_selected as usize] = ar[byte_selected as usize]
        & u64::try_from(((1 << bit_selected) - 1) << (64 - bit_selected)).unwrap();

    for i in (byte_selected + 1)..4 {
        ar[i as usize] = 0;
    }

    let _vec = ar
        .into_iter()
        .map(|v| v.to_le_bytes().to_vec())
        .flatten()
        .collect::<Vec<u8>>();
    Arc::from(_vec)
}

pub fn maskGreaterThan(v: Arc<Vec<u8>>, position: u8) -> Arc<Vec<u8>> {
    let mut ar: [u64; 4] = [0, 0, 0, 0];
    ar[0] = u64::from_le_bytes(v.clone().as_slice()[0..8].try_into().unwrap());
    ar[1] = u64::from_le_bytes(v.clone().as_slice()[8..16].try_into().unwrap());
    ar[2] = u64::from_le_bytes(v.clone().as_slice()[16..24].try_into().unwrap());
    ar[3] = u64::from_le_bytes(v.clone().as_slice()[24..32].try_into().unwrap());

    let bit_selected = position % 64;
    let byte_selected = position / 64;

    ar[byte_selected as usize] = ar[byte_selected as usize]
        & u64::try_from(((1 << (bit_selected + 1)) - 1) << (63 - bit_selected)).unwrap();

    for i in 0..byte_selected {
        ar[i as usize] = 0;
    }

    let _vec = ar
        .into_iter()
        .map(|v| v.to_le_bytes().to_vec())
        .flatten()
        .collect::<Vec<u8>>();
    Arc::from(_vec)
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

pub fn set_bit_u256(mask: Arc<Vec<u8>>, position: usize) -> Arc<Vec<u8>> {
    let byte_position = position / 8;
    let bit_position = u8::try_from(position % 8).unwrap();
    let mut vec_mask: Vec<u8> = Arc::try_unwrap(mask.clone()).unwrap();

    let new_bit = u8::from(1) << (u8::from(7) - bit_position);

    vec_mask[byte_position] = vec_mask[byte_position] | new_bit;

    Arc::from(vec_mask)
}

pub fn unset_bit_u256(mask: Arc<Vec<u8>>, position: usize) -> Arc<Vec<u8>> {
    let byte_position = position / 8;
    let bit_position = u8::try_from(position % 8).unwrap();
    let mut vec_mask: Vec<u8> = Arc::try_unwrap(mask.clone()).unwrap();

    let new_bit = !(u8::from(1) << (u8::from(7) - bit_position));

    vec_mask[byte_position] = vec_mask[byte_position] & new_bit;

    Arc::from(vec_mask)
}

pub fn is_set_u256(mask: Arc<Vec<u8>>, position: usize) -> bool {
    let byte_position: usize = position / 8;
    let bit_position: u8 = u8::try_from(position % 8).unwrap();

    let vec_mask = Arc::try_unwrap(mask).unwrap();

    let set_bit = u8::from(1) << (u8::from(7) - bit_position);

    vec_mask[byte_position] == vec_mask[byte_position] & set_bit
}

pub fn binary_search(high: u128, low: u128, for_highest: bool) -> i32 {
    let x = i32::try_from(size_of::<u128>() * 8).unwrap();
    let zero = u128::try_from(0).unwrap();
    pass_down_binary_search!(high, low, for_highest, 256, zero, u128, x, false)
}

macro_rules! pass_down_bst {
    ($high: tt, $low: tt, $for_highest: tt, $next: tt, $shift: tt) => {
        if ($for_highest || $low == 0) && $high != 0 {
            binary_search_alt::<$next>(ByteView::to_bytes($high), $for_highest)
        } else {
            $shift + binary_search_alt::<$next>(ByteView::to_bytes($low), $for_highest)
        }
    };
}

pub fn binary_search_alt<const N: usize>(_word: Vec<u8>, for_highest: bool) -> i32 {
    match N {
        128 => {
            let word: u128 = ByteView::from_bytes(_word);
            0
        }
        64 => {
            let word: u64 = ByteView::from_bytes(_word);
            0
        }
        32 => {
            let word: u32 = ByteView::from_bytes(_word);
            0
        }
        16 => {
            let word: u16 = ByteView::from_bytes(_word);
            0
        }
        8 => {
            let word: u8 = ByteView::from_bytes(_word);
            let max: u8 = 0x0f;
            0
        }
        4 => {
            let max: u8 = u8::try_from(N - 1).unwrap();
            let word: u8 = ByteView::from_bytes(_word);
            let high = (word >> 1) & max;
            let low = word & max;
            pass_down_bst!(high, low, for_highest, 2, 2)
        }
        2 => {
            let max: u8 = u8::try_from(N - 1).unwrap();
            let word: u8 = ByteView::from_bytes(_word);
            let high = (word >> 1) & max;
            let low = word & max;
            if (for_highest || low == 0) && high != 0 {
                0
            } else {
                1
            }
        }
        _ => 0,
    }
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

impl<K: ByteView + Sized + TryFrom<usize>> BST<K>
where
    <K as TryFrom<usize>>::Error: Debug,
{
    pub fn new(ptr: IndexPointer) -> Self {
        BST {
            ptr,
            _val: K::from_bytes(vec![0]),
        }
    }
    pub fn get_mask_pointer(&self, partial_key: Vec<u8>) -> IndexPointer {
        self.ptr.select(&partial_key).keyword("/mask")
    }
    pub fn mark_path(&self, key: K) {
        let key_bytes = ByteView::to_bytes(key);
        for i in 0..size_of::<K>() {
            let partial_key = ByteView::to_bytes(K::try_from(i).unwrap());
            let ptr = self.get_mask_pointer(partial_key);
            let mask = ptr.get();
            let byte = key_bytes[i];
            if !is_set_u256(mask.clone(), byte as usize) {
                ptr.set(set_bit_u256(mask, byte as usize));
            }
        }
    }
    pub fn unmark_path(&self, key: K) {
        let key_bytes = ByteView::to_bytes(key);
        for i in size_of::<K>() as i32..0 as i32 {
            let partial_key = ByteView::to_bytes(i as usize);
            let ptr = self.get_mask_pointer(partial_key);
            let mask = ptr.get();
        }
    }
    fn _find_boundary_from_partial(&self, key_bytes: Vec<u8>, seek_higher: bool) -> K {
        K::from_bytes(vec![0])
    }
    pub fn seek_lower(&self, start: K) -> K {
        let mut partial_key = ByteView::to_bytes(start);
        loop {
            let this_key = shrink_back(partial_key.clone(), 1);
            let mask = self.get_mask_pointer(this_key.clone()).get();
            if mask.len() > 0 {
                let derived_mask = maskLowerThan(mask, partial_key[this_key.len()]);
                self.get_mask_pointer(this_key.clone())
                    .set(derived_mask.clone());
                let derived_mask_value = Arc::try_unwrap(derived_mask).unwrap();
                let high = u128::from_le_bytes(
                    derived_mask_value.clone().as_slice()[0..16]
                        .try_into()
                        .unwrap(),
                );
                let low = u128::from_le_bytes(
                    derived_mask_value.clone().as_slice()[16..32]
                        .try_into()
                        .unwrap(),
                );
                let symbol = binary_search(high, low, false);
                if symbol != -1 {
                    let mut new_key = this_key.clone();
                    new_key.push(symbol.try_into().unwrap());
                    return self._find_boundary_from_partial(new_key, false);
                }
            }

            partial_key = this_key;
            if partial_key.len() == 0 {
                break;
            }
        }
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
