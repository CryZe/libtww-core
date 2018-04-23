use system::memory::{read, write};
use Addr;

extern "C" {
    #[link_name = "JAIZelBasic::getRandomU32(u32)"]
    // TODO Wrong Signature, takes u32
    pub fn random_u32() -> u32;
    #[link_name = "cM_rndF(f32)"]
    // TODO Wrong Signature, takes f32
    pub fn random() -> f64;
    #[link_name = "cDyl_InitAsync"]
    pub fn cdyl_init_async();
    #[link_name = "dMeter_rupyInit(sub_meter_class*)"]
    pub fn dmeter_rupy_init(addr: Addr);
    #[link_name = "fpcLy_Layer(u32)"]
    pub fn get_layer_by_id(id: u32) -> Addr;
    #[link_name = "fpcLy_SetCurrentLayer(layer_class*)"]
    pub fn set_current_layer(addr: Addr);
    #[link_name = "fpcLy_CurrentLayer()"]
    pub fn get_current_layer() -> Addr;
}

pub fn get_frame_count() -> u32 {
    read(0x80396218)
}

pub fn is_pause_menu_up() -> bool {
    read(0x803EA537) // alternative: 0x80396228
}

pub fn set_wind(direction: u8) {
    write(0x803D894A, direction << 5);
}

pub fn get_root_layer() -> Addr {
    read(0x80365B7C)
}

use game::actor::{ActorMemory, ActorTemplate};

extern "C" {
    #[link_name = "dStage_actorCreate(stage_actor_data_class*, fopAcM_prm_class*)"]
    pub fn dstage_actor_create(template: *const ActorTemplate, memory: *mut ActorMemory);
    #[link_name = "fopAcM_CreateAppend()"]
    pub fn fopacm_create_append() -> &'static mut ActorMemory;
    #[link_name = "layerLoader(void*, dStage_dt_c*, i32)"]
    pub fn layer_loader(dzr: Addr, layer: Addr, room_id: i32);
    #[link_name = "cBgS::GroundCross(cBgS_GndChk*)"]
    pub fn ground_cross(this: Addr, b: Addr) -> f32;
    #[link_name = "fopMsgM_messageSet(u32)"]
    pub fn fopmsgm_message_set(message_id: u32);
    #[link_name = "dStage_dt_c_stageLoader(void*, dStage_dt_c*)"]
    pub fn dStage_dt_c_stageLoader(a: Addr, b: Addr);
    #[link_name = "dSv_player_get_item_c::onItem(i32, u8)"]
    pub fn dSv_player_get_item_c_onItem(this: Addr, slot_id: i32, unknown: u8);
    #[link_name = "dSv_player_return_place_c::set(i8 const*, i8, u8)"]
    pub fn dSv_player_return_place_c_set(this: Addr, stage: *const u8, room: i8, start_code: u8);
}

extern "C" {
    #[link_name = "JKRDvdFile()"]
    fn jkr_dvd_file_constructor(this: *mut u8);
    #[link_name = "~JKRDvdFile()"]
    fn jkr_dvd_file_destructor(this: *mut u8);
    #[link_name = "JKRDvdFile::open(i8 const*)"]
    fn jkr_dvd_file_open(this: *mut u8, path: *const u8);
    #[link_name = "JKRDvdFile::readData(void*, i32, i32)"]
    fn jkr_dvd_file_read_data(this: *mut u8, buffer: *mut u8, len: i32, unknown: i32);
    #[link_name = "JKRDvdFile::close()"]
    fn jkr_dvd_file_close(this: *mut u8);
    #[link_name = "JKRDvdFile::getFileSize() const"]
    fn jkr_dvd_file_get_file_size(this: *const u8) -> i32;
}

pub struct JKRDvdFile;

impl JKRDvdFile {
    pub fn constructor(this: *mut u8) {
        unsafe { jkr_dvd_file_constructor(this) }
    }

    pub fn destructor(this: *mut u8) {
        unsafe { jkr_dvd_file_destructor(this) }
    }

    pub fn open(this: *mut u8, path: *const u8) {
        unsafe { jkr_dvd_file_open(this, path) }
    }

    pub fn read(this: *mut u8, buffer: *mut u8, len: i32, unknown: i32) {
        unsafe { jkr_dvd_file_read_data(this, buffer, len, unknown) }
    }

    pub fn close(this: *mut u8) {
        unsafe { jkr_dvd_file_close(this) }
    }

    pub fn get_file_size(this: *const u8) -> i32 {
        unsafe { jkr_dvd_file_get_file_size(this) }
    }
}

extern "C" {
    #[link_name = "OSGetCurrentThread"]
    fn os_get_current_thread() -> *const u8;
    #[link_name = "OSIsThreadTerminated"]
    fn os_is_thread_terminated(this: *const u8) -> bool;
    #[link_name = "OSCreateThread"]
    fn os_create_thread(
        this: *mut u8,
        entry: extern "C" fn(*mut u8) -> *mut u8,
        arg: *mut u8,
        stack: *mut u8,
        stack_size: usize,
        priority: i32,
        attr: i16,
    ) -> bool;
    #[link_name = "OSResumeThread"]
    fn os_resume_thread(this: *const u8) -> i32;
    #[link_name = "OSSuspendThread"]
    fn os_suspend_thread(this: *const u8) -> i32;
    #[link_name = "OSJoinThread"]
    fn os_join_thread(this: *const u8, ret_value: *mut *mut u8) -> bool;
    #[link_name = "OSYieldThread"]
    fn os_yield_thread();
    #[link_name = "OSInitMutex"]
    fn os_init_mutex(this: *mut u8);
    #[link_name = "OSLockMutex"]
    fn os_lock_mutex(this: *const u8);
    #[link_name = "OSUnlockMutex"]
    fn os_unlock_mutex(this: *const u8);
    #[link_name = "OSTryLockMutex"]
    fn os_try_lock_mutex(this: *const u8) -> bool;
    #[link_name = "OSInitCond"]
    fn os_init_cond(this: *mut u8);
    #[link_name = "OSWaitCond"]
    fn os_wait_cond(this: *const u8, mutex: *const u8);
    #[link_name = "OSSignalCond"]
    fn os_signal_cond(this: *const u8);
    #[link_name = "OSGetTime"]
    fn os_get_time() -> i64;
    #[link_name = "OSReport"]
    fn os_report(text: *const u8);
    #[link_name = "OSPanic"]
    fn os_panic(file: *const u8, line: i32, message: *const u8);
}

pub struct OS;

impl OS {
    // pub fn allocate_thread() -> Box<[u8]> {
    //     vec![0xCE; 792 + 32].into_boxed_slice()
    // }

    // pub fn allocate_mutex() -> Box<[u8]> {
    //     vec![0xCE; 64 + 32].into_boxed_slice()
    // }

    // pub fn allocate_cond() -> Box<[u8]> {
    //     vec![0xCE; 32].into_boxed_slice()
    // }

    pub fn get_current_thread() -> *const u8 {
        unsafe { os_get_current_thread() }
    }

    pub fn is_thread_terminated(this: *const u8) -> bool {
        unsafe { os_is_thread_terminated(this) }
    }

    pub fn create_thread(
        this: *mut u8,
        entry: extern "C" fn(*mut u8) -> *mut u8,
        arg: *mut u8,
        stack: *mut u8,
        stack_size: usize,
        priority: i32,
        attr: i16,
    ) -> bool {
        unsafe { os_create_thread(this, entry, arg, stack, stack_size, priority, attr) }
    }

    pub fn resume_thread(this: *const u8) -> i32 {
        unsafe { os_resume_thread(this) }
    }

    pub fn suspend_thread(this: *const u8) -> i32 {
        unsafe { os_suspend_thread(this) }
    }

    pub fn join_thread(this: *const u8, ret_value: *mut *mut u8) -> bool {
        unsafe { os_join_thread(this, ret_value) }
    }

    pub fn yield_thread() {
        unsafe { os_yield_thread() }
    }

    pub fn init_mutex(this: *mut u8) {
        unsafe { os_init_mutex(this) }
    }

    pub fn lock_mutex(this: *const u8) {
        unsafe { os_lock_mutex(this) }
    }

    pub fn unlock_mutex(this: *const u8) {
        unsafe { os_unlock_mutex(this) }
    }

    pub fn try_lock_mutex(this: *const u8) -> bool {
        unsafe { os_try_lock_mutex(this) }
    }

    pub fn init_cond(this: *mut u8) {
        unsafe { os_init_cond(this) }
    }

    pub fn wait_cond(this: *const u8, mutex: *const u8) {
        unsafe { os_wait_cond(this, mutex) }
    }

    pub fn signal_cond(this: *const u8) {
        unsafe { os_signal_cond(this) }
    }

    pub fn get_time() -> i64 {
        unsafe { os_get_time() }
    }

    pub fn report(text: *const u8) {
        unsafe { os_report(text) }
    }

    pub fn panic(file: *const u8, line: i32, message: *const u8) {
        unsafe { os_panic(file, line, message) }
    }
}

extern "C" {
    #[link_name = "JUTAssertion::getSDevice()"]
    fn jut_assertion_get_s_device() -> u32;
    #[link_name = "JUTAssertion::showAssert(u32, i8 const*, i32, i8 const*)"]
    fn jut_assertion_show_assert(s_device: u32, file: *const u8, line: i32, assertion: *const u8);
    #[link_name = "JUTAssertion::setVisible(bool)"]
    fn jut_assertion_set_visible(visibility: bool);
}

pub struct JUTAssertion;

impl JUTAssertion {
    pub fn get_s_device() -> u32 {
        unsafe { jut_assertion_get_s_device() }
    }

    pub fn show_assert(s_device: u32, file: *const u8, line: i32, assertion: *const u8) {
        unsafe { jut_assertion_show_assert(s_device, file, line, assertion) }
    }

    pub fn set_visible(visibility: bool) {
        unsafe { jut_assertion_set_visible(visibility) }
    }
}
