#![feature(lang_items, compiler_builtins_lib)]
#![no_std]

extern crate arrayvec;
extern crate compiler_builtins;

pub mod game;
pub mod link;
pub mod warping;
pub mod system;

pub type Addr = system::libc::size_t;
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
