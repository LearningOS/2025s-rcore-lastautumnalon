//! Process management syscalls
use crate::{
    mm::{ rw_byte, translated_byte_buffer}, task::{change_program_brk, current_user_token, exit_current_and_run_next, get_syscall_counter, suspend_current_and_run_next}, timer::get_time_us
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let tv = TimeVal {
        sec: us/1_000_000,
        usec: us % 1_000_000,
    };
    let buffers = translated_byte_buffer(current_user_token(), _ts as *const u8, 16);

    let tv_bytes = unsafe {
        core::slice::from_raw_parts((&tv as *const TimeVal) as *const u8, 16)
    };

    let mut written = 0;
    for buf in buffers{
        let len = buf.len();
        buf.copy_from_slice(&tv_bytes[written..written+len]);
        written += len;
    }

    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    if _trace_request == 0 {
        let id = _id as *const u8;
        let ans = rw_byte(current_user_token(), id, 'r', _data);
        if ans.is_none() {
            return -1;
        } else {
            return ans.unwrap() as isize;
        }
    } else if _trace_request == 1 {
        let id = _id as *const u8;
        let ans = rw_byte(current_user_token(), id, 'w', _data);
        if ans.is_none() {
            return -1;
        } else {
            return 0;
        }
    } else if _trace_request == 2{
        return get_syscall_counter(_id) as isize;
    } else {
        -1
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _prot: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    0
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
