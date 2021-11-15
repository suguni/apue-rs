use libc;
use libc_stdhandle::*;

fn main() {
    std::io::stdin();

    unsafe {
        let mut c;

        loop {
            c = libc::fgetc(stdin());

            if c == libc::EOF {
                break;
            }

            if libc::fputc(c, stdout()) == libc::EOF {
                eprintln!("output error");
                libc::exit(1);
            }
        }

        if libc::ferror(stdin()) != 0 {
            eprintln!("input error");
            libc::exit(1);
        }

        libc::exit(0);
    }
}
