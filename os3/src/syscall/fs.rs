//! File and filesystem-related syscalls

use crate::task::set_write_count;

const FD_STDOUT: usize = 1;

// YOUR JOB: 修改 sys_write 使之通过测试
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            set_write_count();
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}