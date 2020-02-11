#![no_std]
#![cfg_attr(feature = "math", feature(core_float))]

pub extern crate alloc;
pub use alloc::{format, vec};
pub use system::allocator::WindWakerAlloc as Alloc;

extern crate arrayvec;
extern crate gcn;

pub mod game;
pub mod link;
pub mod system;
pub mod warping;

pub type Addr = usize;
pub use link::Link;

use core::fmt;

#[repr(C, packed)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Coord {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
}

impl Clone for Coord {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe { write!(f, "{:.2}, {:.2}, {:.2}", self.x, self.y, self.z) }
    }
}

pub mod prelude {
    pub use alloc::{boxed::Box, vec::Vec};
}
