#![feature(const_fn)]
#![no_std]
#![cfg_attr(
    feature = "alloc",
    feature(
        alloc, global_allocator, alloc_system, allocator_api, allocator_internals, macro_reexport
    )
)]
#![cfg_attr(feature = "alloc", default_lib_allocator)]

#[cfg(feature = "alloc")]
#[macro_reexport(vec, format)]
pub extern crate alloc;
#[cfg(feature = "alloc")]
extern crate alloc_system;

#[cfg(feature = "alloc")]
#[global_allocator]
static A: alloc_system::System = alloc_system::System;

extern crate arrayvec;

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
    #[cfg(feature = "alloc")]
    pub use alloc::{boxed::Box, vec::Vec};
}
