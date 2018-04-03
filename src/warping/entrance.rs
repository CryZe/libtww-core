use system::memory::{read_str, reference};

#[repr(C, packed)]
pub struct Entrance {
    pub stage: [u8; 8],
    pub entrance: u16,
    pub room: u8,
}

impl Clone for Entrance {
    fn clone(&self) -> Self {
        Entrance {
            stage: self.stage,
            entrance: self.entrance,
            room: self.room,
        }
    }
}

impl Entrance {
    pub fn last_entrance() -> &'static mut Entrance {
        reference(0x803BD23C)
    }

    pub fn stage_name(&self) -> &'static str {
        read_str(self.stage.as_ptr())
    }
}
