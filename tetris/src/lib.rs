#![no_std]
extern crate alloc;

pub use crate::board::*;
pub use crate::data::*;
pub use crate::game::*;
pub use crate::rng::*;

mod data;
mod rng;
mod board;
mod game;
