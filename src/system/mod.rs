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
        fn memcd_ctrl_c_update(this: *mut u8);
        #[link_name = "mDoCPd_Read()"]
        fn controller_read();
        #[link_name = "mDoAud_Execute()"]
        fn audio_execute();
        #[link_name = "fapGm_Execute()"]
        fn game_execute();
        #[link_name = "debug()"]
        fn debugger_run();

        #[link_name = "MemCardWorkArea0"]
        static mut MEM_CARD_WORK_AREA_BYTES: [u8; 0xa000];
        #[link_name = "MemCardWorkArea0"]
        static mut MEM_CARD_WORK_AREA_WORDS: [u32; 0xa000 / 4];
    }

    // We don't want to inline the per frame code, as the per frame code itself
    // requires some stack space and we want to allocate that extremely
    // temporarily in order to make sure that the game's code is not negatively
    // impacted.
    let mut per_frame = core::convert::identity(
        #[inline(never)]
        || per_frame(),
    );

    unsafe {
        const SOME_WORD_OFFSET: usize = 10100 / 4;
        const SOME_BYTE_OFFSET: usize = 10048;
        const SOME_OTHER_BYTE_OFFSET: usize = 10384;

        loop {
            MEM_CARD_WORK_AREA_WORDS[SOME_WORD_OFFSET] += 1;

            if let Some(0) = MEM_CARD_WORK_AREA_WORDS[SOME_WORD_OFFSET]
                .checked_rem(MEM_CARD_WORK_AREA_BYTES[SOME_BYTE_OFFSET] as _)
            {
                heap_check_all();
            }

            if MEM_CARD_WORK_AREA_BYTES[SOME_OTHER_BYTE_OFFSET] != 0 {
                // FIXME: The linker doesn't have this address available as a
                // symbol. This is the beginning of the .data section. We should
                // make it available to allow this code to be portable.
                memcd_ctrl_c_update(0x80364A20 as *mut u8);
            }

            controller_read();
            audio_execute();
            per_frame();
            game_execute();
            debugger_run();
        }
    }
}
