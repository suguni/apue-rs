use libc;
use libc_stdhandle::*;
use std::env;
use std::ffi::CString;

fn main() {
    unsafe {
        let msg = CString::new("EACCES: %s\n").unwrap();
        libc::fprintf(stderr(), msg.as_ptr(), libc::strerror(libc::EACCES));

        let errno = libc::__error();
        *errno = libc::ENOENT;

        let msg = CString::new(env::args().next().unwrap()).unwrap();
        libc::perror(msg.as_ptr());

        libc::exit(0);
    }
}
