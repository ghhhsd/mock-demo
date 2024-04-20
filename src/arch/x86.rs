use std::usize;

use libc::{c_uchar, c_uint, c_ulong, c_void};

const JUM_LEN: usize = 5;
const LONG_JUM_LEN: usize = 14;

pub fn replace_instruction(origin_fun: *mut c_void, stub_fun: *mut c_void) {
    let instruction_len = get_instruction_len(origin_fun, stub_fun);


    match instruction_len == JUM_LEN {
        true => {
            let mut instruction = [0x0; JUM_LEN];
            let offset = get_offset(origin_fun, stub_fun);

            instruction[0] = 0xe9;
            instruction[1..].copy_from_slice(&(offset as c_uint).to_ne_bytes());
            unsafe {
                *{ origin_fun as *mut [c_uchar; JUM_LEN] } = instruction;
            }
        }
        false => {
            let mut instruction = [0x0; LONG_JUM_LEN];
            instruction[0] = 0xeb;
            instruction[1] = 0xff;
            instruction[2] = 0x25;
            instruction[6..].copy_from_slice(&(stub_fun as c_ulong).to_ne_bytes());
            unsafe {
                *{ origin_fun as *mut [c_uchar; LONG_JUM_LEN] } = instruction;
            }
        }
    }
}

fn get_offset(origin_fun: *mut c_void, stub_fun: *mut c_void) -> usize {
    (stub_fun as usize).wrapping_sub(origin_fun as usize) - JUM_LEN
}

pub fn get_instruction_len(origin_fun: *mut c_void, stub_fun: *mut c_void) -> usize {
    let offset = get_offset(origin_fun, stub_fun);

    let mut mask = usize::MAX;

    mask <<= 32;

    if (mask & offset == 0) || (mask & offset == offset) {
        return JUM_LEN;
    }

    LONG_JUM_LEN
}

pub fn cache_flush(_origin_fun: *mut c_void, _len: usize) {}