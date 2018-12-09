
extern crate libc;

mod raw_fd;
pub use self::raw_fd::*;

use crate::raw::RawConstFlock;
use std::io;




#[inline(always)]
pub (crate) fn wait_lock_shared<'a, I: RawConstFlock<'a>>(arg: I::Arg) -> Result<I::Lock, io::Error> where I::Arg: UnixRawFd {
    wait_flock::<I>(arg, libc::LOCK_SH)
}

#[inline(always)]
pub (crate) fn wait_lock_exclusive<'a, I: RawConstFlock<'a>>(arg: I::Arg) -> Result<I::Lock, io::Error> where I::Arg: UnixRawFd {
    wait_flock::<I>(arg, libc::LOCK_EX)
}

//TRY
#[inline(always)]
pub (crate) fn try_lock_shared<'a, I: RawConstFlock<'a>>(arg: I::Arg) -> Result<I::Lock, io::Error> where I::Arg: UnixRawFd {
    try_flock::<I>(arg, libc::LOCK_SH | libc::LOCK_NB)
}

#[inline(always)]
pub (crate) fn try_lock_exclusive<'a, I: RawConstFlock<'a>>(arg: I::Arg) -> Result<I::Lock, io::Error> where I::Arg: UnixRawFd {
    try_flock::<I>(arg, libc::LOCK_EX | libc::LOCK_NB)
}


//TRY

#[inline(always)]
pub (crate) fn unlock<I: UnixRawFd>(fd: I) -> Result<(), io::Error> {
    match unsafe { libc::flock(fd.as_raw_fd(), libc::LOCK_UN) } {
        0 => {},
        _ => return Err( io::Error::last_os_error() ),
    }

    Ok( () )
}

#[inline(always)]
fn try_flock<'a, I: RawConstFlock<'a>>(arg: I::Arg, flag: libc::c_int) -> Result<I::Lock, io::Error> where I::Arg: UnixRawFd {
    match unsafe { libc::flock(arg.as_raw_fd(), flag) } {
        0 => {},
        /*-1 => {
            println!("{:?}", io::Error::last_os_error());
            return Ok( None )
        },*/
        _ => return Err( io::Error::last_os_error() ),
    }

    Ok( I::new(arg) )
}

#[inline(always)]
fn wait_flock<'a, I: RawConstFlock<'a>>(arg: I::Arg, flag: libc::c_int) -> Result<I::Lock, io::Error> where I::Arg: UnixRawFd {
    match unsafe { libc::flock(arg.as_raw_fd(), flag) } {
        0 => {},
        _ => return Err( io::Error::last_os_error() ),
    }

    Ok( I::new(arg) )
}
