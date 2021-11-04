use apue_rs::*;
use libc;
use libc_stdhandle::*;
use std::ffi::CStr;
use std::io::{self, Write};

fn main() {
    unsafe {
        let buf = [0_i8; MAXLINE].as_mut_ptr();
        let mut pid: libc::pid_t;
        let mut status: i32 = 0;

        print!(">> ");
        io::stdout().flush().unwrap();

        loop {
            let r = libc::fgets(buf, MAXLINE as i32, stdin());
            if r.is_null() {
                break;
            }

            let l = libc::strlen(buf) as isize - 1;
            if *buf.offset(l) as u8 as char == '\n' {
                *buf.offset(l) = 0;
            }

            pid = libc::fork();
            if pid < 0 {
                eprintln!("fork error");
                libc::exit(1);
            } else if pid == 0 {
                libc::execlp(buf, buf, 0 as *const char);
                eprintln!(
                    "counldn't execute: {}",
                    CStr::from_ptr(buf).to_str().unwrap()
                );
                libc::exit(127);
            }

            // parent
            pid = libc::waitpid(pid, &mut status, 0);
            if pid < 0 {
                eprintln!("waitpid error");
                libc::exit(1);
            }

            print!(">> ");
            io::stdout().flush().unwrap();
        }

        libc::exit(0);
    }
}
