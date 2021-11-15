use libc;
use std::ffi::c_void;

const BUFFSIZE: usize = 4096;

fn main() {
    unsafe {
        let mut buf = [0_i8; BUFFSIZE];
        let mut n;

        loop {
            n = libc::read(
                libc::STDIN_FILENO,
                buf.as_mut_ptr() as *mut c_void,
                BUFFSIZE,
            );
            if n <= 0 {
                break;
            }

            if libc::write(
                libc::STDOUT_FILENO,
                buf.as_mut_ptr() as *mut c_void,
                n as usize,
            ) != n
            {
                eprintln!("write error");
                libc::exit(1);
            }
        }

        if n < 0 {
            eprintln!("read error");
            libc::exit(1);
        }

        libc::exit(0);
    }
}
