extern "C" {
    #[link_name = "J2DDrawLine(f32, f32, f32, f32, JUtility::TColor, i32)"]
    pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, color: TColor, line_width: i32);
    #[link_name = "J2DFillBox(f32, f32, f32, f32, JUtility::TColor)"]
    pub fn fill_box(x: f32, y: f32, w: f32, h: f32, color: TColor);
    #[link_name = "J2DDrawFrame(f32, f32, f32, f32, JUtility::TColor, u8)"]
    pub fn draw_frame(x: f32, y: f32, w: f32, h: f32, color: TColor, line_width: u8);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub rgba: u32,
}

pub type TColor = *mut Color;
