use std::{ops::{Range, BitAndAssign, BitXorAssign, Add, Shl}};

use num::PrimInt;

fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

pub trait BitIndexer<T>
{
    fn value(&self) -> T;
    fn get(&self, index: usize) -> bool;
    fn first(&self, size: usize) -> T;
    fn last(&self, size: usize) -> T;
    fn get_range(&self, range: Range<usize>) -> T;
}

pub trait BitIndexerMut<T>
{
    fn clear(&mut self, index: usize);
    fn toggle(&mut self, index: usize);
    fn set(&mut self, index: usize, value: usize);
    fn set_range(&mut self, range: Range<usize>, value: T);
}

pub struct BitIndexerSt<T>
{
    value: T
}

pub struct BitIndexerMutSt<'a, T>
{
    value: &'a mut T
}

impl<'a, T> BitIndexerMutSt<'a, T>
{
    // Avoid exposing this
    fn set_value(&mut self, value: T)
    {
        *self.value = value;
    }
}

impl<T> BitIndexer<T> for BitIndexerSt<T> where T: Add + Sized + PrimInt + BitAndAssign + BitXorAssign + Shl
{
    fn value(&self) -> T {
        self.value
    }

    fn get(&self, index: usize) -> bool {
        (self.value() >> index) & T::one() == T::one()
    }

    fn first(&self, size: usize) -> T {
        if size == 0 { return T::zero(); }
        let zero_bits = num_bits::<T>() - size;
        let f0 = self.value() << zero_bits;
        let f1 = f0 >> zero_bits;
        return f1;
    }

    fn last(&self, size: usize) -> T {
        if size == 0 { return T::zero(); }
        let shift_off = num_bits::<T>() - size;
        let val = self.value() >> shift_off;
        val
    }

    fn get_range(&self, range: Range<usize>) -> T {
        let size = range.end - range.start;
        if size == num_bits::<T>() {
            return self.value();
        }
        if range.start == range.end {
            return T::zero();
        }
        let index = range.start;
        let mask = bit_mask_with::<T>(range.start .. range.end);
        (self.value() & mask) >> index
    }
}

impl<'a, T> BitIndexer<T> for BitIndexerMutSt<'a, T> where T: Add + Sized + PrimInt + BitAndAssign + BitXorAssign + Shl
{
    fn value(&self) -> T {
        return *self.value;
    }

    fn get(&self, index: usize) -> bool {
        (self.value() >> index) & T::one() == T::one()
    }

    fn first(&self, size: usize) -> T {
        if size == 0 { return T::zero(); }
        let zero_bits = num_bits::<T>() - size;
        let f0 = self.value() << zero_bits;
        let f1 = f0 >> zero_bits;
        return f1;
    }

    fn last(&self, size: usize) -> T {
        if size == 0 { return T::zero(); }
        let shift_off = num_bits::<T>() - size;
        let val = self.value() >> shift_off;
        val
    }

    fn get_range(&self, range: Range<usize>) -> T {
        let size = range.end - range.start;
        if size == num_bits::<T>() {
            return self.value();
        }
        if range.start == range.end {
            return T::zero();
        }
        let index = range.start;
        let mask = bit_mask_with::<T>(range.start .. range.end);
        (self.value() & mask) >> index
    }
}

impl<'a, T> BitIndexerMut<T> for BitIndexerMutSt<'a, T> where T: Add + Sized + PrimInt + BitAndAssign + BitXorAssign + Shl
{
    fn clear(&mut self, index: usize) {
        self.set_value(self.value() & !(T::one() << index));
    }

    fn toggle(&mut self, index: usize) {
        self.set_value(self.value() ^ T::one() << index);
    }

    fn set(&mut self, index: usize, value: usize) {
        self.set_value((self.value() & !(T::one() << index)) | T::from(value << index).unwrap());
    }

    fn set_range(&mut self, range: Range<usize>, value: T) {
        if range.start == range.end {
            return;
        }
        let size = range.end - range.start;
        if size == num_bits::<T>() {
            self.set_value(value);
            return;
        }
        let index = range.start;
        let left_side = self.last(num_bits::<T>() - (index + size));
        let right_side = self.first(index);
        let left_shift = left_side << (index + size);
        let value_shift = value << index; // shift it to where it needs to be
        self.set_value(left_shift | right_side | value_shift);
    }
}

pub trait BitManipulator<T>
{
    fn bits(&self) -> BitIndexerSt<T>;
    fn bits_mut(&mut self) -> BitIndexerMutSt<T>;
}

fn bit_mask_with<T>(range: Range<usize>) -> T where T: PrimInt + Shl {
    let size = range.end - range.start;
    let m0 = (T::one() << size) - T::one();
    let m1 = m0 << range.start;
    m1
}

impl<T> BitManipulator<T> for T where T: Add + Sized + PrimInt + BitAndAssign + BitXorAssign
{
    fn bits(&self) -> BitIndexerSt<T> {
        BitIndexerSt { value: *self }
    }

    fn bits_mut(&mut self) -> BitIndexerMutSt<T> {
        BitIndexerMutSt { value: self }
    }
}

#[cfg(test)]
mod test
{
    use std::ops::Range;

    use crate::{BitManipulator, BitIndexerMut, BitIndexer};

    #[test]
    fn test_supported_types() {
        let mut test0: i8 = 0;
        test0.bits_mut().set_range(Range { start: 0, end: 4 }, 0xf);
        assert_eq!(test0, 0xf);
    }

    #[test]
    fn test_basic_operations() {
        let mut test0: u32 = 0xfcffff60;
        test0.bits_mut().set(5, 0);
        assert_eq!(test0, 0xfcffff40);
        test0.bits_mut().toggle(5);
        assert_eq!(test0, 0xfcffff60);
        test0.bits_mut().clear(5);
        assert_eq!(test0, 0xfcffff40);
        test0.bits_mut().set(5, 1);
        assert_eq!(test0, 0xfcffff60);

        let mut test1: u32 = 0;
        test1.bits_mut().set_range(0..32, 123);
        assert_eq!(test1, 123);
    }

    #[test]
    fn test_bit_selector() {
        let test0: u32 = 0x55456F4;
        let test1: u32 = 0xF0000000;
        let test2: u32 = 0xfcffff60;
        let res1 = test1.bits().get_range(28..32);
        assert_eq!(res1, 0xf);
        let val0 = test0.bits().get_range(4..8);
        let val1 = test0.bits().get_range(4..7);
        assert_eq!(val0, 0b1111);
        assert_eq!(val1, 0b111);
        assert_eq!(test2.bits().first(8), 0b01100000);
        assert_eq!(test2.bits().last(8), 0b11111100);
    }

    #[test]
    fn test_bit_setter() {
        let mut test0: u32 = 0;
        test0.bits_mut().set_range(4..8, 0b1111);
        assert_eq!(test0, 0xf0);
        test0.bits_mut().set_range(24..28, 0b1011);
        assert_eq!(test0, 0xb0000f0);
        let mut test1 = u32::max_value(); // 0xffffffff
        test1.bits_mut().set_range(0..8, 0);
        assert_eq!(test1, 0xffffff00);
    }
}