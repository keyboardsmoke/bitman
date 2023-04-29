use std::ops::Range;

use num_traits::PrimInt;

fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

pub trait BitSetIndex<T>
{
    fn set_bit(&mut self, index: usize, value: T);
    fn clear_bit(&mut self, index: usize);
    fn toggle_bit(&mut self, index: usize);
}

pub trait BitSetRange<T>
{
    fn set(&mut self, range: Range<usize>, value: T);
    fn transform(&mut self, range: Range<usize>, value: T, f: fn(a: T, b: T) -> T);
    fn add(&mut self, range: Range<usize>, value: T);
    fn sub(&mut self, range: Range<usize>, value: T);
    fn mul(&mut self, range: Range<usize>, value: T);
    fn div(&mut self, range: Range<usize>, value: T);
    fn and(&mut self, range: Range<usize>, value: T);
    fn or(&mut self, range: Range<usize>, value: T);
    fn not(&mut self, range: Range<usize>);
    fn xor(&mut self, range: Range<usize>, value: T);
    fn lsh(&mut self, range: Range<usize>, value: T);
    fn rsh(&mut self, range: Range<usize>, value: T);
}

pub trait BitGetRange<T>
{
    // first N bits
    fn first(&self, size: usize) -> T;

    // last N bits
    fn last(&self, size: usize) -> T;

    // Get N bits
    fn get(&self, range: Range<usize>) -> T;

    // Compare N bits to match bits, except where wildcard bits are set to 1.
    fn compare(&self, range: Range<usize>, match_bits: T, wildcard_bits: T) -> bool;
}

pub trait BitGetIndex<T>
{
    fn get_bit(&self, index: usize) -> bool;
}

pub struct BitManipulatorImpl<T>
{
    pub(crate) value: T
}

pub struct BitManipulatorImplMut<'a, T>
{
    pub(crate) value: &'a mut T
}

pub trait BitManipulator<T>
{
    fn bits(&self) -> BitManipulatorImpl<T>;
    fn bits_mut(&mut self) -> BitManipulatorImplMut<T>;
}

impl<'a, T> BitSetIndex<T> for BitManipulatorImplMut<'a, T> where T: PrimInt
{
    fn set_bit(&mut self, index: usize, value: T) {
        let res = (*self.value & !(T::one() << index)) | T::from(value << index).unwrap();
        *self.value = res;
    }

    fn clear_bit(&mut self, index: usize) {
        *self.value = *self.value & !(T::one() << index);
    }

    fn toggle_bit(&mut self, index: usize) {
        *self.value = *self.value ^ (T::one() << index);
    }
}

impl<T> BitGetIndex<T> for BitManipulatorImpl<T> where T: PrimInt 
{
    fn get_bit(&self, index: usize) -> bool {
        (self.value >> index) & T::one() == T::one()
    }
}

impl<T> BitGetRange<T> for BitManipulatorImpl<T> where T: PrimInt 
{
    fn first(&self, size: usize) -> T {
        if size == 0 { return T::zero(); }
        let zero_bits = num_bits::<T>() - size;
        let f0 = self.value << zero_bits;
        let f1 = f0 >> zero_bits;
        return f1;
    }
    
    fn last(&self, size: usize) -> T {
        if size == 0 { return T::zero(); }
        let shift_off = num_bits::<T>() - size;
        let val = self.value >> shift_off;
        val
    }

    fn get(&self, range: Range<usize>) -> T {
        let bit_width = range.end - range.start;
        let bit_offset = range.start;
		let mut val: T = T::zero();
		for i in 0..(bit_width as usize) {
			if self.get_bit(i + bit_offset) {
				let index = if cfg!(target_endian = "big") {
					bit_width as usize - 1 - i
				} else {
					i
				};
                val = val | T::one() << index;
			}
		}
		val
    }

    fn compare(&self, range: Range<usize>, match_bits: T, wildcard_bits: T) -> bool {
        let value = self.get(range);
        let a = value ^ match_bits; // Any non-zero bit is a non-match
        wildcard_bits & a == a
    }
}

impl<'a, T> BitGetRange<T> for BitManipulatorImplMut<'a, T> where T: PrimInt 
{
    fn first(&self, size: usize) -> T {
        if size == 0 { return T::zero(); }
        let zero_bits = num_bits::<T>() - size;
        let f0 = *self.value << zero_bits;
        let f1 = f0 >> zero_bits;
        return f1;
    }
    
    fn last(&self, size: usize) -> T {
        if size == 0 { return T::zero(); }
        let shift_off = num_bits::<T>() - size;
        let val = *self.value >> shift_off;
        val
    }

    fn get(&self, range: Range<usize>) -> T {
        let bit_width = range.end - range.start;
        let bit_offset = range.start;
		let mut val: T = T::zero();
		for i in 0..(bit_width as usize) {
			if self.get_bit(i + bit_offset) {
				let index = if cfg!(target_endian = "big") {
					bit_width as usize - 1 - i
				} else {
					i
				};
                val = val | T::one() << index;
			}
		}
		val
    }

    fn compare(&self, range: Range<usize>, match_bits: T, wildcard_bits: T) -> bool {
        let value = self.get(range);
        let a = value ^ match_bits; // Any non-zero bit is a non-match
        wildcard_bits & a == a
    }
}

impl<'a, T> BitGetIndex<T> for BitManipulatorImplMut<'a, T> where T: PrimInt 
{
    fn get_bit(&self, index: usize) -> bool {
        (*self.value >> index) & T::one() == T::one()
    }
}

impl<'a, T> BitSetRange<T> for BitManipulatorImplMut<'a, T> where T: PrimInt
{
    fn set(&mut self, range: Range<usize>, value: T) {
        if range.start == range.end {
            return;
        }
        let size = range.end - range.start;
        if size == num_bits::<T>() {
            *self.value = value;
            return;
        }
        let index = range.start;
        let left_side = self.last(num_bits::<T>() - (index + size));
        let right_side = self.first(index);
        let left_shift = left_side << (index + size);
        let value_shift = value << index; // shift it to where it needs to be
        *self.value = left_shift | right_side | value_shift;
    }

    fn transform(&mut self, range: Range<usize>, value: T, f: fn(a: T, b: T) -> T) {
        let size = range.end - range.start;
        let v = self.get(range.clone());
        self.set(range, f(v, value).bits().get(0..size));
    }

    fn add(&mut self, range: Range<usize>, value: T) {
        self.transform(range, value, |a, b| a + b);
    }

    fn sub(&mut self, range: Range<usize>, value: T) {
        self.transform(range, value, |a, b| a - b);
    }

    fn mul(&mut self, range: Range<usize>, value: T) {
        self.transform(range, value, |a, b| a * b);
    }

    fn div(&mut self, range: Range<usize>, value: T) {
        self.transform(range, value, |a, b| a / b);
    }

    fn and(&mut self, range: Range<usize>, value: T) {
        self.transform(range, value, |a, b| a & b);
    }

    fn or(&mut self, range: Range<usize>, value: T) {
        self.transform(range, value, |a, b| a | b);
    }

    fn not(&mut self, range: Range<usize>) {
        let v = self.get(range.clone());
        let size = range.end - range.start;
        self.set(range.clone(), v.not().bits().get(0..size));
    }

    fn xor(&mut self, range: Range<usize>, value: T) {
        self.transform(range, value, |a, b| a ^ b);
    }

    fn lsh(&mut self, range: Range<usize>, value: T) {
        self.transform(range, value, |a, b| a << b.to_usize().unwrap());
    }

    fn rsh(&mut self, range: Range<usize>, value: T) {
        self.transform(range, value, |a, b| a >> b.to_usize().unwrap());
    }
}

impl<T> BitManipulator<T> for T where T: Sized + Copy 
{
    fn bits(&self) -> BitManipulatorImpl<T> {
        BitManipulatorImpl { value: *self }
    }

    fn bits_mut(&mut self) -> BitManipulatorImplMut<T> 
    {
        BitManipulatorImplMut { value: self }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_supported_types() {
        let mut test0: i8 = 0;
        test0.bits_mut().set(Range { start: 0, end: 4 }, 0xf);
        assert_eq!(test0, 0xf);
    }

    #[test]
    fn test_basic_operations() {
        let mut test0: u32 = 0xfcffff60;
        test0.bits_mut().set_bit(5, 0);
        assert_eq!(test0, 0xfcffff40);
        test0.bits_mut().toggle_bit(5);
        assert_eq!(test0, 0xfcffff60);
        test0.bits_mut().clear_bit(5);
        assert_eq!(test0, 0xfcffff40);
        test0.bits_mut().set_bit(5, 1);
        assert_eq!(test0, 0xfcffff60);

        let mut test1: u32 = 0;
        test1.bits_mut().set(0..32, 123);
        assert_eq!(test1, 123);
    }

    #[test]
    fn test_bit_selector() {
        let test0: u32 = 0x55456F4;
        let test1: u32 = 0xF0000000;
        let test2: u32 = 0xfcffff60;
        let res1 = test1.bits().get(28..32);
        assert_eq!(res1, 0xf);
        let val0 = test0.bits().get(4..8);
        let val1 = test0.bits().get(4..7);
        assert_eq!(val0, 0b1111);
        assert_eq!(val1, 0b111);
        assert_eq!(test2.bits().first(8), 0b01100000);
        assert_eq!(test2.bits().last(8), 0b11111100);
    }

    #[test]
    fn test_bit_setter() {
        let mut test0: u32 = 0;
        test0.bits_mut().set(4..8, 0b1111);
        assert_eq!(test0, 0xf0);
        test0.bits_mut().set(24..28, 0b1011);
        assert_eq!(test0, 0xb0000f0);
        let mut test1 = u32::max_value(); // 0xffffffff
        test1.bits_mut().set(0..8, 0);
        assert_eq!(test1, 0xffffff00);
    }
    
    #[test]
    fn test_match_exact() {
        let t0: u32 = 0b00; // 0
        let t1: u32 = 0b01; // 1
        let t2: u32 = 0b10; // 2
        let t3: u32 = 0b11; // 3

        assert!(t0.bits().compare(0..2, 0b00, 0b10));
        assert!(t1.bits().compare(0..2, 0b00, 0b10) == false);
        assert!(t2.bits().compare(0..2, 0b00, 0b10));
        assert!(t3.bits().compare(0..2, 0b00, 0b11));
    }

    #[test]
    fn run_ops() {
        let mut t0 = 0x5DBD2565u32;
        t0.bits_mut().xor(20..24, 0b1011);
        assert_eq!(t0, 0x5D0D2565u32);
        t0.bits_mut().set(20..24, 0b1011);
        assert_eq!(t0, 0x5DBD2565u32);
        t0.bits_mut().or(20..24, 0b1111);
        assert_eq!(t0, 0x5DFD2565u32);
        t0.bits_mut().div(20..24, 0b1010); // 0b1101 (11) / 0b1010 (10) = 0b1
        assert_eq!(t0, 0x5D1D2565u32);
        t0.bits_mut().not(20..24);
        assert_eq!(t0, 0x5DED2565);
    }
}