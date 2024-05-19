//! Error structures used in cluFlock methods.
//!

use crate::element::FlockElement;
use crate::r#macro::cfg_std;
use core::fmt::Debug;
use core::fmt::Display;
use core::ops::Deref;
use core::ops::DerefMut;

cfg_std! {
	if #std {
		/// A list specifying general categories of I/O error.
		pub type IoErrorKind = std::io::ErrorKind;
		/// The error type for I/O operations of the `Read`, `Write`, `Seek`, and
		/// associated traits.
		pub type IoError = std::io::Error;

		use std::error::Error;
	}else {
		/// A list specifying general categories of I/O error.
		pub type IoErrorKind = crate::err_nostd::ErrorKind;
		/// The error type for I/O operations of the `Read`, `Write`, `Seek`, and
		/// associated traits.
		pub type IoError = crate::err_nostd::Error;
	}
}

/// The standard error for Flock methods, from the error you can get a borrowed value.
pub struct FlockError<T>
where
	T: FlockElement,
{
	data: T,
	err: IoError,
}

cfg_std! {
	if #std {
		#[cfg_attr(docsrs, doc(
			cfg(feature = "std")
		))]
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
			fn cause(&self) -> Option<&dyn std::error::Error> {
				Error::cause(&self.err)
			}
		}
	}
}

// Required only for compatibility with StdErr(Error).
impl<T> Display for FlockError<T>
where
	T: FlockElement,
{
	#[inline(always)]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		Display::fmt(&self.err, f)
	}
}

// Required only for compatibility with StdErr(Error).
impl<T> Debug for FlockError<T>
where
	T: FlockElement,
{
	#[inline(always)]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		Debug::fmt(&self.err, f)
	}
}

impl<T> From<(T, IoError)> for FlockError<T>
where
	T: FlockElement,
{
	#[inline(always)]
	fn from((data, err): (T, IoError)) -> Self {
		Self::new(data, err)
	}
}

impl<T> FlockError<T>
where
	T: FlockElement,
{
	/// Creating an error consisting of only the
	/// data structure and the error itself.
	#[inline]
	pub const fn new(a: T, err: IoError) -> Self {
		Self { data: a, err }
	}

	/// Get the data debugging trait.
	#[inline(always)]
	pub const fn get_debug_data(&self) -> &impl Debug
	where
		T: Debug,
	{
		&self.data
	}

	/// Get the error debugging trait.
	#[inline(always)]
	pub const fn get_debug_err(&self) -> &impl Debug {
		&self.err
	}

	/// Retrieve only the data from the error structure.
	#[inline(always)]
	pub fn into(self) -> T {
		self.into_data()
	}

	/// The operation must be blocked to complete,
	/// but it was requested that the blocking operation not be performed.
	#[inline(always)]
	pub fn is_would_block(&self) -> bool {
		self.err.kind() == IoErrorKind::WouldBlock
	}

	/// The error occurred due to the presence of a lock.
	#[inline(always)]
	pub fn is_already_lock(&self) -> bool {
		self.is_would_block()
	}

	/// Get a link to data.
	#[inline(always)]
	pub const fn as_data(&self) -> &T {
		&self.data
	}

	/// Get a link to err.
	#[inline(always)]
	pub const fn as_err(&self) -> &IoError {
		&self.err
	}

	/// Get a link to err.
	#[inline(always)]
	pub fn as_mut_err(&mut self) -> &mut IoError {
		&mut self.err
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

impl<T> From<FlockError<T>> for IoError
where
	T: FlockElement,
{
	#[inline(always)]
	fn from(a: FlockError<T>) -> IoError {
		a.into_err()
	}
}

impl<T> Deref for FlockError<T>
where
	T: FlockElement,
{
	type Target = IoError;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.as_err()
	}
}

impl<T> DerefMut for FlockError<T>
where
	T: FlockElement,
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_err()
	}
}
