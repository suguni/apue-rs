use libc::*;
use std::ffi::CString;

fn main() {
    let login = unsafe {
        let login = getlogin();
        CString::from_raw(login).into_string().unwrap()
    };

    println!("Hello, {}", login);
}
