# test_gen

[![crate](https://img.shields.io/crates/v/typed_test_gen.svg)](https://crates.io/crates/typed_test_gen)
[![documentation](https://docs.rs/typed_test_gen/badge.svg)](https://docs.rs/typed_test_gen)

Rust crate for defining a macro that automatically specializes generic tests on provided types. This is useful
when we want to write identical tests with different underlying types. This crate provide the syntax `#[test_with(Type1, Type2)]`
which will instantiate 2 tests: one on `Type1`, the other on `Type2`. Those types could be any path to any generic type like 
`module::something::Type<Gen>`.


An example is better than words:
```rust
#[test_with(u32, u64, char)]
fn test_vec<T>() {
    let vec = Vec::<T>::with_capacity(10);
    assert_eq!(vec.len(), 0);
    assert!(vec.capacity() >= 10);
}
```
This code will generate 3 tests function: `_specialized__test_vec__u32_`, `_specialized__test_vec__u64_`, and `_specialized__test_vec__char_`.


This support adding the attribute `#[should_panic]` to the definition.
```rust
#[test_with(u32, u64, char)]
#[should_panic]
fn test_vec_fail<T>() {
    let vec = Vec::<T>::with_capacity(10);
    assert_eq!(vec.len(), 0);
    assert!(vec.capacity() < 10);
}
```
