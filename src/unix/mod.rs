


use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::io;

#[inline]
pub fn lock_shared(file: &File) -> Result<(), io::Error> {
    flock(file, libc::LOCK_SH)
}

#[inline]
pub fn lock_unigue(file: &File) -> Result<(), io::Error> {
    flock(file, libc::LOCK_EX)
}

//TRY
#[inline]
pub fn try_lock_shared(file: &File) -> Result<(), io::Error> {
    flock(file, libc::LOCK_SH | libc::LOCK_NB)
}

#[inline]
pub fn try_lock_unigue(file: &File) -> Result<(), io::Error> {
    flock(file, libc::LOCK_EX | libc::LOCK_NB)
}
//TRY

#[inline]
pub fn unlock(file: &File) -> Result<(), io::Error> {
    flock(file, libc::LOCK_UN)
}

#[inline]
pub fn flock(file: &File, flag: libc::c_int) -> Result<(), io::Error> {
    match unsafe { libc::flock(file.as_raw_fd(), flag) } {
        a if a < 0 => Err( io::Error::last_os_error() ),
        _ => Ok( () )
    }
}
