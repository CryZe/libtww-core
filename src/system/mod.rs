#[cfg(feature = "alloc")]
pub mod allocator;
pub mod game_info;
pub mod j2d;
pub mod j3d;
pub mod libc;
#[cfg(feature = "math")]
pub mod math;
pub mod memory;
pub mod tww;
pub use gcn::{gx, os};

pub use self::tww::*;

// We don't want to consume unnecessary stack space, so inline it as much as possible.
#[inline(always)]
pub fn custom_game_loop(mut per_frame: impl FnMut()) -> ! {
    extern "C" {
        #[link_name = "mDoMch_HeapCheckAll()"]
        fn heap_check_all();
        #[link_name = "mDoMemCd_Ctrl_c::update()"]
        fn memory_card_update(this: *const u8);
        #[link_name = "mDoCPd_Read()"]
        fn controller_read();
        #[link_name = "mDoAud_Execute()"]
        fn audio_execute();
        #[link_name = "fapGm_Execute()"]
        fn game_execute();
        #[link_name = "debug()"]
        fn debugger_run();

        #[link_name = "mDoDvdThd::SyncWidthSound"]
        static mut dvd_thread_sync_width_sound: u8;
        #[link_name = "frame$4235"]
        static mut frame_count: u32;
        #[link_name = "fillcheck_check_frame"]
        static mut fillcheck_check_frame: u8;
        #[link_name = "g_mDoMemCd_control"]
        static memory_card_controller: u8;
    }

    // We don't want to inline the per frame code, as the per frame code itself
    // requires some stack space and we want to allocate that extremely
    // temporarily in order to make sure that the game's code is not negatively
    // impacted.
    let mut per_frame = {
        #[inline(never)]
        || per_frame()
    };

    unsafe {
        loop {
            frame_count += 1;

            if let Some(0) = frame_count.checked_rem(fillcheck_check_frame as _) {
                heap_check_all();
            }

            if dvd_thread_sync_width_sound != 0 {
                memory_card_update(&memory_card_controller);
            }

            controller_read();
            audio_execute();
            per_frame();
            game_execute();
            debugger_run();
        }
    }
}
