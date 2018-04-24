pub const BYTES_PER_PIXEL: usize = 2;

pub fn to_rgba(v: &[u8]) -> [u8; 4] {
    let (x, y) = (v[0], v[1]);

    let high_bit = x >> 7;
    if high_bit != 0 {
        // 1RRRRRGG GGGBBBBB
        let r = (x >> 2) & 0b11111;
        let g = ((x & 0b11) << 3) | (y >> 5);
        let b = y & 0b11111;

        [0x8 * r, 0x8 * g, 0x8 * b, 0xFF]
    } else {
        // 0AAARRRR GGGGBBBB
        let a = (x >> 4) & 0b111;
        let r = x & 0b1111;
        let g = (y >> 4) & 0b1111;
        let b = y & 0b1111;

        [0x11 * r, 0x11 * g, 0x11 * b, 0x20 * a]
    }
}

pub fn from_rgba(v: &[u8]) -> [u8; BYTES_PER_PIXEL] {
    let (r, g, b, a) = (v[0], v[1], v[2], v[3]);

    if a >= 0xF0 {
        // 1RRRRRGG GGGBBBBB
        let (r, g, b) = (
            r.saturating_add(4),
            g.saturating_add(4),
            b.saturating_add(4),
        );
        let (r, g, b) = (r / 0x8, g / 0x8, b / 0x8);

        let x = (1 << 7) | (r << 2) | (g >> 3);
        let y = (g << 5) | b;

        [x, y]
    } else {
        // 0AAARRRR GGGGBBBB
        let (r, g, b, a) = (
            r.saturating_add(8) / 0x11,
            g.saturating_add(8) / 0x11,
            b.saturating_add(8) / 0x11,
            a.saturating_add(0x10) / 0x20,
        );

        let x = (a << 4) | r;
        let y = (g << 4) | b;

        [x, y]
    }
}
