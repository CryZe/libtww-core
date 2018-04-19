use super::gx::Mtx44;

#[repr(C)]
pub struct GameInfo {
    unknown: [u8; 0x5cec],
    pub projection_base: *mut ProjectionBase,
}

#[repr(C)]
pub struct ProjectionBase {
    unknown: [u8; 256],
    pub projection_mtx: Mtx44,
}

extern "C" {
    #[link_name = "g_dComIfG_gameInfo"]
    pub static mut GAME_INFO: GameInfo;
}
