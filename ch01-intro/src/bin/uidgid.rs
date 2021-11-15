use libc;

fn main() {
    unsafe {
        println!("uid = {}, gid = {}", libc::getuid(), libc::getgid());
        libc::exit(0);
    }
}
