//! Based on https://github.com/devkitPro/libogc/blob/90b38ee52cb14db11ee961b0e167fda9ed87d090/gc/ogc/gx.h

use core::mem::transmute;
use core::ptr::write_volatile;
use Addr;

const WG_PIPE: Addr = 0xCC008000;

pub const POINTS: u8 = 0xB8;
pub const LINES: u8 = 0xA8;
pub const LINESTRIP: u8 = 0xB0;
pub const TRIANGLES: u8 = 0x90;
pub const TRIANGLESTRIP: u8 = 0x98;
pub const TRIANGLEFAN: u8 = 0xA0;
pub const QUADS: u8 = 0x80;

pub const VTXFMT0: u8 = 0;
pub const VTXFMT1: u8 = 1;
pub const VTXFMT2: u8 = 2;
pub const VTXFMT3: u8 = 3;
pub const VTXFMT4: u8 = 4;
pub const VTXFMT5: u8 = 5;
pub const VTXFMT6: u8 = 6;
pub const VTXFMT7: u8 = 7;

pub const VA_PTNMTXIDX: u32 = 0;
pub const VA_TEX0MTXIDX: u32 = 1;
pub const VA_TEX1MTXIDX: u32 = 2;
pub const VA_TEX2MTXIDX: u32 = 3;
pub const VA_TEX3MTXIDX: u32 = 4;
pub const VA_TEX4MTXIDX: u32 = 5;
pub const VA_TEX5MTXIDX: u32 = 6;
pub const VA_TEX6MTXIDX: u32 = 7;
pub const VA_TEX7MTXIDX: u32 = 8;
pub const VA_POS: u32 = 9;
pub const VA_NRM: u32 = 10;
pub const VA_CLR0: u32 = 11;
pub const VA_CLR1: u32 = 12;
pub const VA_TEX0: u32 = 13;
pub const VA_TEX1: u32 = 14;
pub const VA_TEX2: u32 = 15;
pub const VA_TEX3: u32 = 16;
pub const VA_TEX4: u32 = 17;
pub const VA_TEX5: u32 = 18;
pub const VA_TEX6: u32 = 19;
pub const VA_TEX7: u32 = 20;
pub const POSMTXARRAY: u32 = 21;
pub const NRMMTXARRAY: u32 = 22;
pub const TEXMTXARRAY: u32 = 23;
pub const LIGHTARRAY: u32 = 24;
pub const VA_NBT: u32 = 25;
pub const VA_MAXATTR: u32 = 26;
pub const VA_NULL: u32 = 0xff;

pub const POS_XY: u32 = 0;
pub const POS_XYZ: u32 = 1;
pub const NRM_XYZ: u32 = 0;
pub const NRM_NBT: u32 = 1;
pub const NRM_NBT3: u32 = 2;
pub const CLR_RGB: u32 = 0;
pub const CLR_RGBA: u32 = 1;
pub const TEX_S: u32 = 0;
pub const TEX_ST: u32 = 1;

pub const U8: u32 = 0;
pub const S8: u32 = 1;
pub const U16: u32 = 2;
pub const S16: u32 = 3;
pub const F32: u32 = 4;
pub const RGB565: u32 = 0;
pub const RGB8: u32 = 1;
pub const RGBX8: u32 = 2;
pub const RGBA4: u32 = 3;
pub const RGBA6: u32 = 4;
pub const RGBA8: u32 = 5;

pub const BM_NONE: u8 = 0;
pub const BM_BLEND: u8 = 1;
pub const BM_LOGIC: u8 = 2;
pub const BM_SUBTRACT: u8 = 3;
pub const MAX_BLENDMODE: u8 = 4;

pub const BL_ZERO: u8 = 0;
pub const BL_ONE: u8 = 1;
pub const BL_SRCCLR: u8 = 2;
pub const BL_INVSRCCLR: u8 = 3;
pub const BL_SRCALPHA: u8 = 4;
pub const BL_INVSRCALPHA: u8 = 5;
pub const BL_DSTALPHA: u8 = 6;
pub const BL_INVDSTALPHA: u8 = 7;
pub const BL_DSTCLR: u8 = BL_SRCCLR;
pub const BL_INVDSTCLR: u8 = BL_INVSRCCLR;

/// 0
pub const LO_CLEAR: u8 = 0;
/// src & dst
pub const LO_AND: u8 = 1;
/// src & ~dst
pub const LO_REVAND: u8 = 2;
/// src
pub const LO_COPY: u8 = 3;
/// ~src & dst
pub const LO_INVAND: u8 = 4;
/// dst
pub const LO_NOOP: u8 = 5;
/// src ^ dst
pub const LO_XOR: u8 = 6;
/// src | dst
pub const LO_OR: u8 = 7;
/// ~(src | dst)
pub const LO_NOR: u8 = 8;
/// ~(src ^ dst)
pub const LO_EQUIV: u8 = 9;
/// ~dst
pub const LO_INV: u8 = 10;
/// src | ~dst
pub const LO_REVOR: u8 = 11;
/// ~src
pub const LO_INVCOPY: u8 = 12;
/// ~src | dst
pub const LO_INVOR: u8 = 13;
/// ~(src & dst)
pub const LO_NAND: u8 = 14;
/// 1
pub const LO_SET: u8 = 15;

pub const PNMTX0: u32 = 0;
pub const PNMTX1: u32 = 3;
pub const PNMTX2: u32 = 6;
pub const PNMTX3: u32 = 9;
pub const PNMTX4: u32 = 12;
pub const PNMTX5: u32 = 15;
pub const PNMTX6: u32 = 18;
pub const PNMTX7: u32 = 21;
pub const PNMTX8: u32 = 24;
pub const PNMTX9: u32 = 27;

pub fn set_blend_mode(mode: u8, src_fact: u8, dst_fact: u8, op: u8) {
    let GXSetBlendMode = unsafe { transmute::<Addr, extern "C" fn(u8, u8, u8, u8)>(0x8032425c) };
    GXSetBlendMode(mode, src_fact, dst_fact, op)
}

pub fn begin(primitive: u8, vtxfmt: u8, vtxcnt: u16) {
    let GXBegin = unsafe { transmute::<Addr, extern "C" fn(u8, u8, u16)>(0x80320e0c) };
    GXBegin(primitive, vtxfmt, vtxcnt)
}

pub fn end() {}

pub fn set_vtx_attr_fmt(vtxfmt: u8, vtxattr: u32, comptype: u32, compsize: u32, frac: u32) {
    let GXSetVtxAttrFmt = unsafe { transmute::<Addr, extern "C" fn(u8, u32, u32, u32, u32)>(0x8031f850) };
    GXSetVtxAttrFmt(vtxfmt, vtxattr, comptype, compsize, frac)
}

pub fn load_pos_mtx_imm(mtx: *mut Mtx, pnidx: u32) {
    let GXLoadPosMtxImm = unsafe { transmute::<Addr, extern "C" fn(*mut Mtx, u32)>(0x8032493c) };
    GXLoadPosMtxImm(mtx, pnidx)
}

pub fn submit_f32(val: f32) {
    unsafe {
        write_volatile(WG_PIPE as *mut f32, val);
    }
}

pub fn submit_f32s(arr: &[f32; 3]) {
    for &v in arr {
        submit_f32(v);
    }
}

pub fn submit_u32(val: u32) {
    unsafe {
        write_volatile(WG_PIPE as *mut u32, val);
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mtx {
    cells: [[f32; 4]; 3],
}

impl Mtx {
    pub fn identity() -> Self {
        Mtx {
            cells: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
        }
    }
}
