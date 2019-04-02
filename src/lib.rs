#![feature(unboxed_closures)]

pub mod latin_1;
pub mod character;

pub use latin_1::{BMLatin1, BMLatin1Searchable, BMLatin1BadCharShiftMap, BMLatin1BadCharShiftMapRev};
pub use character::{BMCharacter, BMCharacterSearchable, BMCharacterBadCharShiftMap, BMCharacterBadCharShiftMapRev};