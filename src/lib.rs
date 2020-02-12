#![no_std]
#![cfg_attr(feature = "math", feature(core_float))]

#[cfg(feature = "alloc")]
pub extern crate alloc;
#[cfg(feature = "alloc")]
pub use system::allocator::WindWakerAlloc as Alloc;

extern crate arrayvec;
extern crate gcn;

pub mod futures;
pub mod game;
pub mod link;
pub mod system;
pub mod warping;

pub type Addr = usize;
pub use link::Link;

use core::fmt;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2}, {:.2}, {:.2}", self.x, self.y, self.z)
    }
}

pub mod prelude {
    #[cfg(feature = "alloc")]
    pub use alloc::{boxed::Box, vec::Vec};
}
