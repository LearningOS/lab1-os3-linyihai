//! Process management syscalls

use crate::config::{MAX_APP_NUM, MAX_SYSCALL_NUM};
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, task_info, set_get_time_count};
use crate::timer::{get_time_us, get_time};
use crate::syscall::{SYSCALL_TASK_INFO, SYSCALL_WRITE, SYSCALL_YIELD, SYSCALL_EXIT, SYSCALL_GET_TIME};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub struct TaskInfo {
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    set_get_time_count();
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let (gettime_count, taskinfo_count, write_count, yield_count, exit_count, time) = task_info();
    unsafe {
        (*ti).syscall_times[SYSCALL_GET_TIME] = gettime_count;
        (*ti).syscall_times[SYSCALL_TASK_INFO] = taskinfo_count;
        (*ti).time = time;
        (*ti).syscall_times[SYSCALL_WRITE] = write_count;
        (*ti).syscall_times[SYSCALL_EXIT] = exit_count;
        (*ti).syscall_times[SYSCALL_YIELD] = yield_count;
        (*ti).status = TaskStatus::Running;
    }
    return 0
}
