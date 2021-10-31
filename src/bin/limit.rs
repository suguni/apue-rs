use libc;
use std::env;
use std::ffi;

fn main() {
    pr_sysconf("ARG_MAX = ", libc::_SC_ARG_MAX);
    pr_sysconf("ATEXIT_MAX = ", libc::_SC_ATEXIT_MAX);
    pr_sysconf("CHILD_MAX = ", libc::_SC_CHILD_MAX);
}

fn pr_sysconf(msg: &str, name: i32) {
    let val = unsafe { libc::sysconf(name) };
    if val < 0 {
        let errno = errno();
        if errno != 0 {
            if errno == libc::EINVAL {
                eprintln!("{} (not supported)", msg);
            } else {
                eprintln!("sysconf error");
            }
        } else {
            println!("{} (no limit)", msg);
        }
    } else {
        println!("{}{}", msg, val);
    }
}

#[cfg(target_os = "macos")]
fn errno() -> i32 {
    unsafe { *libc::__error() }
}

#[cfg(target_os = "linux")]
fn errno() -> i32 {
    unsafe { *libc::__errno_location() }
}
