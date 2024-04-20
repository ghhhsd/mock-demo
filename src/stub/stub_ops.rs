use std::ffi::c_void;
use std::ptr;

use crate::arch::{cache_flush, get_instruction_len, replace_instruction};
use crate::stub::lock::get_lock;
use crate::stub::mode_protect::{change_page_rwx_mode, change_page_rx_mode};
use crate::stub::stub_data::{MAX_INSTRUCTION_LEN, Stub, StubError};

static mut MOCK_STUBS: Vec<Stub> = Vec::new();

pub fn install_stub(origin_func: *mut c_void, mock_func: *mut c_void) -> Result<(), StubError> {
    if origin_func.is_null() || mock_func.is_null() {
        return Err(StubError::NullPointer);
    }

    if origin_func == mock_func {
        return Err(StubError::SamePointer);
    }


    unsafe {
        if MOCK_STUBS.iter().find(|stub| origin_func.eq(&stub.origin_func) && stub.stub).is_some() {
            return Err(StubError::RepeatedlyStub);
        }
    }

    let lock = get_lock().as_ref().expect("get lock").lock().expect("set lock");

    let mut stub = Stub {
        origin_func,
        mock_func,
        stub: true,
        origin_instruction: [0x0; MAX_INSTRUCTION_LEN],
    };


    if let Err(err) = make_stub(&mut stub) {
        drop(lock);
        println!("make_stub failed,error is {:?}", err);
        return Err(err);
    }

    unsafe {
        MOCK_STUBS.push(stub);
    }


    Ok(())
}

fn make_stub(stub: &mut Stub) -> Result<(), StubError> {
    let instruction_len = get_instruction_len(stub.origin_func, stub.mock_func);
    change_page_rwx_mode(stub.origin_func, instruction_len)?;
    unsafe { ptr::copy(stub.origin_func, stub.origin_instruction.as_mut_ptr() as *mut c_void, instruction_len); }

    replace_instruction(stub.origin_func, stub.mock_func);
    change_page_rx_mode(stub.origin_func, instruction_len)
}


pub fn remove_stub(origin_func: &*mut c_void) -> Result<(), StubError> {
    if origin_func.is_null() {
        return Err(StubError::NullPointer);
    }


    unsafe {
        if let Some(stub) = MOCK_STUBS.iter_mut().find(|stub| origin_func.eq(&stub.origin_func) & stub.stub) {
            let instruction_len = get_instruction_len(stub.origin_func, stub.mock_func);

            change_page_rwx_mode(stub.origin_func, instruction_len)?;
            ptr::copy(stub.origin_instruction.as_mut_ptr() as *mut c_void, stub.origin_func, instruction_len);
            change_page_rx_mode(stub.origin_func, instruction_len)?;
            cache_flush(stub.origin_func, instruction_len);
        }
    }


    Ok(())
}
