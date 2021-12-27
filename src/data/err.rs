
//! Error structures used in cluFlock methods.
//!

use core::fmt::Debug;
use core::fmt::Display;
use std::error::Error;
use crate::element::FlockElement;
use core::ops::Deref;
use std::io::ErrorKind;

/// The standard error for Flock methods, from the error you can get a borrowed value.
//#[derive(Debug)]
pub struct FlockError<T> where T: FlockElement {
	data: T,
	err: std::io::Error,
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

impl<T> From<(T, std::io::Error)> for FlockError<T> where T: FlockElement {
	#[inline(always)]
	fn from((data, err): (T, std::io::Error)) -> Self {
		Self::new(data, err)
	}
}

impl<T> FlockError<T> where T: FlockElement {
	#[inline]
	pub fn new(a: T, err: std::io::Error) -> Self {
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
	pub fn as_err(&self) -> &std::io::Error {
		&self.err
	}
	
	/// Retrieve only the data from the error structure.
	#[inline(always)]
	pub fn into_data(self) -> T {
		self.data
	}
	
	/// Get all data from the error structure.
	#[inline(always)]
	pub fn into_all(self) -> (T, std::io::Error) {
		(self.data, self.err)
	}
	
	/// Get only the error from the error structure.
	#[inline(always)]
	pub fn into_err(self) -> std::io::Error {
		self.err
	}
	
	/// Get only the error from the error structure.
	#[inline(always)]
	#[deprecated(since="1.2.6", note="please use `into_err` instead")]
	pub fn err(self) -> std::io::Error {
		self.into_err()
	}
	
	/// Get all data from the error structure.
	#[inline(always)]
	#[deprecated(since="1.2.6", note="please use `into_all` instead")]
	pub fn all(self) -> (T, std::io::Error) {
		self.into_all()
	}
}

impl<T> From<FlockError<T>> for std::io::Error where T: FlockElement {
	#[inline(always)]
	fn from(a: FlockError<T>) -> std::io::Error {
		a.err
	}
}

impl<T> Deref for FlockError<T> where T: FlockElement {
	type Target = std::io::Error;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.as_err()
	}
}
