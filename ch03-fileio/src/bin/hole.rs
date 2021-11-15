use common::FILE_MODE;
use libc;
use std::ffi::{c_void, CString};

const BUF1: *const u8 = "abcdefghij".as_bytes().as_ptr();

const BUF2: *const u8 = "ABCDEFGHIJ".as_bytes().as_ptr();

fn main() {
    unsafe {
        let filename = CString::new("file.hole").unwrap();
        let fd = libc::creat(filename.as_ptr() as *const i8, FILE_MODE);
        if fd < 0 {
            eprintln!("creat error");
            libc::exit(1);
        }

        if libc::write(fd, BUF1 as *const c_void, 10) != 10 {
            eprintln!("BUF1 write error");
            libc::exit(1);
        }

        if libc::lseek(fd, 16384, libc::SEEK_SET) == -1 {
            eprintln!("lseek error");
            libc::exit(1);
        }

        if libc::write(fd, BUF2 as *const c_void, 10) != 10 {
            eprintln!("BUF2 write error");
            libc::exit(1);
        }

        libc::exit(0);
    }
}

/*
확인
od -c file.hole
 */
