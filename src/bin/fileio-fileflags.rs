use libc;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("usage: {} <descriptor#>", &args[0]);
        unsafe { libc::exit(1) }
    }

    let fd: i32 = i32::from_str_radix(&args[1], 10).unwrap();

    unsafe {
        let val = libc::fcntl(fd, libc::F_GETFL, 0);
        if val < 0 {
            eprintln!("fcntl error for fd {}", fd);
            libc::exit(1);
        }

        match val & libc::O_ACCMODE {
            libc::O_RDONLY => {
                print!("read only");
            }
            libc::O_WRONLY => {
                print!("write only");
            }
            libc::O_RDWR => {
                print!("read write");
            }
            _ => {
                eprintln!("unknown access mode");
                libc::exit(1);
            }
        }

        if val & libc::O_APPEND != 0 {
            print!(", append");
        }

        if val & libc::O_NONBLOCK != 0 {
            print!(", nonblocking");
        }

        if val & libc::O_SYNC != 0 {
            print!(", synchronous writes");
        }

        if val & libc::O_FSYNC != 0 {
            print!(", fsynchronous writes");
        }

        println!();
        libc::exit(1);
    }
}
