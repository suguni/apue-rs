use libc;

const BUFSIZE: usize = 4096;

fn main() {
    unsafe {
        let mut n;

        let buf = [0_u8; BUFSIZE].as_mut_ptr() as *mut std::ffi::c_void;

        loop {
            n = libc::read(libc::STDIN_FILENO, buf, BUFSIZE);
            if n <= 0 {
                break;
            }

            if libc::write(libc::STDOUT_FILENO, buf, n as usize) != n {
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
