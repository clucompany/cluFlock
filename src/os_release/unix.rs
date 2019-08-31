
use crate::err::FlockFnError;
use crate::FlockFnBuilder;
use crate::SafeUnlockFlock;
use crate::err::FlockError;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use crate::ExclusiveFlock;
use crate::BehOsRelease;
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

impl<D, F, Fr> FlockElement for FlockFnBuilder<D, F, Fr> where D: FlockElement, F: FnOnce(SafeUnlockFlock<D>) -> Fr {
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd {
		self.data.as_raw_fd()
	}
}

impl<'a> FlockElement for (dyn AsRawFd + 'a) {
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd {
		AsRawFd::as_raw_fd(self)
	}
}

impl<'a, 'l, T: 'l> FlockElement for &'a T where T: FlockElement {
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd {
		T::as_raw_fd(self)
	}
}

impl<'a, 'l, T: 'l> FlockElement for &'a mut T where T: FlockElement {
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd {
		T::as_raw_fd(self)
	}
}

/*impl<'a, 'l> FlockElement for &'a (dyn FlockElement + 'l) {
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd {
		(**self).as_raw_fd()
	}
}*/



impl<T> FlockUnlock for T where T: FlockElement {
	type UnlockResult = ();
	
	unsafe fn flock_unlock_no_result(&mut self) {
		#[allow(unused_unsafe)]
		unsafe { libc::flock(self.as_raw_fd(), libc::LOCK_UN); }
	}
	
	unsafe fn flock_unlock(&mut self) -> Result<(), io::Error> {
		#[allow(unused_unsafe)]
		match unsafe { libc::flock(self.as_raw_fd(), libc::LOCK_UN) } {
			0 => Ok( () ),
			_ => Err( io::Error::last_os_error() ),
		}
	}
}


impl<T> SharedFlock for T where T: FlockElement {	
	fn try_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> {
		try_flock::<FlockLock<Self>>(self, libc::LOCK_SH | libc::LOCK_NB)
	}
	fn wait_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> {
		wait_flock::<FlockLock<Self>>(self, libc::LOCK_SH)
	}
	
	fn try_lock_fn<Fn: FnOnce(SafeUnlockFlock<T>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> {
		try_flock::<FlockFnBuilder<Self, Fn, Fr>>(FlockFnBuilder::new(self, f), libc::LOCK_EX | libc::LOCK_NB)
	}
	fn wait_lock_fn<Fn: FnOnce(SafeUnlockFlock<T>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> {
		wait_flock::<FlockFnBuilder<Self, Fn, Fr>>(FlockFnBuilder::new(self, f), libc::LOCK_EX)
	}
}

impl<T> ExclusiveFlock for T where T: FlockElement {
	fn try_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> {
		try_flock::<FlockLock<Self>>(self, libc::LOCK_EX | libc::LOCK_NB)
	}
	fn wait_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> {
		wait_flock::<FlockLock<Self>>(self, libc::LOCK_EX)
	}
	
	fn try_lock_fn<Fn: FnOnce(SafeUnlockFlock<T>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> {
		try_flock::<FlockFnBuilder<Self, Fn, Fr>>(FlockFnBuilder::new(self, f), libc::LOCK_EX | libc::LOCK_NB)
	}
	fn wait_lock_fn<Fn: FnOnce(SafeUnlockFlock<T>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> {
		wait_flock::<FlockFnBuilder<Self, Fn, Fr>>(FlockFnBuilder::new(self, f), libc::LOCK_EX)
	}
}



#[inline]
fn try_flock<I: BehOsRelease>(arg: I::Data, flag: libc::c_int) -> Result<I::Ok, I::Err> where I::Data : FlockElement {
	match unsafe { libc::flock(arg.as_raw_fd(), flag) } {
		0 => Ok( I::ok(arg) ),
		/*-1 => {
			println!("{:?}", io::Error::last_os_error());
			return Ok( None )
		},*/
		_ => Err( I::err(arg, io::Error::last_os_error()) ),
	}

	
}

#[inline]
fn wait_flock<I: BehOsRelease>(arg: I::Data, flag: libc::c_int) -> Result<I::Ok, I::Err> where I::Data : FlockElement {
	match unsafe { libc::flock(arg.as_raw_fd(), flag) } {
		0 => Ok( I::ok(arg) ),
		_ => Err( I::err(arg, io::Error::last_os_error()) ),
	}
}

