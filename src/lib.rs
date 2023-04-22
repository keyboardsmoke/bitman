use std::ops::{Range, BitAndAssign, BitXorAssign, Add};

use num::PrimInt;

fn bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

pub trait BitManipulator<T>
{
    fn bit_clear(&mut self, index: usize);
    fn bit_toggle(&mut self, index: usize);
    fn bit_check(&self, index: usize) -> bool;
    fn bit_set(&mut self, index: usize, value: usize);

    fn bits_first(&self, size: usize) -> T;
    fn bits_last(&self, size: usize) -> T;

    fn bits_get(&self, range: Range<usize>) -> T;
    fn bits_set(&mut self, range: Range<usize>, value: T);
}

fn bit_mask(range: Range<usize>) -> usize {
    let size = range.end - range.start;
    let m0: usize = (1 << size) - 1;
    let m1: usize = m0 << range.start;
    m1
}

impl<T> BitManipulator<T> for T where T: Add + Sized + PrimInt + BitAndAssign + BitXorAssign
{
    fn bit_clear(&mut self, index: usize) {
        *self &= !(Self::one() << index);
    }

    fn bit_toggle(&mut self, index: usize) {
        *self ^= Self::one() << index;
    }

    fn bit_check(&self, index: usize) -> bool {
        (*self >> index) & Self::one() == Self::one()
    }

    fn bit_set(&mut self, index: usize, value: usize) {
        *self = (*self & !(Self::one() << index)) | T::from(value << index).unwrap();
    }

    fn bits_first(&self, size: usize) -> T {
        if size == 0 { return Self::zero(); }
        let zero_bits = bits::<T>() - size;
        let f0 = *self << zero_bits;
        let f1 = f0 >> zero_bits;
        return f1;
    }

    fn bits_last(&self, size: usize) -> T {
        if size == 0 { return Self::zero(); }
        let shift_off = bits::<T>() - size;
        let val = *self >> shift_off;
        val
    }

    fn bits_get(&self, range: Range<usize>) -> T {
        let size = range.end - range.start;
        if size == bits::<T>() {
            return *self;
        }
        if range.start == range.end {
            return Self::zero();
        }
        let index = range.start;
        let mask = bit_mask(Range { start: range.start as usize, end: range.end as usize });
        (*self & T::from(mask).unwrap()) >> index
    }

    fn bits_set(&mut self, range: Range<usize>, value: T) {
        if range.start == range.end {
            return;
        }
        let size = range.end - range.start;
        if size == bits::<T>() {
            *self = value;
            return;
        }
        let index = range.start;
        let left_side = self.bits_last(bits::<T>() - (index + size));
        let right_side = self.bits_first(index);
        let left_shift = left_side << (index + size);
        let value_shift = value << index; // shift it to where it needs to be
        *self = left_shift | right_side | value_shift;
    }
}

#[cfg(test)]
mod test
{
    use std::ops::Range;

    use crate::{BitManipulator};

    #[test]
    fn test_supported_types() {
        let mut test0: i8 = 0;
        test0.bits_set(Range { start: 0, end: 4 }, 0xf);
        assert_eq!(test0, 0xf);
    }

    #[test]
    fn test_basic_operations() {
        let mut test0: u32 = 0xfcffff60;
        test0.bit_set(5, 0);
        assert_eq!(test0, 0xfcffff40);
        test0.bit_toggle(5);
        assert_eq!(test0, 0xfcffff60);
        test0.bit_clear(5);
        assert_eq!(test0, 0xfcffff40);
        test0.bit_set(5, 1);
        assert_eq!(test0, 0xfcffff60);

        let mut test1: u32 = 0;
        test1.bits_set(Range { start: 0, end: 32 }, 123);
        assert_eq!(test1, 123);
    }

    #[test]
    fn test_bit_selector() {
        let test0: u32 = 0x55456F4;
        let test1: u32 = 0xF0000000;
        let test2: u32 = 0xfcffff60;
        let res1 = test1.bits_get(Range { start: 28, end: 32 });
        assert_eq!(res1, 0xf);
        let val0 = test0.bits_get(Range { start: 4, end: 8 });
        let val1 = test0.bits_get(Range { start: 4, end: 7 });
        assert_eq!(val0, 0b1111);
        assert_eq!(val1, 0b111);
        assert_eq!(test2.bits_first(8), 0b01100000);
        assert_eq!(test2.bits_last(8), 0b11111100);
    }

    #[test]
    fn test_bit_setter() {
        let mut test0: u32 = 0;
        test0.bits_set(Range { start: 4, end: 8 }, 0b1111);
        assert_eq!(test0, 0xf0);
        test0.bits_set(Range { start: 24, end: 28 }, 0b1011);
        assert_eq!(test0, 0xb0000f0);
        let mut test1 = u32::max_value(); // 0xffffffff
        test1.bits_set(Range { start: 0, end: 8 }, 0);
        assert_eq!(test1, 0xffffff00);
    }
}