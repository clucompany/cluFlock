
//! Error structures used in cluFlock methods.
//!

use crate::data::unlock::WaitFlockUnlock;
use crate::element::FlockElement;
use crate::SafeUnlockFlock;
use std::ops::Deref;
use std::io;


/// The standard error for Flock methods, from the error you can get a borrowed value.
#[derive(Debug)]
pub struct FlockError<T> where T: FlockElement {
	data: T,
	err: io::Error,
}

impl<T> FlockError<T> where T: FlockElement {
	#[inline]
	pub fn new(a: T, err: io::Error) -> Self {
		Self {
			data: a,
			err: err,
		}
	}
	
	/// Retrieve only the data from the error structure.
	#[inline(always)]
	pub fn into(self) -> T {
		self.data
	}
	
	/// Get only the error from the error structure.
	#[inline(always)]
	pub fn err(self) -> io::Error {
		self.err
	}
	
	/// Get all data from the error structure.
	#[inline(always)]
	pub fn all(self) -> (T, io::Error) {
		(self.data, self.err)
	}
}

impl<T> From<FlockError<T>> for io::Error where T: FlockElement {
	#[inline(always)]
	fn from(a: FlockError<T>) -> io::Error {
		a.err
	}
}


impl<T> Deref for FlockError<T> where T: FlockElement {
	type Target = io::Error;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.err
	}
}

///The standard error for FlockFn! methods, from the error you can get a borrowed value.
#[derive(Debug)]
pub struct FlockFnError<D, F, Fr> where D: FlockElement + WaitFlockUnlock, F: FnOnce(SafeUnlockFlock<D>) -> Fr {
	data: FlockError<D>,
	function: F,
}

impl<D, F, Fr> FlockFnError<D, F, Fr> where D: FlockElement + WaitFlockUnlock, F: FnOnce(SafeUnlockFlock<D>) -> Fr {
	#[inline(always)]
	pub fn new(data: D, function: F, err: io::Error) -> Self {
		Self::flock_error(FlockError::new(data, err), function)
	}
	
	#[inline]
	pub fn flock_error(data: FlockError<D>, function: F) -> Self {
		Self {
			data: data,
			function: function,
		}	
	}
	
	#[inline(always)]
	pub fn into_flock_error(self) -> FlockError<D> {
		self.data
	}
	
	/// Retrieve only the data from the error structure.
	#[inline(always)]
	pub fn into(self) -> D {
		self.data.into()
	}
	
	/// Get only not executed FnOnce from the error structure.
	#[inline(always)]
	pub fn function(self) -> F {
		self.function
	}
	
	/// Get only the error from the error structure.
	#[inline(always)]
	pub fn err(self) -> io::Error {
		self.data.err()
	}
	
	/// Get all the data from the error.
	#[inline(always)]
	pub fn all(self) -> (D, F, io::Error) {
		(self.data.data, self.function, self.data.err)
	}
	
	/// Get all the data from the error structure as it is.
	#[inline(always)]
	pub fn raw_all(self) -> (FlockError<D>, F) {
		(self.data, self.function)
	}
}


impl<D, F, Fr> Deref for FlockFnError<D, F, Fr> where D: FlockElement + WaitFlockUnlock, F: FnOnce(SafeUnlockFlock<D>) -> Fr {
	type Target = io::Error;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.data.err
	}
}

impl<D, F, Fr> From<FlockFnError<D, F, Fr>> for io::Error where D: FlockElement + WaitFlockUnlock, F: FnOnce(SafeUnlockFlock<D>) -> Fr {
	#[inline(always)]
	fn from(a: FlockFnError<D, F, Fr>) -> io::Error {
		a.data.err
	}
}
