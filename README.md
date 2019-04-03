Boyer-Moore-MagicLen
====================

[![Build Status](https://travis-ci.org/magiclen/boyer-moore-magiclen.svg?branch=master)](https://travis-ci.org/magiclen/boyer-moore-magiclen)
[![Build status](https://ci.appveyor.com/api/projects/status/wl24no8qec6yogdh/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/boyer-moore-magiclen/branch/master)


This crate can be used to search substrings in a string or search any sub-sequences in any sequence by using boyer-moore-magiclen (which is sometimes faster than boyer-moore and boyer-moore-horspool).

## Usage

For binary data and UTF-8 data, use the `BMByte` struct. For character sequences, use the `BMCharacter` struct (however it much slower than `BMByte`).

Every `BMXXX` has a `from` associated function to create the instance by a search pattern (the needle).

For example,

```rust
extern crate boyer_moore_magiclen;

use boyer_moore_magiclen::BMByte;

let bmb = BMByte::from("oocoo").unwrap();
```

Now, we can search any binary data or UTF-8 data for the pattern `oocoo`.

There are two search modes and two search directions. The first mode is called **full text search**, which finds the positions of the matched sub-sequences including the overlap ones.

```rust
extern crate boyer_moore_magiclen;

use boyer_moore_magiclen::BMByte;

let bmb = BMByte::from("oocoo").unwrap();

assert_eq!(vec![1, 4], bmb.find_full_in("coocoocoocoo", 2));
```

The other mode is called **normal text search**, which finds the positions of the matched sub-sequences excluding the overlap ones.

```rust
extern crate boyer_moore_magiclen;

use boyer_moore_magiclen::BMByte;

let bmb = BMByte::from("oocoo").unwrap();

assert_eq!(vec![1, 7], bmb.find_in("coocoocoocoo", 2));
```

The search direction can be from the head (searching forward, `find_xxx`) or from the tail (searching backward, `rfind_xxx`).

```rust
extern crate boyer_moore_magiclen;

use boyer_moore_magiclen::BMByte;

let bmb = BMByte::from("oocoo").unwrap();

assert_eq!(vec![7, 1], bmb.rfind_in("coocoocoocoo", 2));
```

To search all results at a time, use the `find_all_in`, `rfind_all_in`, `find_full_all_in` or `rfind_full_all_in` method.

```rust
extern crate boyer_moore_magiclen;

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