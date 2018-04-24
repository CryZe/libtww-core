#![allow(non_snake_case)]

//! Based on https://github.com/devkitPro/libogc/blob/90b38ee52cb14db11ee961b0e167fda9ed87d090/gc/ogc/gx.h
//! Documentation: http://libogc.devkitpro.org/gx_8h.html

use core::ptr::write_volatile;
use Addr;

const WG_PIPE: Addr = 0xCC008000;

pub const FALSE: u8 = 0;
pub const TRUE: u8 = 1;
pub const DISABLE: u8 = 0;
pub const ENABLE: u8 = 1;

pub const POINTS: u8 = 0xB8;
pub const LINES: u8 = 0xA8;
pub const LINESTRIP: u8 = 0xB0;
pub const TRIANGLES: u8 = 0x90;
pub const TRIANGLESTRIP: u8 = 0x98;
pub const TRIANGLEFAN: u8 = 0xA0;
pub const QUADS: u8 = 0x80;

pub const SRC_REG: u8 = 0;
pub const SRC_VTX: u8 = 1;

// lightid Light ID
/// Light 0
pub const LIGHT0: u8 = 0x001;
/// Light 2
pub const LIGHT1: u8 = 0x002;
/// Light 3
pub const LIGHT2: u8 = 0x004;
/// Light 4
pub const LIGHT3: u8 = 0x008;
/// Light 5
pub const LIGHT4: u8 = 0x010;
/// Light 6
pub const LIGHT5: u8 = 0x020;
/// Light 7
pub const LIGHT6: u8 = 0x040;
/// Light 8
pub const LIGHT7: u8 = 0x080;
// /// All lights
// pub const MAXLIGHT: u8 = 0x100;
/// No lights
pub const LIGHT_NULL: u8 = 0x000;

// difffn Diffuse function
pub const DF_NONE: u8 = 0;
pub const DF_SIGNED: u8 = 1;
pub const DF_CLAMP: u8 = 2;

// attenfunc Attenuation function
/// Specular computation
pub const AF_SPEC: u8 = 0;
/// Spot light attenuation
pub const AF_SPOT: u8 = 1;
/// No attenuation
pub const AF_NONE: u8 = 2;

// vtxfmt Vertex format index
pub const VTXFMT0: u8 = 0;
pub const VTXFMT1: u8 = 1;
pub const VTXFMT2: u8 = 2;
pub const VTXFMT3: u8 = 3;
pub const VTXFMT4: u8 = 4;
pub const VTXFMT5: u8 = 5;
pub const VTXFMT6: u8 = 6;
pub const VTXFMT7: u8 = 7;

// vtxattrin Vertex data input type
/// Input data is not used
pub const NONE: u8 = 0;
/// Input data is set direct
pub const DIRECT: u8 = 1;
/// Input data is set by a 8bit index
pub const INDEX8: u8 = 2;
/// Input data is set by a 16bit index
pub const INDEX16: u8 = 3;

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

// Compare type
pub const NEVER: u8 = 0;
pub const LESS: u8 = 1;
pub const EQUAL: u8 = 2;
pub const LEQUAL: u8 = 3;
pub const GREATER: u8 = 4;
pub const NEQUAL: u8 = 5;
pub const GEQUAL: u8 = 6;
pub const ALWAYS: u8 = 7;

// Alpha combine control
pub const AOP_AND: u8 = 0;
pub const AOP_OR: u8 = 1;
pub const AOP_XOR: u8 = 2;
pub const AOP_XNOR: u8 = 3;
pub const MAX_ALPHAOP: u8 = 4;

// TEV stage
pub const TEVSTAGE0: u8 = 0;
pub const TEVSTAGE1: u8 = 1;
pub const TEVSTAGE2: u8 = 2;
pub const TEVSTAGE3: u8 = 3;
pub const TEVSTAGE4: u8 = 4;
pub const TEVSTAGE5: u8 = 5;
pub const TEVSTAGE6: u8 = 6;
pub const TEVSTAGE7: u8 = 7;
pub const TEVSTAGE8: u8 = 8;
pub const TEVSTAGE9: u8 = 9;
pub const TEVSTAGE10: u8 = 10;
pub const TEVSTAGE11: u8 = 11;
pub const TEVSTAGE12: u8 = 12;
pub const TEVSTAGE13: u8 = 13;
pub const TEVSTAGE14: u8 = 14;
pub const TEVSTAGE15: u8 = 15;
pub const MAX_TEVSTAGE: u8 = 16;

// TEV combiner operation
/// Cv=CrCt; Av=ArAt
pub const MODULATE: u8 = 0;
/// Cv=(1-At)Cr + AtCt; Av=Ar
pub const DECAL: u8 = 1;
/// Cv=(1-Ct)Cr + Ct; Av=AtAr
pub const BLEND: u8 = 2;
/// Cv=Ct; Ar=At
pub const REPLACE: u8 = 3;
/// Cv=Cr; Av=Ar
pub const PASSCLR: u8 = 4;

// Color channel ID
pub const COLOR0: u8 = 0;
pub const COLOR1: u8 = 1;
pub const ALPHA0: u8 = 2;
pub const ALPHA1: u8 = 3;
pub const COLOR0A0: u8 = 4;
pub const COLOR1A1: u8 = 5;
pub const COLORZERO: u8 = 6;
pub const ALPHA_BUMP: u8 = 7;
pub const ALPHA_BUMPN: u8 = 8;
pub const COLOR_NULL: u8 = 0xff;

// texmapid texture map slot
/// Texture map slot 0
pub const TEXMAP0: u32 = 0;
/// Texture map slot 1
pub const TEXMAP1: u32 = 1;
/// Texture map slot 2
pub const TEXMAP2: u32 = 2;
/// Texture map slot 3
pub const TEXMAP3: u32 = 3;
/// Texture map slot 4
pub const TEXMAP4: u32 = 4;
/// Texture map slot 5
pub const TEXMAP5: u32 = 5;
/// Texture map slot 6
pub const TEXMAP6: u32 = 6;
/// Texture map slot 7
pub const TEXMAP7: u32 = 7;
pub const MAX_TEXMAP: u32 = 8;
/// No texmap
pub const TEXMAP_NULL: u32 = 0xff;
/// Disable texmap lookup for this texmap slot (use bitwise OR with a texture map slot).
pub const TEXMAP_DISABLE: u32 = 0x100;

// texcoordid texture coordinate slot
pub const TEXCOORD0: u8 = 0x0;
pub const TEXCOORD1: u8 = 0x1;
pub const TEXCOORD2: u8 = 0x2;
pub const TEXCOORD3: u8 = 0x3;
pub const TEXCOORD4: u8 = 0x4;
pub const TEXCOORD5: u8 = 0x5;
pub const TEXCOORD6: u8 = 0x6;
pub const TEXCOORD7: u8 = 0x7;
pub const MAXCOORD: u8 = 0x8;
pub const TEXCOORD_NULL: u8 = 0xff;

// cullmode Backface culling mode
/// Do not cull any primitives.
pub const CULL_NONE: u8 = 0;
/// Cull front-facing primitives.
pub const CULL_FRONT: u8 = 1;
/// Cull back-facing primitives.
pub const CULL_BACK: u8 = 2;
/// Cull all primitives.
pub const CULL_ALL: u8 = 3;

// texmtx Texture matrix index
pub const TEXMTX0: u32 = 30;
pub const TEXMTX1: u32 = 33;
pub const TEXMTX2: u32 = 36;
pub const TEXMTX3: u32 = 39;
pub const TEXMTX4: u32 = 42;
pub const TEXMTX5: u32 = 45;
pub const TEXMTX6: u32 = 48;
pub const TEXMTX7: u32 = 51;
pub const TEXMTX8: u32 = 54;
pub const TEXMTX9: u32 = 57;
pub const IDENTITY: u32 = 60;

// dttmtx Post-transform texture matrix index
pub const DTTMTX1: u32 = 67;
pub const DTTMTX0: u32 = 64;
pub const DTTMTX2: u32 = 70;
pub const DTTMTX3: u32 = 73;
pub const DTTMTX4: u32 = 76;
pub const DTTMTX5: u32 = 79;
pub const DTTMTX6: u32 = 82;
pub const DTTMTX7: u32 = 85;
pub const DTTMTX8: u32 = 88;
pub const DTTMTX9: u32 = 91;
pub const DTTMTX10: u32 = 94;
pub const DTTMTX11: u32 = 97;
pub const DTTMTX12: u32 = 100;
pub const DTTMTX13: u32 = 103;
pub const DTTMTX14: u32 = 106;
pub const DTTMTX15: u32 = 109;
pub const DTTMTX16: u32 = 112;
pub const DTTMTX17: u32 = 115;
pub const DTTMTX18: u32 = 118;
pub const DTTMTX19: u32 = 121;
pub const DTTIDENTITY: u32 = 125;

// texoff Texture offset value
pub const TO_ZERO: u8 = 0;
pub const TO_SIXTEENTH: u8 = 1;
pub const TO_EIGHTH: u8 = 2;
pub const TO_FOURTH: u8 = 3;
pub const TO_HALF: u8 = 4;
pub const TO_ONE: u8 = 5;
pub const MAX_TEXOFFSET: u8 = 6;

// mtxtype Matrix type
pub const MTX2x4: u8 = 0;
pub const MTX3x4: u8 = 1;

// texgentyp Texture coordinate generation type
/// 2x4 matrix multiply on the input attribute and generate S,T texture coordinates.
pub const TG_MTX3x4: u32 = 0;
/// 3x4 matrix multiply on the input attribute and generate S,T,Q coordinates; S,T are then divided by Q to produce the actual 2D texture coordinates.
pub const TG_MTX2x4: u32 = 1;
/// Use light 0 in the bump map calculation.
pub const TG_BUMP0: u32 = 2;
/// Use light 1 in the bump map calculation.
pub const TG_BUMP1: u32 = 3;
/// Use light 2 in the bump map calculation.
pub const TG_BUMP2: u32 = 4;
/// Use light 3 in the bump map calculation.
pub const TG_BUMP3: u32 = 5;
/// Use light 4 in the bump map calculation.
pub const TG_BUMP4: u32 = 6;
/// Use light 5 in the bump map calculation.
pub const TG_BUMP5: u32 = 7;
/// Use light 6 in the bump map calculation.
pub const TG_BUMP6: u32 = 8;
/// Use light 7 in the bump map calculation.
pub const TG_BUMP7: u32 = 9;
/// Coordinates generated from vertex lighting results; one of the color channel results is converted into texture coordinates.
pub const TG_SRTG: u32 = 10;

// texgensrc Texture coordinate source
pub const TG_POS: u32 = 0;
pub const TG_NRM: u32 = 1;
pub const TG_BINRM: u32 = 2;
pub const TG_TANGENT: u32 = 3;
pub const TG_TEX0: u32 = 4;
pub const TG_TEX1: u32 = 5;
pub const TG_TEX2: u32 = 6;
pub const TG_TEX3: u32 = 7;
pub const TG_TEX4: u32 = 8;
pub const TG_TEX5: u32 = 9;
pub const TG_TEX6: u32 = 10;
pub const TG_TEX7: u32 = 11;
pub const TG_TEXCOORD0: u32 = 12;
pub const TG_TEXCOORD1: u32 = 13;
pub const TG_TEXCOORD2: u32 = 14;
pub const TG_TEXCOORD3: u32 = 15;
pub const TG_TEXCOORD4: u32 = 16;
pub const TG_TEXCOORD5: u32 = 17;
pub const TG_TEXCOORD6: u32 = 18;
pub const TG_COLOR0: u32 = 19;
pub const TG_COLOR1: u32 = 20;

pub const _TF_ZTF: u8 = 0x10;
pub const _TF_CTF: u8 = 0x20;

// texfmt Texture format
pub const TF_I4: u8 = 0x0;
pub const TF_I8: u8 = 0x1;
pub const TF_IA4: u8 = 0x2;
pub const TF_IA8: u8 = 0x3;
pub const TF_RGB565: u8 = 0x4;
pub const TF_RGB5A3: u8 = 0x5;
pub const TF_RGBA8: u8 = 0x6;
pub const TF_CI4: u8 = 0x8;
pub const TF_CI8: u8 = 0x9;
pub const TF_CI14: u8 = 0xa;
/// Compressed
pub const TF_CMPR: u8 = 0xE;

pub const TL_IA8: u8 = 0x00;
pub const TL_RGB565: u8 = 0x01;
pub const TL_RGB5A3: u8 = 0x02;

/// For copying 4 bits from red
pub const CTF_R4: u8 = 0x0 | _TF_CTF;
/// For copying 4 bits from red, 4 bits from alpha
pub const CTF_RA4: u8 = 0x2 | _TF_CTF;
/// For copying 8 bits from red, 8 bits from alpha
pub const CTF_RA8: u8 = 0x3 | _TF_CTF;
pub const CTF_YUVA8: u8 = 0x6 | _TF_CTF;
/// For copying 8 bits from alpha
pub const CTF_A8: u8 = 0x7 | _TF_CTF;
/// For copying 8 bits from red
pub const CTF_R8: u8 = 0x8 | _TF_CTF;
/// For copying 8 bits from green
pub const CTF_G8: u8 = 0x9 | _TF_CTF;
/// For copying 8 bits from blue
pub const CTF_B8: u8 = 0xA | _TF_CTF;
/// For copying 8 bits from red, 8 bits from green
pub const CTF_RG8: u8 = 0xB | _TF_CTF;
/// For copying 8 bits from green, 8 bits from blue
pub const CTF_GB8: u8 = 0xC | _TF_CTF;

// Text Wrap Mode
pub const CLAMP: u8 = 0;
pub const REPEAT: u8 = 1;
pub const MIRROR: u8 = 2;
pub const MAXTEXWRAPMODE: u8 = 3;

pub fn end() {}

extern "C" {
    #[link_name = "GXSetBlendMode"]
    pub fn set_blend_mode(mode: u8, src_fact: u8, dst_fact: u8, op: u8);
    #[link_name = "GXBegin"]
    pub fn begin(primitive: u8, vtxfmt: u8, vtxcnt: u16);
    #[link_name = "GXSetVtxAttrFmt"]
    pub fn set_vtx_attr_fmt(vtxfmt: u8, vtxattr: u32, comptype: u32, compsize: u32, frac: u32);
    #[link_name = "GXLoadPosMtxImm"]
    pub fn load_pos_mtx_imm(mtx: *mut Mtx, pnidx: u32);
    #[link_name = "GXSetNumIndStages"]
    pub fn set_num_ind_stages(nstages: u8);
    #[link_name = "GXSetTevDirect"]
    pub fn set_tev_direct(tevstage: u8);
    #[link_name = "GXSetAlphaCompare"]
    pub fn set_alpha_compare(comp0: u8, ref0: u8, aop: u8, comp1: u8, ref1: u8);
    #[link_name = "GXSetZMode"]
    pub fn set_z_mode(enable: u8, func: u8, update_enable: u8);
    #[link_name = "GXSetTevOp"]
    pub fn set_tev_op(tevstage: u8, mode: u8);
    #[link_name = "GXSetNumChans"]
    pub fn set_num_chans(num: u8);
    #[link_name = "GXSetNumTevStages"]
    pub fn set_num_tev_stages(num: u8);
    #[link_name = "GXSetNumTexGens"]
    pub fn set_num_tex_gens(nr: u32);
    #[link_name = "GXSetTevOrder"]
    pub fn set_tev_order(tevstage: u8, texcoord: u8, texmap: u32, color: u8);
    #[link_name = "GXSetCullMode"]
    pub fn set_cull_mode(mode: u8);
    #[link_name = "GXLoadTexMtxImm"]
    pub fn load_tex_mtx_imm(mtx: *mut Mtx, texidx: u32, typ: u8);
    #[link_name = "GXSetChanCtrl"]
    pub fn set_chan_ctrl(channel: i32, enable: u8, ambsrc: u8, matsrc: u8, litmask: u8, diff_fn: u8, attn_fn: u8);
    #[link_name = "GXSetCurrentMtx"]
    pub fn set_current_mtx(mtx: u32);
    #[link_name = "GXSetTexCoordGen2"]
    pub fn set_tex_coord_gen2(texcoord: u16, tgen_typ: u32, tgen_src: u32, mtxsrc: u32, normalize: u32, postmtx: u32);
    #[link_name = "GXSetLineWidth"]
    pub fn set_line_width(width: u8, fmt: u8);
    #[link_name = "GXClearVtxDesc"]
    pub fn clear_vtx_desc();
    #[link_name = "GXSetVtxDesc"]
    pub fn set_vtx_desc(attr: u8, typ: u8);
    #[link_name = "GXInitTexObj"]
    pub fn init_tex_obj(obj: *mut TexObj, img_ptr: *const u8, wd: u16, ht: u16, fmt: u8, wrap_s: u8, wrap_t: u8, mipmap: u8);
    #[link_name = "GXLoadTexObj"]
    pub fn load_tex_obj(obj: *mut TexObj, mapid: u8);
    #[link_name = "GXInvalidateTexAll"]
    pub fn invalidate_tex_all();
}

pub unsafe fn set_tex_coord_gen(texcoord: u16, tgen_typ: u32, tgen_src: u32, mtxsrc: u32) {
    set_tex_coord_gen2(texcoord, tgen_typ, tgen_src, mtxsrc, FALSE as u32, DTTIDENTITY);
}

pub fn submit_f32(val: f32) {
    unsafe {
        write_volatile(WG_PIPE as *mut f32, val);
    }
}

pub fn submit_f32s(arr: &[f32]) {
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

#[derive(Default)]
#[repr(C)]
pub struct TexObj {
    val: [u32; 8],
}
