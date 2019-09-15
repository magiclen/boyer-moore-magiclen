//! # Boyer-Moore-MagicLen
//!
//! This crate can be used to search substrings in a string or search any sub-sequences in any sequence by using boyer-moore-magiclen (which is sometimes faster than boyer-moore and boyer-moore-horspool).
//!
//! ## Usage
//!
//! For binary data and UTF-8 data, use the `BMByte` struct. For character sequences, use the `BMCharacter` struct (however it is much slower than `BMByte`). The `BMCharacter` struct needs the standard library support, and you have to enable the `character` feature to make it available.
//!
//! Every `BMXXX` has a `from` associated function to create the instance by a search pattern (the needle).
//!
//! For example,
//!
//! ```rust
//! extern crate boyer_moore_magiclen;
//!
//! use boyer_moore_magiclen::BMByte;
//!
//! let bmb = BMByte::from("oocoo").unwrap();
//! ```
//!
//! Now, we can search any binary data or UTF-8 data for the pattern `oocoo`.
//!
//! There are two search modes and two search directions. The first mode is called **full text search**, which finds the positions of the matched sub-sequences including the overlapping ones.
//!
//! ```rust
//! extern crate boyer_moore_magiclen;
//!
//! use boyer_moore_magiclen::BMByte;
//!
//! let bmb = BMByte::from("oocoo").unwrap();
//!
//! assert_eq!(vec![1, 4], bmb.find_full_in("coocoocoocoo", 2));
//! ```
//!
//! The other mode is called **normal text search**, which finds the positions of the matched sub-sequences excluding the overlapping ones.
//!
//! ```rust
//! extern crate boyer_moore_magiclen;
//!
//! use boyer_moore_magiclen::BMByte;
//!
//! let bmb = BMByte::from("oocoo").unwrap();
//!
//! assert_eq!(vec![1, 7], bmb.find_in("coocoocoocoo", 2));
//! ```
//!
//! The search direction can be from the head (searching forward, `find_xxx`) or from the tail (searching backward, `rfind_xxx`).
//!
//! ```rust
//! extern crate boyer_moore_magiclen;
//!
//! use boyer_moore_magiclen::BMByte;
//!
//! let bmb = BMByte::from("oocoo").unwrap();
//!
//! assert_eq!(vec![7, 1], bmb.rfind_in("coocoocoocoo", 2));
//! ```
//!
//! To search all results at a time, use the `find_all_in`, `rfind_all_in`, `find_full_all_in` or `rfind_full_all_in` method.
//!
//! ```rust
//! extern crate boyer_moore_magiclen;
//!
//! use boyer_moore_magiclen::BMByte;
//!
//! let bmb = BMByte::from("oocoo").unwrap();
//!
//! assert_eq!(vec![7, 4, 1], bmb.rfind_full_all_in("coocoocoocoo"));
//! ```

#![cfg_attr(not(feature = "character"), no_std)]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate debug_helper;

/// This module helps you search sub-sequences in any byte sequence, including self-synchronizing string encoding data such as UTF-8.
pub mod byte;
#[cfg(feature = "character")]
/// This module helps you search character sub-sequences in any character sequence.
pub mod character;

pub use byte::{BMByte, BMByteBadCharShiftMap, BMByteBadCharShiftMapRev, BMByteSearchable};
#[cfg(feature = "character")]
pub use character::{
    BMCharacter, BMCharacterBadCharShiftMap, BMCharacterBadCharShiftMapRev, BMCharacterSearchable,
};
