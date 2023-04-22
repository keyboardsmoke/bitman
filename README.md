# bitman

Bit manipulation for primitive integer types in rust

![badge](https://github.com/keyboardsmoke/bitman/actions/workflows/rust.yml/badge.svg)

```rust
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
```