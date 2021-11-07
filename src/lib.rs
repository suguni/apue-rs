use libc::*;

pub const MAXLINE: usize = 4098;

/*
 * Default file access permissions for new files.
 */
pub const FILE_MODE: mode_t = S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH;

/*
 * Default permissions for new directories.
 */
pub const DIR_MODE: mode_t = FILE_MODE | S_IXUSR | S_IXGRP | S_IXOTH;

// fig 3.12 Turn on one or more of the file status flags for a descriptor
pub fn set_fl(fd: i32, flags: u32) -> Result<(), String> {
    unsafe {
        let mut val = libc::fcntl(fd, libc::F_GETFL, 0);
        if val < 0 {
            return Err("fcntl F_GETFL error".to_string());
        }

        val = libc::fcntl(fd, libc::F_SETFL, val as u32 | flags);
        if val < 0 {
            return Err("fcntl F_SETFL error".to_string());
        }
    }
    Ok(())
}
