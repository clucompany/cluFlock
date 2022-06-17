
//! Error structures used in cluFlock methods.
//!

use core::fmt::Debug;
use core::fmt::Display;
use std::error::Error;
use crate::element::FlockElement;
use core::ops::Deref;
use std::io::ErrorKind;
use std::io::Error as IoError;

/// The standard error for Flock methods, from the error you can get a borrowed value.
//#[derive(Debug)]
pub struct FlockError<T> where T: FlockElement {
	data: T,
	err: IoError,
}

impl<T> Error for FlockError<T> where T: FlockElement {
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		Error::source(&self.err)
	}
	
	/*#[inline(always)] <<-- TODO, UNSTABLE unstable(feature = "backtrace", issue = "53487")
	fn backtrace(&self) -> Option<&Backtrace> {
		Error::backtrace(&self.err)
	}*/
	
	/*#[inline(always)] <<-- TODO, UNSTABLE issue = "60784"
	fn type_id(&self, _: private::Internal) -> TypeId {
		
	}
	*/
	
	#[allow(deprecated)]
	#[inline(always)]
	fn description(&self) -> &str {
		Error::description(&self.err)
	}
	
	#[allow(deprecated)]
	#[inline(always)]
	fn cause(&self) -> Option<&dyn Error> {
		Error::cause(&self.err)
	}
}

// Required only for compatibility with StdErr(Error).
impl<T> Display for FlockError<T> where T: FlockElement {
	#[inline(always)]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		Display::fmt(&self.err, f)
	}
}

// Required only for compatibility with StdErr(Error).
impl<T> Debug for FlockError<T> where T: FlockElement {
	#[inline(always)]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		Debug::fmt(&self.err, f)
	}
}

impl<T> From<(T, IoError)> for FlockError<T> where T: FlockElement {
	#[inline(always)]
	fn from((data, err): (T, IoError)) -> Self {
		Self::new(data, err)
	}
}

impl<T> FlockError<T> where T: FlockElement {
	#[inline]
	pub fn new(a: T, err: IoError) -> Self {
		Self {
			data: a,
			err: err,
		}
	}
	
	#[inline(always)]
	pub fn get_debug_data(&self) -> &impl Debug where T: Debug {
		&self.data
	}
	
	#[inline(always)]
	pub fn get_debug_err(&self) -> &impl Debug {
		&self.err
	}
	
	/// Retrieve only the data from the error structure.
	#[inline(always)]
	pub fn into(self) -> T {
		self.into_data()
	}
	
	#[inline(always)]
	pub fn is_would_block(&self) -> bool {
		self.err.kind() == ErrorKind::WouldBlock
	}
	
	/// The error occurred due to the presence of a lock.
	#[inline(always)]
	pub fn is_already_lock(&self) -> bool {
		self.is_would_block()
	}
	
	/// Get a link to data.
	#[inline(always)]
	pub fn as_data(&self) -> &T {
		&self.data
	}
	
	/// Get a link to err.
	#[inline(always)]
	pub fn as_err(&self) -> &IoError {
		&self.err
	}
	
	/// Retrieve only the data from the error structure.
	#[inline(always)]
	pub fn into_data(self) -> T {
		self.data
	}
	
	/// Get all data from the error structure.
	#[inline(always)]
	pub fn into_all(self) -> (T, IoError) {
		(self.data, self.err)
	}
	
	/// Get only the error from the error structure.
	#[inline(always)]
	pub fn into_err(self) -> IoError {
		self.err
	}
}

impl<T> From<FlockError<T>> for IoError where T: FlockElement {
	#[inline(always)]
	fn from(a: FlockError<T>) -> IoError {
		a.err
	}
}

impl<T> Deref for FlockError<T> where T: FlockElement {
	type Target = IoError;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.as_err()
	}
}
