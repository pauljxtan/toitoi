//! A riichi mahjong library.
//!
//! Principle: keep the public API surface as small as possible for relevant use cases.

pub mod score;
pub mod tile;
pub mod types;

mod calculation;
mod division;
mod fu;
mod meld;
mod utils;
mod yaku;
