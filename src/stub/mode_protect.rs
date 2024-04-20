use libc::{c_int, c_void, mprotect, PROT_EXEC, PROT_READ, PROT_WRITE};

use crate::stub::stub_data::StubError;

const MPROTEXT_RX: c_int = PROT_READ | PROT_EXEC;
const MPROTEXT_RWX: c_int = PROT_READ | PROT_WRITE | PROT_EXEC;

/// Unix `mprotect`.
#[cfg(unix)]
fn _mprotect(ptr: *mut c_void, len: usize, prot: c_int) -> Result<(), StubError> {
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;
    if len > page_size.try_into().unwrap() {
        return Err(StubError::InvalidParameter);
    }

    let ori_ptr = ptr as usize;

    let ptr_start = ori_ptr - ori_ptr % page_size;
    let mut ptr_end = ori_ptr + len;

    if ptr_end % page_size != 0 {
        ptr_end = ori_ptr + page_size - ptr_end % page_size;
    }

    let len = ptr_end - ptr_start;

    unsafe {
        match mprotect(ptr_start as *mut c_void, len, prot) {
            0 => Ok(()),
            ret => Err(StubError::MProtectError(ret))
        }
    }
}

pub fn change_page_rx_mode(origin_func: *mut c_void, inst_len: usize) -> Result<(), StubError> {
    _mprotect(origin_func, inst_len, MPROTEXT_RX)
}

pub fn change_page_rwx_mode(origin_func: *mut c_void, inst_len: usize) -> Result<(), StubError> {
    _mprotect(origin_func, inst_len, MPROTEXT_RWX)
}