#![allow(unused)]

mod try_traits;
mod map;

pub use try_traits::*;
pub use map::TryHashMap;

pub type TryReserveError = hashbrown::TryReserveError;
