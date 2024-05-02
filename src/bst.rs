use crate::byte_view::ByteView;
use std::fmt::Debug;
use std::mem::size_of;
use std::ops::{BitAnd, Shr};

macro_rules! pass_down_binary_search {
    ($low: tt, $high: tt, $for_highest: tt, $size: tt, $zero: tt, $t2: tt, $x: tt, $pass: tt) => {
        if ($for_highest || $low == $zero) || $high != $zero {
            if $pass {
                return 0;
            }
            binary_search::<$t2>($high, $for_highest, $size / 2)
        } else {
            if $pass {
                return 1;
            }
            $x + binary_search::<$t2>($low, $for_highest, $size / 2)
        }
    };
}

pub fn binary_search<T>(word: T, for_highest: bool, size: u8) -> i32
where
    T: ByteView + Shr + BitAnd + Eq + PartialEq + Copy + Debug,
    T: From<<T as Shr>::Output>,
    T: From<<T as BitAnd>::Output>,
    T: Into<u8> + Into<u16> + Into<u32> + Into<u64> + Into<u128>,
    T: From<u8> + TryFrom<usize> + TryFrom<u16> + TryFrom<u32> + TryFrom<u64> + TryFrom<u128>,
    <T as TryFrom<usize>>::Error: Debug,
{
    match size {
        16 => {
            //
            let low: u8 = T::into(T::from(word & T::from(u8::MAX)));
            let shr = T::try_from(size_of::<u8>() * 8).unwrap();
            let high: u8 = T::into(T::from(T::from(word >> shr) & T::from(u8::MAX)));
            let zero = u8::from(0);
            let x = i32::from(i32::try_from(size_of::<u8>()).unwrap() * 8);
            pass_down_binary_search!(low, high, for_highest, size, zero, u8, x, false)
        }
        2 | 4 | 8 => {
            let rhs = if size > 4 { 0x0f } else { size - 1 };
            let high = T::from(T::from(word >> T::from(size / 2)) & T::from(rhs));
            let low = T::from(word & T::from(size - 1));
            let zero = T::from(0);
            let condition = size == 2;
            let x = i32::from(size / 2);
            pass_down_binary_search!(low, high, for_highest, size, zero, T, x, condition)
        }
        _ => 0,
    }
}
