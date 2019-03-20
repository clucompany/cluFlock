
extern crate libc;

use crate::ExclusiveFlockFn;
use crate::SharedFlockFn;
use crate::unlock::UnlockFlock;
use crate::error::FlockError;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use crate::ExclusiveFlock;
use crate::RawConstructorElement;
use crate::FlockLock;
use crate::SharedFlock;
use crate::FlockUnlock;
use std::os::unix::io::RawFd;
use std::io;


pub trait FlockElement {
	fn as_raw_fd(&self) -> RawFd;
}

impl FlockElement for File {
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd {
		AsRawFd::as_raw_fd(self)	
	}
}

impl<A: FlockElement, T> FlockElement for (A, T) {
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd {
		A::as_raw_fd(&self.0)
	}
}

impl<'a> FlockElement for (AsRawFd + 'a) {
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd {
		AsRawFd::as_raw_fd(self)
	}
}

impl<'a, T: FlockElement> FlockElement for &'a T {
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd {
		T::as_raw_fd(self)
	}
}

impl<'a, T: FlockElement> FlockElement for &'a mut T {
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd {
		T::as_raw_fd(self)
	}
}

impl<'a> FlockElement for &'a dyn FlockElement {
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd {
		(**self).as_raw_fd()
	}
}



impl<T> FlockUnlock for T where T: FlockElement {
	fn flock_unlock(&mut self) -> Result<(), io::Error> {
		match unsafe { libc::flock(self.as_raw_fd(), libc::LOCK_UN) } {
			0 => {},
			_ => return Err( io::Error::last_os_error() ),
		}

		Ok( () )
	}
}


impl<T> SharedFlock for T where T: FlockElement {	
	fn try_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> {
		try_flock::<FlockLock<Self>>(self, libc::LOCK_SH | libc::LOCK_NB)
	}
	fn wait_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> {
		wait_flock::<FlockLock<Self>>(self, libc::LOCK_SH)
	}
}

impl<T> ExclusiveFlock for T where T: FlockElement {	
	fn try_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> {
		try_flock::<FlockLock<Self>>(self, libc::LOCK_EX | libc::LOCK_NB)
	}
	fn wait_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> {
		wait_flock::<FlockLock<Self>>(self, libc::LOCK_EX)
	}
}

impl<T> SharedFlockFn for T where T: FlockElement {
	fn try_lock_fn<A: FnOnce(UnlockFlock<T>) -> R, R>(self, f: A) -> Result<R, FlockError<(T, A)>> {
		try_flock::<(Self, A)>((self, f), libc::LOCK_EX | libc::LOCK_NB)
	}
	fn wait_lock_fn<A: FnOnce(UnlockFlock<T>) -> R, R>(self, f: A) -> Result<R, FlockError<(T, A)>> {
		wait_flock::<(Self, A)>((self, f), libc::LOCK_EX)
	}
}

impl<T> ExclusiveFlockFn for T where T: FlockElement {
	fn try_lock_fn<A: FnOnce(UnlockFlock<T>) -> R, R>(self, f: A) -> Result<R, FlockError<(T, A)>> {
		try_flock::<(Self, A)>((self, f), libc::LOCK_EX | libc::LOCK_NB)
	}
	fn wait_lock_fn<A: FnOnce(UnlockFlock<T>) -> R, R>(self, f: A) -> Result<R, FlockError<(T, A)>> {
		wait_flock::<(Self, A)>((self, f), libc::LOCK_EX)
	}
}


#[inline]
fn try_flock<I: RawConstructorElement>(arg: I::Arg, flag: libc::c_int) -> Result<I::ConstResult, FlockError<I::Arg>> {
	match unsafe { libc::flock(arg.as_raw_fd(), flag) } {
		0 => {},
		/*-1 => {
			println!("{:?}", io::Error::last_os_error());
			return Ok( None )
		},*/
		_ => return Err( FlockError::new(arg, io::Error::last_os_error()) ),
	}

	Ok( I::raw_constructor(arg) )
}

#[inline]
fn wait_flock<I: RawConstructorElement>(arg: I::Arg, flag: libc::c_int) -> Result<I::ConstResult, FlockError<I::Arg>> {
	match unsafe { libc::flock(arg.as_raw_fd(), flag) } {
		0 => {},
		_ => return Err( FlockError::new(arg, io::Error::last_os_error()) ),
	}

	Ok( I::raw_constructor(arg) )
}

