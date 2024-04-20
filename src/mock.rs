use std::ffi::c_void;

use crate::stub::{install_stub, remove_stub, StubError};

#[derive(Debug)]
pub struct Mock {
    fn_ptr: FUNC,
}

#[derive(Debug)]
enum FUNC {
    SyncType(*mut c_void),
    AsyncType(*mut c_void, *mut c_void),
}

impl Drop for Mock {
    fn drop(&mut self) {
        if let Err(err) = self.remove_mock() {
            println!("recover_instruction failed,error is {err:?}");
        }
    }
}

impl Mock {
    pub fn remove_mock(&self) -> Result<(), StubError> {
        match &self.fn_ptr {
            FUNC::SyncType(origin_func_ptr) => { remove_stub(origin_func_ptr) }
            FUNC::AsyncType(origin_func_ptr, poll_func_ptr) => {
                remove_stub(origin_func_ptr)?;
                remove_stub(poll_func_ptr)
            }
        }
    }


    pub fn add_mock(ori_func: *mut c_void, mock_func: *mut c_void) -> Result<Mock, StubError> {
        install_stub(ori_func, mock_func)?;
        Ok(Mock { fn_ptr: FUNC::SyncType(ori_func) })
    }

    pub fn add_async_mock(ori_func: *mut c_void, mock_func: *mut c_void, ori_poll_func: *mut c_void, mock_poll_func: *mut c_void) -> Result<Mock, StubError> {
        install_stub(ori_func, mock_func)?;
        install_stub(ori_poll_func, mock_poll_func)?;

        Ok(Mock { fn_ptr: FUNC::AsyncType(ori_func, ori_poll_func) })
    }
}




