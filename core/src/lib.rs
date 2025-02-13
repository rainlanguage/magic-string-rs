#![deny(clippy::all)]
#![allow(clippy::assigning_clones)]

#[macro_use]
extern crate serde;

mod mapping;
mod utils;

mod chunk;
mod source_map;

pub mod magic_string;
pub mod result;

pub use crate::magic_string::*;
pub use crate::result::*;
pub use crate::source_map::SourceMap;
