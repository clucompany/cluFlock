
extern crate libc;

use InitFlockLock;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::io;

#[inline]
pub (crate) fn lock_shared<'a, I: InitFlockLock<'a>>(file: &'a File) -> Result<I::Lock, io::Error> {
    flock::<I>(file, libc::LOCK_SH)
}

#[inline]
pub (crate) fn lock_unigue<'a, I: InitFlockLock<'a>>(file: &'a File) -> Result<I::Lock, io::Error> {
    flock::<I>(file, libc::LOCK_EX)
}

//TRY
#[inline]
pub (crate) fn try_lock_shared<'a, I: InitFlockLock<'a>>(file: &'a File) -> Result<Option<I::Lock>, io::Error> {
    try_flock::<I>(file, libc::LOCK_SH | libc::LOCK_NB)
}

#[inline]
pub (crate) fn try_lock_unigue<'a, I: InitFlockLock<'a>>(file: &'a File) -> Result<Option<I::Lock>, io::Error> {
    try_flock::<I>(file, libc::LOCK_EX | libc::LOCK_NB)
}


//TRY

#[inline]
pub fn unlock(file: &File) -> Result<(), io::Error> {
    match unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) } {
        a if a < 0 => Err( io::Error::last_os_error() ),
        _ => Ok( () )
    }
}

#[inline]
fn try_flock<'a, I: InitFlockLock<'a>>(file: &'a File, flag: libc::c_int) -> Result<Option<I::Lock>, io::Error> {
    match unsafe { libc::flock(file.as_raw_fd(), flag) } {
        -1 => Ok( None ),
        a if a < 0 => Err( io::Error::last_os_error() ),
        _ => Ok( Some( I::new(file) ) )
    }
}

#[inline]
fn flock<'a, I: InitFlockLock<'a>>(file: &'a File, flag: libc::c_int) -> Result<I::Lock, io::Error> {
    match unsafe { libc::flock(file.as_raw_fd(), flag) } {
        a if a < 0 => Err( io::Error::last_os_error() ),
        _ => Ok( I::new(file) )
    }
}
