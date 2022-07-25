
extern crate alloc;

use core::ops::Deref;
use core::fmt::Display;
use alloc::borrow::Cow;
use core::fmt::Formatter;
use core::fmt::Error;

/// The error type for I/O operations of the `Read`, `Write`, `Seek`, and
/// associated traits. Not a full replacement for std::error::Error 
/// (only needed for no_std).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Error {
	info: Cow<'static, str>,
	
	kind: ErrorKind,
}

impl Display for Error {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		let str = self.info.deref() as &str;
		
		Display::fmt(str, f)
	}
}

impl Error {
	/// Creates a new I/O error from a known kind of error as well as an
	/// arbitrary error payload.
	#[inline]
	pub fn new<I: Into<Cow<'static, str>>>(kind: ErrorKind, info: I) -> Self {
		Self {
			kind,
			info: info.into(),
		}
	}
	
	/// Returns the corresponding ErrorKind for this error.
	#[inline(always)]
	pub const fn kind(&self) -> ErrorKind {
		self.kind
	}
	
	/// Returns an error representing the last OS error which occurred.
	/// !!! (not supported in no_std, only as a stub).
	#[inline]
	pub const fn last_os_error() -> Self {
		Self {
			info: Cow::Borrowed("Getting last_os_error is not supported. Error unknown."),
			kind: ErrorKind::UnsupportedErr,
		}
	}
}

/// A list specifying general categories of I/O error.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorKind {
	UnsupportedErr = 0,
	
	/// The operation needs to block to complete, but the blocking operation was
	/// requested to not occur.
	WouldBlock = 4,
	
	/// A parameter was incorrect.
	InvalidInput = 6,
}

