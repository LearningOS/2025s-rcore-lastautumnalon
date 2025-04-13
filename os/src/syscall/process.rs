//! Process management syscalls
use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next,get_syscall_counter},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// TODO: implement the syscall
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {

    if _trace_request == 0 {
        let id = _id as *const u8;
        let ret = unsafe {core::slice::from_raw_parts(id, 1)};
        ret[0] as isize
    } else if _trace_request == 1 {
        let id = _id as *mut u8;
        let area = unsafe {core::slice::from_raw_parts_mut(id, 1)}; // gpt hinted this line.
        area[0] = _data as u8;
        0
    } else if _trace_request == 2{
        get_syscall_counter(_id) as isize
    } else {
        -1
    }
}
