# Primitive promotions for primitive numeric types

According to Rust's reference, [primitive numeric types][primitive numeric type] in Rust are such:

# Numeric types

## Integer types

The unsigned integer types consist of:

Type   | Minimum | Maximum
-------|---------|-------------------
`u8`   | 0       | 2<sup>8</sup>-1
`u16`  | 0       | 2<sup>16</sup>-1
`u32`  | 0       | 2<sup>32</sup>-1
`u64`  | 0       | 2<sup>64</sup>-1
`u128` | 0       | 2<sup>128</sup>-1

The signed two's complement integer types consist of:

Type   | Minimum            | Maximum
-------|--------------------|-------------------
`i8`   | -(2<sup>7</sup>)   | 2<sup>7</sup>-1
`i16`  | -(2<sup>15</sup>)  | 2<sup>15</sup>-1
`i32`  | -(2<sup>31</sup>)  | 2<sup>31</sup>-1
`i64`  | -(2<sup>63</sup>)  | 2<sup>63</sup>-1
`i128` | -(2<sup>127</sup>) | 2<sup>127</sup>-1


## Floating-point types

The IEEE 754-2008 "binary32" and "binary64" floating-point types are `f32` and
`f64`, respectively.

## Machine-dependent integer types

The `usize` type is an unsigned integer type with the same number of bits as the
platform's pointer type. It can represent every memory address in the process.

The `isize` type is a signed integer type with the same number of bits as the
platform's pointer type. The theoretical upper bound on object and array size
is the maximum `isize` value. This ensures that `isize` can be used to calculate
differences between pointers into an object or array and can address every byte
within an object along with one byte past the end.

`usize` and `isize` are at least 16-bits wide.

> **Note**: Many pieces of Rust code may assume that pointers, `usize`, and
> `isize` are either 32-bit or 64-bit. As a consequence, 16-bit
> pointer support is limited and may require explicit care and acknowledgment
> from a library to support.

# Why this trait is needed

All [primitive numeric types][primitive numeric type], including machine-dependent types, come with known size that can be obtained via [`core::mem::size_of<T>()`][core::mem::size_of]. The greater the size is, the greater the number of possible values that can be represented by the type. Integer intervals as sets are not [closed](https://en.wikipedia.org/wiki/Closure_(mathematics)) under many operations, notably addition and multiplication. Since `u8` represents the integer interval \[0..2<sup>8</sup>-1\], the same holds for this type. By analogy, the same is true for `u16`, `u32`, etc. Similarly, the set of values representable by floating point numbers with algebraic structure avoiding imprecision (i.e. distinct from the algebraic structure on floating point numbers) is not closed under many operations as well.

One way to circumvent the problem is to use [type promotion]. [Type promotion][type promotion] allows to use a type representing a superset of the original type. For every [primitive numeric type] (except for `u128`, `i128`, and `f64`) there is a canonical [type promotion]. For `u8` the canonical [type promotion] is `u16`, for `i16` the canonical type promotion is `i32`, and so on. Theoretically, one could go one step further and define `u256`, yet it would not be primitive and even simple operations on that type (such as addition) would not have the corresponding CPU instructions.

**Note** Strictly speaking, `u128` and `i128` are poorly supported on current architectures and it may or may not be reasonable to **use** implementation of `PrimitivePromotionExt` extension trait on `u64` and `i64`. However, if you want to use these implementations, `primitive_promotion` crate is what you need because `u64` and `i64` implement the `PrimitivePromotionExt` trait.

# Example

You can notice that `PrimitivePromotionExt` is quite long to type. To make it shorter, you are advised to rename the imported trait as `PP`. Because its uses are meant to be accompanied with [fully qualified syntax](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name), such shorthand is indispensible.

```rust
use primitive_promotion::PrimitivePromotionExt as PP;

fn midpoint(a: &u8, b: &u8) -> u8 {
  // <u8 as TraitName>:: is an example of fully qualified syntax
  let a = *a as <u8 as PP>::PrimitivePromotion;
  let b = *b as <u8 as PP>::PrimitivePromotion;
  ((a+b)/2) as u8
}

fn main() {
  let a: u8 = u8::MAX;
  let b: u8 = u8::MAX;
  assert_eq!(midpoint(&a,&b), u8::MAX);
}
```

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[primitive numeric type]: https://doc.rust-lang.org/reference/types/numeric.html
[core::mem::size_of]: https://doc.rust-lang.org/stable/core/mem/fn.size_of.html
[type promotion]: https://en.wikipedia.org/wiki/Type_conversion#Type_promotion