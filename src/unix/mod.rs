
extern crate libc;

use InitFlockLock;
use std::io;

mod raw_fd;
pub use self::raw_fd::*;

#[inline]
pub (crate) fn lock_shared<'a, I: InitFlockLock<'a>>(arg: I::Arg) -> Result<I::Lock, io::Error> where I::Arg: UnixRawFd {
    flock::<I>(arg, libc::LOCK_SH)
}

#[inline]
pub (crate) fn lock_unigue<'a, I: InitFlockLock<'a>>(arg: I::Arg) -> Result<I::Lock, io::Error> where I::Arg: UnixRawFd {
    flock::<I>(arg, libc::LOCK_EX)
}

//TRY
#[inline]
pub (crate) fn try_lock_shared<'a, I: InitFlockLock<'a>>(arg: I::Arg) -> Result<Option<I::Lock>, io::Error> where I::Arg: UnixRawFd {
    try_flock::<I>(arg, libc::LOCK_SH | libc::LOCK_NB)
}

#[inline]
pub (crate) fn try_lock_unigue<'a, I: InitFlockLock<'a>>(arg: I::Arg) -> Result<Option<I::Lock>, io::Error> where I::Arg: UnixRawFd {
    try_flock::<I>(arg, libc::LOCK_EX | libc::LOCK_NB)
}


//TRY

#[inline]
pub (crate) fn unlock<I: UnixRawFd>(file: I) -> Result<(), io::Error> {
    match unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) } {
        a if a < 0 => Err( io::Error::last_os_error() ),
        _ => Ok( () )
    }
}

#[inline]
fn try_flock<'a, I: InitFlockLock<'a>>(arg: I::Arg, flag: libc::c_int) -> Result<Option<I::Lock>, io::Error> where I::Arg: UnixRawFd {
    match unsafe { libc::flock(arg.as_raw_fd(), flag) } {
        -1 => Ok( None ),
        a if a < 0 => Err( io::Error::last_os_error() ),
        _ => Ok( Some( I::new(arg) ) )
    }
}

#[inline]
fn flock<'a, I: InitFlockLock<'a>>(arg: I::Arg, flag: libc::c_int) -> Result<I::Lock, io::Error> where I::Arg: UnixRawFd {
    match unsafe { libc::flock(arg.as_raw_fd(), flag) } {
        a if a < 0 => Err( io::Error::last_os_error() ),
        _ => Ok( I::new(arg) )
    }
}
