use crate::element::FlockElement;
use crate::err::FlockError;
use crate::err::IoError;
use crate::unlock::WaitFlockUnlock;
use crate::ExclusiveFlock;
use crate::SharedFlock;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::Deref;
use core::ops::DerefMut;
use SafeManuallyDrop::ManuallyDrop;

/// Type for securely creating and securely managing 'flock' locks.
#[derive(/*Copy, */ Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FlockLock<T>
where
	T: FlockElement + WaitFlockUnlock,
{
	data: ManuallyDrop<T>,
}

impl<T> Debug for FlockLock<T>
where
	T: Debug + FlockElement + WaitFlockUnlock,
{
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		f.debug_struct("FlockLock")
			.field("data", &self.data as &T)
			.finish()
	}
}

impl<T> FlockLock<T>
where
	T: FlockElement + WaitFlockUnlock,
{
	/// Create lock surveillance structure, unsafe because it
	/// is not known if a lock has been created before.
	#[inline]
	pub const unsafe fn force_new(data: T) -> Self {
		Self {
			data: ManuallyDrop::new(data),
		}
	}

	/// Expect to get an exclusive lock or get an error right away.
	#[inline(always)]
	pub fn wait_exclusive_lock(data: T) -> Result<FlockLock<T>, FlockError<T>>
	where
		T: ExclusiveFlock,
	{
		ExclusiveFlock::wait_lock(data)
	}

	/// Get an exclusive lock without waiting (if there was no lock before)
	/// or get an error right away.
	#[inline(always)]
	pub fn try_exclusive_lock(data: T) -> Result<FlockLock<T>, FlockError<T>>
	where
		T: ExclusiveFlock,
	{
		ExclusiveFlock::try_lock(data)
	}

	/// Expect to get an exclusive lock or get an error right away.
	#[inline(always)]
	pub fn wait_exclusive_lock_fn<R>(
		data: T,
		next: impl FnOnce(FlockLock<T>) -> R,
		errf: impl FnOnce(FlockError<T>) -> R,
	) -> R
	where
		T: ExclusiveFlock,
	{
		ExclusiveFlock::wait_lock_fn(data, next, errf)
	}

	/// Get an exclusive lock without waiting (if there was no lock before)
	/// or get an error right away.
	#[inline(always)]
	pub fn try_exclusive_lock_fn<R>(
		data: T,
		next: impl FnOnce(FlockLock<T>) -> R,
		errf: impl FnOnce(FlockError<T>) -> R,
	) -> R
	where
		T: ExclusiveFlock,
	{
		ExclusiveFlock::try_lock_fn(data, next, errf)
	}

	/// Expect to get an shared lock or get an error right away.
	#[inline(always)]
	pub fn wait_shared_lock(f: T) -> Result<FlockLock<T>, FlockError<T>>
	where
		T: SharedFlock,
	{
		SharedFlock::wait_lock(f)
	}

	/// Get an shared lock without waiting (if there was no lock before)
	/// or get an error right away.
	#[inline(always)]
	pub fn try_shared_lock(f: T) -> Result<FlockLock<T>, FlockError<T>>
	where
		T: SharedFlock,
	{
		SharedFlock::try_lock(f)
	}

	/// Expect to get an shared lock or get an error right away.
	#[inline(always)]
	pub fn wait_shared_lock_fn<R>(
		data: T,
		next: impl FnOnce(FlockLock<T>) -> R,
		errf: impl FnOnce(FlockError<T>) -> R,
	) -> R
	where
		T: SharedFlock,
	{
		SharedFlock::wait_lock_fn(data, next, errf)
	}

	/// Get an shared lock without waiting (if there was no lock before)
	/// or get an error right away.
	#[inline(always)]
	pub fn try_shared_lock_fn<R>(
		data: T,
		next: impl FnOnce(FlockLock<T>) -> R,
		errf: impl FnOnce(FlockError<T>) -> R,
	) -> R
	where
		T: SharedFlock,
	{
		SharedFlock::try_lock_fn(data, next, errf)
	}
	//

	/// Get pointer to data
	#[inline(always)]
	pub fn as_data(&self) -> &T {
		&self.data
	}

	/// Get mut pointer to data
	#[inline(always)]
	pub fn as_mut_data(&mut self) -> &mut T {
		&mut self.data
	}

	/// Get raw pointer to data
	#[inline(always)]
	pub fn as_ptr(&self) -> *const T {
		ManuallyDrop::as_ptr(&self.data)
	}

	/// Get raw mut pointer to data
	#[inline(always)]
	pub fn as_mut_ptr(&mut self) -> *mut T {
		ManuallyDrop::as_mut_ptr(&mut self.data)
	}

	/// Destroy the 'flock' lock, return a good result or error.
	#[inline]
	pub fn unlock_fn<R>(mut self, next: impl FnOnce() -> R, errf: impl FnOnce(IoError) -> R) -> R {
		let result = unsafe { WaitFlockUnlock::unlock_fn(self.as_mut_data(), next, errf) };

		// always drop
		ManuallyDrop::drop(&mut self.data);
		ManuallyDrop::forget(self);

		result
	}

	/// Destroy the 'flock' lock, return a good result or error.
	#[inline]
	pub fn unlock(self) -> Result<(), IoError> {
		self.unlock_fn(|| Ok(()), Err)
	}

	/// Ignore flock unlock.
	#[inline]
	pub unsafe fn ignore_unlock_no_result(mut self) {
		// always drop
		self.nomove_ignore_unlock_no_result();
		ManuallyDrop::forget(self);
	}

	/// Ignore flock unlock.
	/// ->>> (!!! Be sure to call "forget(self)" on yourself. The FlockRange destructor should no longer be executed!)
	/// ->>> (!!! If you suddenly forget to call "forget(self)", you will still be caught by SafeManuallyDrop)
	#[inline]
	pub unsafe fn nomove_ignore_unlock_no_result(&mut self) {
		// always drop
		ManuallyDrop::drop(&mut self.data);
	}

	/// Is FlockLock a wrapper with values, or is it actually a transparent value with no false data.
	#[inline(always)]
	pub const fn is_repr_transparent(&self) -> bool {
		self.data.is_repr_transparent()
	}

	/// Ignore flock unlock. Return data only.
	#[inline]
	pub unsafe fn ignore_unlock(mut self) -> T {
		let data = ManuallyDrop::take(&mut self.data);
		ManuallyDrop::forget(self);

		data
	}

	/// Destroy the 'flock' lock, return a good result or error.
	#[inline]
	pub fn unlock_no_err_result(mut self) {
		unsafe {
			WaitFlockUnlock::unlock_no_result(self.as_mut_data());

			self.ignore_unlock_no_result()
		}
	}

	/// Destroy the "flock" lock, return data and error data.
	#[inline]
	pub fn unlock_data(self) -> (T, Result<(), IoError>) {
		self.unlock_data_fn(|| Ok(()), Err)
	}

	/// Destroy the "flock" lock, return data and error data.
	#[inline]
	pub fn unlock_data_fn<R>(
		mut self,
		next: impl FnOnce() -> R,
		errf: impl FnOnce(IoError) -> R,
	) -> (T, R) {
		unsafe {
			let result = WaitFlockUnlock::unlock_fn(self.as_mut_data(), next, errf);
			let data = self.ignore_unlock();

			(data, result)
		}
	}

	/// Remove lock "flock", return data.
	#[inline]
	pub fn unlock_data_no_err_result(mut self) -> T {
		unsafe {
			WaitFlockUnlock::unlock_no_result(self.as_mut_data());

			self.ignore_unlock()
		}
	}
}

impl<T> AsRef<T> for FlockLock<T>
where
	T: FlockElement + WaitFlockUnlock,
{
	#[inline(always)]
	fn as_ref(&self) -> &T {
		&self.data
	}
}

impl<T> AsMut<T> for FlockLock<T>
where
	T: FlockElement + WaitFlockUnlock,
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut T {
		&mut self.data
	}
}

impl<T> Deref for FlockLock<T>
where
	T: FlockElement + WaitFlockUnlock,
{
	type Target = T;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.as_data()
	}
}

impl<T> DerefMut for FlockLock<T>
where
	T: FlockElement + WaitFlockUnlock,
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_data()
	}
}

impl<T> Drop for FlockLock<T>
where
	T: FlockElement + WaitFlockUnlock,
{
	#[inline(always)]
	fn drop(&mut self) {
		unsafe {
			WaitFlockUnlock::unlock_no_result(self.as_mut_data());
		}

		// alternative self.ignore_unlock_no_result()
		// always drop
		unsafe { self.nomove_ignore_unlock_no_result() }
	}
}
