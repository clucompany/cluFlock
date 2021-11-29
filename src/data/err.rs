
//! Error structures used in cluFlock methods.
//!

use crate::element::FlockElement;
use core::ops::Deref;

/// The standard error for Flock methods, from the error you can get a borrowed value.
#[derive(Debug)]
pub struct FlockError<T> where T: FlockElement {
	data: T,
	err: std::io::Error,
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
	
	/// Retrieve only the data from the error structure.
	#[inline(always)]
	pub fn into(self) -> T {
		self.into_data()
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
		&self.err
	}
}
