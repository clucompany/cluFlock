
use crate::data::WaitFlockUnlock;
use crate::data::TryFlockUnlock;
use crate::element::FlockElement;
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
use std::os::unix::io::RawFd;
use std::io;


impl FlockElement for File {
	type FilePtr = RawFd;
	
	#[inline(always)]
	fn as_file_ptr(&self) -> Self::FilePtr {
		AsRawFd::as_raw_fd(self)
	}
}


impl<T> TryFlockUnlock for T where T: FlockElement<FilePtr = RawFd> {
	type UnlockResult = ();
	
	unsafe fn unlock_no_result(&mut self) {
		#[allow(unused_unsafe)]
		unsafe { libc::flock(self.as_file_ptr(), libc::LOCK_UN | libc::LOCK_NB); }
	}
	
	unsafe fn unlock(&mut self) -> Result<(), io::Error> {
		#[allow(unused_unsafe)]
		match unsafe { libc::flock(self.as_file_ptr(), libc::LOCK_UN | libc::LOCK_NB) } {
			0 => Ok( () ),
			_ => Err( io::Error::last_os_error() ),
		}
	}
}

impl<T> WaitFlockUnlock for T where T: FlockElement<FilePtr = RawFd> {
	type UnlockResult = ();

	unsafe fn unlock_no_result(&mut self) {
		#[allow(unused_unsafe)]
		unsafe { libc::flock(self.as_file_ptr(), libc::LOCK_UN); }
	}
	
	unsafe fn unlock(&mut self) -> Result<(), io::Error> {
		#[allow(unused_unsafe)]
		match unsafe { libc::flock(self.as_file_ptr(), libc::LOCK_UN) } {
			0 => Ok( () ),
			_ => Err( io::Error::last_os_error() ),
		}
	}
}


impl<T> SharedFlock for T where T: FlockElement<FilePtr = RawFd> {	
	fn try_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> {
		flock::<FlockLock<Self>>(self, libc::LOCK_SH | libc::LOCK_NB)
	}
	fn wait_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> {
		flock::<FlockLock<Self>>(self, libc::LOCK_SH)
	}
	
	fn try_lock_fn<Fn: FnOnce(SafeUnlockFlock<T>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> {
		flock::<FlockFnBuilder<Self, Fn, Fr>>(FlockFnBuilder::new(self, f), libc::LOCK_EX | libc::LOCK_NB)
	}
	fn wait_lock_fn<Fn: FnOnce(SafeUnlockFlock<T>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> {
		flock::<FlockFnBuilder<Self, Fn, Fr>>(FlockFnBuilder::new(self, f), libc::LOCK_EX)
	}
}

impl<T> ExclusiveFlock for T where T: FlockElement<FilePtr = RawFd> {
	fn try_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> {
		flock::<FlockLock<Self>>(self, libc::LOCK_EX | libc::LOCK_NB)
	}
	fn wait_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> {
		flock::<FlockLock<Self>>(self, libc::LOCK_EX)
	}
	
	fn try_lock_fn<Fn: FnOnce(SafeUnlockFlock<T>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> {
		flock::<FlockFnBuilder<Self, Fn, Fr>>(FlockFnBuilder::new(self, f), libc::LOCK_EX | libc::LOCK_NB)
	}
	fn wait_lock_fn<Fn: FnOnce(SafeUnlockFlock<T>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> {
		flock::<FlockFnBuilder<Self, Fn, Fr>>(FlockFnBuilder::new(self, f), libc::LOCK_EX)
	}
}



#[inline]
fn flock<I: BehOsRelease>(arg: I::Data, flag: libc::c_int) -> Result<I::Ok, I::Err> where I::Data : FlockElement<FilePtr = RawFd> {
	match unsafe { libc::flock(arg.as_file_ptr(), flag) } {
		0 => Ok( I::ok(arg) ),
		_ => Err( I::err(arg, io::Error::last_os_error()) ),
	}
}

