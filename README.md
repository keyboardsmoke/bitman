# bitman

Bit manipulation for primitive integer types in rust

![badge](https://github.com/keyboardsmoke/bitman/actions/workflows/rust.yml/badge.svg)

```rust
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
```
