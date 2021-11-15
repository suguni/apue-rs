use libc;
use std::env;
use std::ffi::{c_void, CString};
use std::path::Path;

const BUFFSIZE: usize = 4096;

fn main() {
    unsafe {
        let mut args = env::args();

        if args.len() != 3 {
            eprintln!("usage: bbcp source target");
            libc::exit(1);
        }

        args.next();

        let input = args.next().unwrap();
        let output = args.next().unwrap();

        let input_cstr = CString::new(input.clone()).unwrap();

        let fd_in = libc::open(input_cstr.as_ptr(), libc::O_RDONLY);
        if fd_in < 0 {
            eprintln!("open error: {}", input);
            libc::exit(1);
        }

        let mut output_cstr = CString::new(output.clone()).unwrap();
        let fd = libc::open(output_cstr.as_ptr(), libc::O_RDONLY | libc::O_DIRECTORY);
        if fd >= 0 {
            output_cstr = CString::new(format!(
                "{}/{}",
                output,
                Path::new(&input).file_name().unwrap().to_str().unwrap()
            ))
            .unwrap();
        }
        libc::close(fd);

        let fd_out = libc::open(
            output_cstr.as_ptr(),
            libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL,
        );
        if fd_out < 0 {
            eprintln!("open error: {}", output);
        }

        let mut buf = [0_i8; BUFFSIZE];
        let mut n;

        loop {
            n = libc::read(fd_in, buf.as_mut_ptr() as *mut c_void, BUFFSIZE);
            if n <= 0 {
                break;
            }

            if libc::write(fd_out, buf.as_mut_ptr() as *mut c_void, n as usize) != n {
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
