#![allow(non_snake_case)]

use core::mem::transmute;
use system::memory::{read, write};
use Addr;

extern "C" {
    #[link_name = "JAIZelBasic::getRandomU32(u32)"]
    // TODO Wrong Signature, takes u32
    fn game_random_u32() -> u32;
    #[link_name = "cM_rndF(f32)"]
    // TODO Wrong Signature, takes f32
    fn game_random() -> f64;
    #[link_name = "cDyl_InitAsync()"]
    fn game_cdyl_init_async();
    #[link_name = "dMeter_rupyInit(sub_meter_class*)"]
    fn game_dmeter_rupy_init(addr: Addr);
}

pub fn random_u32() -> u32 {
    unsafe { game_random_u32() }
}

pub fn random() -> f64 {
    unsafe { game_random() }
}

pub fn cdyl_init_async() {
    unsafe { game_cdyl_init_async() }
}

pub fn dmeter_rupy_init(addr: Addr) {
    unsafe { game_dmeter_rupy_init(addr) }
}

pub fn get_frame_count() -> u32 {
    #[repr(C)]
    struct ZelAudio {
        _padding: [u8; 0x10],
        frame_count: u32,
    }
    extern "C" {
        #[link_name = "g_mDoAud_zelAudio"]
        static mut ZEL_AUDIO: ZelAudio;
    }
    unsafe { ZEL_AUDIO.frame_count }
}

pub fn is_pause_menu_up() -> bool {
    read(0x803EA537) // alternative: 0x80396228
}

pub fn set_wind(direction: u8) {
    write(0x803D894A, direction << 5);
}

pub fn get_layer_by_id(id: u32) -> Addr {
    let fpcly_layer = unsafe { transmute::<Addr, extern "C" fn(u32) -> Addr>(0x8003b92c) };
    fpcly_layer(id)
}

pub fn set_current_layer(addr: Addr) {
    let fpcly_set_current_layer = unsafe { transmute::<Addr, extern "C" fn(Addr)>(0x8003b8cc) };
    fpcly_set_current_layer(addr)
}

pub fn get_current_layer() -> Addr {
    let fpcly_current_layer = unsafe { transmute::<Addr, extern "C" fn() -> Addr>(0x8003b8d4) };
    fpcly_current_layer()
}

pub fn get_root_layer() -> Addr {
    read(0x80365B7C)
}

use game::actor::{ActorMemory, ActorTemplate};

pub fn dstage_actor_create(template: *const ActorTemplate, memory: *mut ActorMemory) {
    let dstage_actor_create = unsafe {
        transmute::<Addr, extern "C" fn(*const ActorTemplate, *mut ActorMemory)>(0x8003f484)
    };
    dstage_actor_create(template, memory);
}

pub fn fopacm_create_append() -> &'static mut ActorMemory {
    let fopacm_create_append =
        unsafe { transmute::<Addr, extern "C" fn() -> *mut ActorMemory>(0x80023f3c) };
    let actor_memory = fopacm_create_append();
    unsafe { &mut *actor_memory }
}

pub fn layer_loader(dzr: Addr, layer: Addr, room_id: u8) {
    let layer_loader = unsafe { transmute::<Addr, extern "C" fn(Addr, Addr, u8)>(0x80040f3c) };
    layer_loader(dzr, layer, room_id);
}

pub fn ground_cross(a: Addr, b: Addr) -> f32 {
    let ground_cross = unsafe { transmute::<Addr, extern "C" fn(Addr, Addr) -> f32>(0x80244074) };
    ground_cross(a, b)
}

pub fn fopmsgm_message_set(message_id: u16) {
    let fopmsgm_message_set = unsafe { transmute::<Addr, extern "C" fn(u16)>(0x8002b458) };
    fopmsgm_message_set(message_id)
}

pub fn dStage_dt_c_stageLoader(a: Addr, b: Addr) {
    let stage_loader = unsafe { transmute::<Addr, extern "C" fn(Addr, Addr)>(0x80040f98) };
    stage_loader(a, b)
}

pub fn dSv_player_get_item_c_onItem(dSv_player_get_item_c: Addr, slot_id: i32, unknown: u8) {
    let on_item = unsafe { transmute::<Addr, extern "C" fn(Addr, i32, u8)>(0x800572bc) };
    on_item(dSv_player_get_item_c, slot_id, unknown)
}

pub fn dSv_player_return_place_c_set(
    dSv_player_return_place_c: Addr,
    stage: *const u8,
    room: i8,
    start_code: u8,
) {
    let set = unsafe { transmute::<Addr, extern "C" fn(Addr, *const u8, i8, u8)>(0x800569c0) };
    set(dSv_player_return_place_c, stage, room, start_code)
}

pub struct JKRDvdFile;

impl JKRDvdFile {
    pub fn constructor(this: *mut u8) {
        let constructor = unsafe { transmute::<Addr, extern "C" fn(*mut u8)>(0x802b9d30) };
        constructor(this)
    }

    pub fn destructor(this: *mut u8) {
        let destructor = unsafe { transmute::<Addr, extern "C" fn(*mut u8)>(0x802b9ef4) };
        destructor(this)
    }

    pub fn open(this: *mut u8, path: *const u8) {
        let open = unsafe { transmute::<Addr, extern "C" fn(*mut u8, *const u8)>(0x802b9ffc) };
        open(this, path)
    }

    pub fn read(this: *mut u8, buffer: *mut u8, len: i32, unknown: i32) {
        let read =
            unsafe { transmute::<Addr, extern "C" fn(*mut u8, *mut u8, i32, i32)>(0x802ba15c) };
        read(this, buffer, len, unknown)
    }

    pub fn close(this: *mut u8) {
        let close = unsafe { transmute::<Addr, extern "C" fn(*mut u8)>(0x802ba0e4) };
        close(this)
    }

    pub fn get_file_size(this: *mut u8) -> i32 {
        let get_file_size = unsafe { transmute::<Addr, extern "C" fn(*mut u8) -> i32>(0x802ba328) };
        get_file_size(this)
    }
}

pub struct JUTAssertion;

impl JUTAssertion {
    pub fn get_s_device() -> u32 {
        let get_s_device = unsafe { transmute::<Addr, extern "C" fn() -> u32>(0x802c4d0c) };
        get_s_device()
    }

    pub fn show_assert(s_device: u32, file: *const u8, line: i32, assertion: *const u8) {
        let show_assert =
            unsafe { transmute::<Addr, extern "C" fn(u32, *const u8, i32, *const u8)>(0x802c4e04) };
        show_assert(s_device, file, line, assertion);
    }

    pub fn set_visible(visibility: bool) {
        let set_visible = unsafe { transmute::<Addr, extern "C" fn(bool)>(0x802c5290) };
        set_visible(visibility)
    }
}
