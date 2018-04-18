//! Based on http://www.gc-forever.com/yagcd/chap13.html#sec13
//! and https://github.com/LordNed/WArchive-Tools

pub mod reader;
pub mod virtual_file_system;
pub mod writer;

pub mod consts {
    // DOL_ALIGNMENT and FST_ALIGNMENT are set to 1024 and 256 to match the
    // original ISO. Due to poor documentation of how, and why, these values
    // should or shouldn't be changed we opted to preserve their values since
    // there was no observed benefit of setting them higher, however lower
    // values were not tested.

    pub const OFFSET_DOL_OFFSET: usize = 0x420;
    pub const OFFSET_FST_OFFSET: usize = 0x424;
    pub const OFFSET_FST_SIZE: usize = 0x428;
    pub const HEADER_LENGTH: usize = 0x2440;
    pub const DOL_ALIGNMENT: usize = 1024;
    pub const FST_ALIGNMENT: usize = 256;
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
enum FstNodeType {
    File = 0,
    Directory = 1,
}

impl Default for FstNodeType {
    fn default() -> Self {
        FstNodeType::File
    }
}

#[derive(Clone, Default)]
struct FstEntry<'a> {
    kind: FstNodeType,
    relative_file_name: &'a str,
    file_offset_parent_dir: usize,
    file_size_next_dir_index: usize,
    file_name_offset: usize,
}
