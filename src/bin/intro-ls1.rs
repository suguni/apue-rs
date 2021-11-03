use libc;
use std::env;
use std::ffi;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("usage: {} directory_name", args[0]);
        return;
    }

    let dir = &args[1];

    let dp = unsafe {
        let dir = ffi::CString::new(dir.as_str()).expect("invalid dir");
        libc::opendir(dir.as_ptr())
    };

    if dp.is_null() {
        eprintln!("can't open {}", dir);
        return;
    }

    loop {
        let dirp = unsafe { libc::readdir(dp) };

        if dirp.is_null() {
            break;
        }

        let d_name = unsafe {
            ffi::CStr::from_ptr((*dirp).d_name.as_ptr())
                .to_str()
                .unwrap()
        };

        println!("{}", d_name);
    }

    unsafe {
        libc::closedir(dp);
    };
}
