use std::ptr::null_mut;

use libc::{c_int, c_void};

pub const MAX_INSTRUCTION_LEN: usize = 16;

pub struct Stub {
    pub origin_func: *mut c_void,
    pub mock_func: *mut c_void,
    pub origin_instruction: [u8; MAX_INSTRUCTION_LEN],
    pub stub: bool,
}

#[derive(Debug)]
pub enum StubError {
    NullPointer,
    SamePointer,
    RepeatedlyStub,
    MProtectError(c_int),
}


impl Default for Stub {
    fn default() -> Self {
        Stub {
            origin_func: null_mut(),
            mock_func: null_mut(),
            origin_instruction: [0x0; MAX_INSTRUCTION_LEN],
            stub: false,
        }
    }
}
