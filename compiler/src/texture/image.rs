use super::rgb5a3;

const TILE_WIDTH: usize = 4;
const TILE_HEIGHT: usize = 4;
const RGBA_BYTES_PER_PIXEL: usize = 4;

pub fn encode<F>(
    src: &[u8],
    dst: &mut [u8],
    (width, height): (usize, usize),
    encoded_bytes_per_pixel: usize,
    mut encode_pixel: F,
) where
    F: FnMut(&[u8], &mut [u8]),
{
    assert_eq!(height % TILE_HEIGHT, 0);
    assert_eq!(width % TILE_WIDTH, 0);

    let rows = height / TILE_HEIGHT;
    let columns = width / TILE_WIDTH;

    let mut dst = dst.chunks_mut(encoded_bytes_per_pixel);

    for row in 0..rows {
        let row_y = row * TILE_HEIGHT;
        for column in 0..columns {
            let column_x = column * TILE_WIDTH;
            for y in 0..TILE_HEIGHT {
                let y = row_y + y;
                for x in 0..TILE_WIDTH {
                    let x = column_x + x;
                    let pixel_index = RGBA_BYTES_PER_PIXEL * (y * width + x);
                    let src = &src[pixel_index..][..RGBA_BYTES_PER_PIXEL];
                    let dst = dst.next().unwrap();
                    encode_pixel(src, dst);
                }
            }
        }
    }
}

pub fn decode<F>(
    src: &[u8],
    dst: &mut [u8],
    (width, height): (usize, usize),
    encoded_bytes_per_pixel: usize,
    mut decode_pixel: F,
) where
    F: FnMut(&[u8], &mut [u8]),
{
    assert_eq!(height % TILE_HEIGHT, 0);
    assert_eq!(width % TILE_WIDTH, 0);

    let rows = height / TILE_HEIGHT;
    let columns = width / TILE_WIDTH;

    let mut src = src.chunks(encoded_bytes_per_pixel);

    for row in 0..rows {
        let row_y = row * TILE_HEIGHT;
        for column in 0..columns {
            let column_x = column * TILE_WIDTH;
            for y in 0..TILE_HEIGHT {
                let y = row_y + y;
                for x in 0..TILE_WIDTH {
                    let x = column_x + x;
                    let pixel_index = RGBA_BYTES_PER_PIXEL * (y * width + x);
                    let src = src.next().unwrap();
                    let dst = &mut dst[pixel_index..][..RGBA_BYTES_PER_PIXEL];
                    decode_pixel(src, dst);
                }
            }
        }
    }
}

pub fn decode_rgb5a3(src: &[u8], dst: &mut [u8], (width, height): (usize, usize)) {
    decode(
        src,
        dst,
        (width, height),
        rgb5a3::BYTES_PER_PIXEL,
        |src, dst| dst.copy_from_slice(&rgb5a3::to_rgba(src)),
    );
}

pub fn encode_rgb5a3(src: &[u8], dst: &mut [u8], (width, height): (usize, usize)) {
    encode(
        src,
        dst,
        (width, height),
        rgb5a3::BYTES_PER_PIXEL,
        |src, dst| dst.copy_from_slice(&rgb5a3::from_rgba(src)),
    );
}
