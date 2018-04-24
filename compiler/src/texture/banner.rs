//! Based on http://www.gc-forever.com/yagcd/chap14.html#sec14.1
//! and http://wiki.tockdom.com/wiki/Image_Formats#RGB5A3

use encoding_rs::{UTF_8, SHIFT_JIS};

use super::image;

const WIDTH: usize = 96;
const HEIGHT: usize = 32;
const UNCOMPRESSED_BYTES_PER_PIXEL: usize = 4;
const COMPRESSED_BYTES_PER_PIXEL: usize = 2;
const UNCOMPRESSED_IMAGE_SIZE: usize = WIDTH * HEIGHT * UNCOMPRESSED_BYTES_PER_PIXEL;
const COMPRESSED_IMAGE_SIZE: usize = WIDTH * HEIGHT * COMPRESSED_BYTES_PER_PIXEL;
const SHORT_TEXT_LEN: usize = 0x20;
const LONG_TEXT_LEN: usize = 0x40;
const DESCRIPTION_LEN: usize = 0x80;
const MAGIC_LEN: usize = 4;
const OFFSET_IMAGE: usize = 0x20;
const OFFSET_GAME_NAME: usize = OFFSET_IMAGE + COMPRESSED_IMAGE_SIZE;
const OFFSET_DEVELOPER_NAME: usize = OFFSET_GAME_NAME + SHORT_TEXT_LEN;
const OFFSET_FULL_GAME_NAME: usize = OFFSET_DEVELOPER_NAME + SHORT_TEXT_LEN;
const OFFSET_FULL_DEVELOPER_NAME: usize = OFFSET_FULL_GAME_NAME + LONG_TEXT_LEN;
const OFFSET_GAME_DESCRIPTION: usize = OFFSET_FULL_DEVELOPER_NAME + LONG_TEXT_LEN;
const BANNER_LEN: usize = OFFSET_GAME_DESCRIPTION + DESCRIPTION_LEN;

pub struct Banner {
    pub magic: [u8; MAGIC_LEN],
    pub image: [u8; UNCOMPRESSED_IMAGE_SIZE],
    pub game_name: String,
    pub developer_name: String,
    pub full_game_name: String,
    pub full_developer_name: String,
    pub game_description: String,
}

fn read_string(is_japanese: bool, bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&x| x == 0).unwrap_or(bytes.len());
    let bytes = &bytes[..end];
    let encoding = if is_japanese { SHIFT_JIS } else { UTF_8 };
    encoding
        .decode_without_bom_handling_and_without_replacement(bytes)
        .expect("Couldn't parse string")
        .into_owned()
}

fn write_string(is_japanese: bool, text: &str, bytes: &mut [u8]) {
    let encoding = if is_japanese { SHIFT_JIS } else { UTF_8 };
    encoding.new_encoder().encode_from_utf8(text, bytes, true);
}

impl Banner {
    pub fn parse(is_japanese: bool, data: &[u8]) -> Self {
        let mut magic = [0; MAGIC_LEN];
        magic.copy_from_slice(&data[..MAGIC_LEN]);
        if &magic != b"BNR1" && &magic != b"BNR2" {
            panic!("Invalid Banner File");
        }

        let image_data = &data[OFFSET_IMAGE..][..COMPRESSED_IMAGE_SIZE];
        let mut rgba_image = [0; UNCOMPRESSED_IMAGE_SIZE];
        image::decode_rgb5a3(image_data, &mut rgba_image, (WIDTH, HEIGHT));

        let game_name = read_string(is_japanese, &data[OFFSET_GAME_NAME..][..SHORT_TEXT_LEN]);
        let developer_name = read_string(
            is_japanese,
            &data[OFFSET_DEVELOPER_NAME..][..SHORT_TEXT_LEN],
        );
        let full_game_name =
            read_string(is_japanese, &data[OFFSET_FULL_GAME_NAME..][..LONG_TEXT_LEN]);
        let full_developer_name = read_string(
            is_japanese,
            &data[OFFSET_FULL_DEVELOPER_NAME..][..LONG_TEXT_LEN],
        );
        let game_description = read_string(
            is_japanese,
            &data[OFFSET_GAME_DESCRIPTION..][..DESCRIPTION_LEN],
        );

        Self {
            magic,
            image: rgba_image,
            game_name,
            developer_name,
            full_game_name,
            full_developer_name,
            game_description,
        }
    }

    pub fn to_bytes(&self, is_japanese: bool) -> [u8; BANNER_LEN] {
        let mut data = [0; BANNER_LEN];

        data[..MAGIC_LEN].copy_from_slice(&self.magic);

        image::encode_rgb5a3(
            &self.image,
            &mut data[OFFSET_IMAGE..][..COMPRESSED_IMAGE_SIZE],
            (WIDTH, HEIGHT),
        );

        write_string(
            is_japanese,
            &self.game_name,
            &mut data[OFFSET_GAME_NAME..][..SHORT_TEXT_LEN],
        );
        write_string(
            is_japanese,
            &self.developer_name,
            &mut data[OFFSET_DEVELOPER_NAME..][..SHORT_TEXT_LEN],
        );
        write_string(
            is_japanese,
            &self.full_game_name,
            &mut data[OFFSET_FULL_GAME_NAME..][..LONG_TEXT_LEN],
        );
        write_string(
            is_japanese,
            &self.full_developer_name,
            &mut data[OFFSET_FULL_DEVELOPER_NAME..][..LONG_TEXT_LEN],
        );
        write_string(
            is_japanese,
            &self.game_description,
            &mut data[OFFSET_GAME_DESCRIPTION..][..DESCRIPTION_LEN],
        );

        data
    }
}
