use std::arch::asm;
use std::mem::size_of;

use libc::{c_uchar, c_uint, c_ulong, c_void};

// LDR X16, [PC + #8]
const LDR_X16_NEXT2: c_uchar = 0x58000050;

// BR X16
const BR_X16: c_uchar = 0xD61F0200;


const B_JUM_LEN: usize = 4;
const DIRECT_JUMP_LEN: usize = 16;


pub fn replace_instruction(origin_fun: *mut c_void, stub_fun: *mut c_void) {
    let instruction_len = get_instruction_len(origin_fun, stub_fun);
    if B_JUM_LEN == inst_len {
        let offset = (origin_fun as c_ulong).wrapping_sub(stub_fun as c_ulong);
        offset >>= 2_i32;
        let diff = offset & 0x3FFFFFF;
        unsafe {
            origin_fun = 0x14000000 | diff;
        }
    } else {
        unsafe {
            origin_fun = LDR_X16_NEXT2;
            origin_fun = origin_fun.add(size_of(c_uint));
            origin_fun = BR_X16;
            origin_fun = origin_fun.add(size_of(c_ulong));
            origin_fun = stub_fun as *mut c_ulong;
        }
    }
}

fn get_offset(origin_fun: *mut c_void, stub_fun: *mut c_void) -> usize {
    (stub_fun as usize).wrapping_sub(origin_fun as usize)
}

pub fn get_instruction_len(origin_fun: *mut c_void, stub_fun: *mut c_void) -> usize {
    let offset = get_offset(origin_fun, stub_fun);

    // B imm with +- 128MB offset
    if offset >> 26 == 5 {
        return B_JUM_LEN;
    }


    DIRECT_JUMP_LEN
}


pub fn cache_flush(origin_fun: *mut c_void, len: usize) {
    for idx in 0..len {
        unsafe {
            let tmp_ptr = origin_fun.add(idx);
            cache(tmp_ptr)
        }
    }
}

fn cache(tmp_ptr: *mut c_void) {
    unsafe {
        asm!(
        " mov {tmp}, {res} \n",
        " ic IVAU, {tmp} ",
        res = inout(reg) tmp_ptr,
        tmp = out(reg) _,
        )
    }
}