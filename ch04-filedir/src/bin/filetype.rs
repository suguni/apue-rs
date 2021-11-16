#![allow(non_snake_case)]

use libc;
use std::env;
use std::ffi::CString;
use std::mem::MaybeUninit;

fn main() {
    let args: Vec<String> = {
        let mut args = env::args();
        args.next();
        args.collect()
    };

    unsafe {
        for arg in args {
            let filename = arg.clone();
            let argv = CString::new(arg).unwrap();
            let mut buf = MaybeUninit::<libc::stat>::uninit();

            let ret = libc::lstat(argv.as_ptr(), buf.as_mut_ptr());

            if ret < 0 {
                eprintln!("lstat error");
                continue;
            }

            let buf = buf.assume_init();

            let ptr = if S_ISREG(buf.st_mode) {
                "regular"
            } else if S_ISDIR(buf.st_mode) {
                "directory"
            } else if S_ISCHR(buf.st_mode) {
                "character special"
            } else if S_ISBLK(buf.st_mode) {
                "block special"
            } else if S_ISFIFO(buf.st_mode) {
                "fifo"
            } else if S_ISLNK(buf.st_mode) {
                "symbolic link"
            } else if S_ISSOCK(buf.st_mode) {
                "socket"
            } else {
                "** unknown mode **"
            };

            println!("{}: {}", filename, ptr);
        }
    }
}

fn S_ISREG(st_mode: u16) -> bool {
    st_mode & libc::S_IFMT == libc::S_IFREG
}

fn S_ISDIR(st_mode: u16) -> bool {
    st_mode & libc::S_IFMT == libc::S_IFDIR
}

fn S_ISCHR(st_mode: u16) -> bool {
    st_mode & libc::S_IFMT == libc::S_IFCHR
}

fn S_ISBLK(st_mode: u16) -> bool {
    st_mode & libc::S_IFMT == libc::S_IFBLK
}

fn S_ISFIFO(st_mode: u16) -> bool {
    st_mode & libc::S_IFMT == libc::S_IFIFO
}

fn S_ISLNK(st_mode: u16) -> bool {
    st_mode & libc::S_IFMT == libc::S_IFLNK
}

fn S_ISSOCK(st_mode: u16) -> bool {
    st_mode & libc::S_IFMT == libc::S_IFSOCK
}
