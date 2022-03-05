Boyer-Moore-MagicLen
====================

[![CI](https://github.com/magiclen/boyer-moore-magiclen/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/boyer-moore-magiclen/actions/workflows/ci.yml)

This crate can be used to search substrings in a string or search any sub-sequences in any sequence by using boyer-moore-magiclen (which is sometimes faster than boyer-moore and boyer-moore-horspool).

## Usage

For binary data and UTF-8 data, use the `BMByte` struct. For character sequences, use the `BMCharacter` struct (however it is much slower than `BMByte`). The `BMCharacter` struct needs the standard library support, and you have to enable the `character` feature to make it available.

Every `BMXXX` has a `from` associated function to create the instance by a search pattern (the needle).

For example,

```rust
use boyer_moore_magiclen::BMByte;

let bmb = BMByte::from("oocoo").unwrap();
```

Now, we can search any binary data or UTF-8 data for the pattern `oocoo`.

There are two search modes and two search directions. The first mode is called **full text search**, which finds the positions of the matched sub-sequences including the overlapping ones.

```rust
use boyer_moore_magiclen::BMByte;

let bmb = BMByte::from("oocoo").unwrap();

assert_eq!(vec![1, 4], bmb.find_full_in("coocoocoocoo", 2));
```

The other mode is called **normal text search**, which finds the positions of the matched sub-sequences excluding the overlapping ones.

```rust
use boyer_moore_magiclen::BMByte;

let bmb = BMByte::from("oocoo").unwrap();

assert_eq!(vec![1, 7], bmb.find_in("coocoocoocoo", 2));
```

The search direction can be from the head (searching forward, `find_xxx`) or from the tail (searching backward, `rfind_xxx`).

```rust
use boyer_moore_magiclen::BMByte;

let bmb = BMByte::from("oocoo").unwrap();

assert_eq!(vec![7, 1], bmb.rfind_in("coocoocoocoo", 2));
```

To search all results at a time, use the `find_all_in`, `rfind_all_in`, `find_full_all_in` or `rfind_full_all_in` method.

```rust
use boyer_moore_magiclen::BMByte;

let bmb = BMByte::from("oocoo").unwrap();

assert_eq!(vec![7, 4, 1], bmb.rfind_full_all_in("coocoocoocoo"));
```

## Benchmark

```bash
cargo bench --bench full_text_search
```

or

```bash
cargo bench --bench normal_text_search
```

## Crates.io

https://crates.io/crates/boyer-moore-magiclen

## Documentation

https://docs.rs/boyer-moore-magiclen

## License

[MIT](LICENSE)