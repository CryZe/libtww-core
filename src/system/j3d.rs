use super::gx;

extern "C" {
    #[link_name = "j3dSys"]
    pub static mut CAMERA_MATRIX: gx::Mtx;
}
