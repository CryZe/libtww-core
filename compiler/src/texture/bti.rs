//! Based on https://github.com/LordNed/JStudio/blob/f49c0a769d8736b8039e19442a039a3594a24d64/JStudio/J3D/BinaryTextureImage.cs
//! and http://wiki.tockdom.com/wiki/BTI_(File_Format)

use byteorder::{WriteBytesExt, BE};
use std::io::{prelude::*, Result};

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Format {
    I4 = 0x00,
    I8 = 0x01,
    IA4 = 0x02,
    IA8 = 0x03,
    RGB565 = 0x04,
    RGB5A3 = 0x05,
    RGBA32 = 0x06,
    C4 = 0x08,
    C8 = 0x09,
    C14X2 = 0x0A,
    CMPR = 0x0E,
}

pub struct Image {
    image_format: Format,
    enable_alpha: bool,
    width: u16,
    height: u16,
    wrap_s: u8,
    wrap_t: u8,
    palette_format: u16,
    palette_len: u16,
    encoded_images: Vec<Vec<u8>>,
}

impl Image {
    pub fn file_size(&self) -> usize {
        0x20 + self.encoded_images.iter().map(|i| i.len()).sum::<usize>()
    }

    pub fn write<W: Write>(&self, mut writer: W) -> Result<()> {
        writer.write_u8(self.image_format as u8)?;
        writer.write_u8(self.enable_alpha as u8)?;
        writer.write_u16::<BE>(self.width)?;
        writer.write_u16::<BE>(self.height)?;
        writer.write_u8(self.wrap_s)?;
        writer.write_u8(self.wrap_t)?;
        // writer.write_u16(self.)

        unimplemented!()
    }
}
