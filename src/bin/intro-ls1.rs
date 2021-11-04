use libc;
use std::env;
use std::ffi;

fn main() {
    unsafe {
        let args = env::args().collect::<Vec<String>>();
        if args.len() < 2 {
            eprintln!("usage: {} directory_name", args[0]);
            libc::exit(1);
        }

        let dir = &args[1];

        let cdir = ffi::CString::new(dir.as_str()).expect("invalid dir");
        let dp = libc::opendir(cdir.as_ptr());

        if dp.is_null() {
            eprintln!("can't open {}", dir);
            libc::exit(1);
        }

        loop {
            let dirp = libc::readdir(dp);

            if dirp.is_null() {
                break;
            }

            let d_name = ffi::CStr::from_ptr((*dirp).d_name.as_ptr())
                .to_str()
                .unwrap();

            println!("{}", d_name);
        }

        libc::closedir(dp);
        libc::exit(0);
    }
}
