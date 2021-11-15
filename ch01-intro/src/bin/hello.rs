use libc;

fn main() {
    unsafe {
        println!("hello world from process ID {}", libc::getpid());
        libc::exit(0);
    }
}
