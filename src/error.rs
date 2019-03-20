
use std::ops::Deref;
use std::io;

#[derive(Debug)]
pub struct FlockError<T> {
	value: T,
	error: io::Error,
}

impl<T> FlockError<T> {
	pub fn new(a: T, err: io::Error) -> Self {
		Self {
			value: a,
			error: err,
		}	
	}
	
	#[inline(always)]
	pub fn value(self) -> T {
		self.value
	}
	
	#[inline(always)]
	pub fn error(self) -> io::Error {
		self.error	
	}
	
	#[inline(always)]
	pub fn all(self) -> (T, io::Error) {
		(self.value, self.error)
	}
}

impl<T> From<FlockError<T>> for io::Error {
	#[inline(always)]
	fn from(a: FlockError<T>) -> io::Error {
		a.error
	}
}


impl<T> Deref for FlockError<T> {
	type Target = io::Error;	
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.error	
	}
}

impl<T> From<(T, io::Error)> for FlockError<T> {
	#[inline(always)]
	fn from((t, err): (T, io::Error)) -> Self {
		FlockError::new(t, err)	
	}
}