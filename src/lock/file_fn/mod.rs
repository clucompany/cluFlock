
use crate::FlockLock;
use std::marker::PhantomData;
use crate::raw::RawConstFlock;
use crate::SharedFlockFn;
use crate::ExclusiveFlockFn;
use std::ops::Deref;
use std::ops::DerefMut;
use std::fs::File;
use std::io;

///The type used in closure automatically will unblock 'flock' at destruction of type.
#[derive(Debug)]
pub struct FlockUnlock(File);
impl FlockLock for FlockUnlock {}


impl<F: FnMut(FlockUnlock) -> R, R> RawConstFlock for (FlockUnlock, PhantomData<F>, PhantomData<R>) {
     type Lock = R;
     type Arg = (File, F);

     #[inline(always)]
     fn next((file, mut f): Self::Arg) -> Self::Lock {
          f(FlockUnlock(file))
     }
}

impl crate::FlockUnlock for FlockUnlock {
     type ResultUnlock = ();


     fn unlock(self) -> Self::ResultUnlock {
          
     }
}


impl Deref for FlockUnlock {
     type Target = File;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          &self.0
     }
}
impl DerefMut for FlockUnlock {
     #[inline(always)]
     fn deref_mut(&mut self) -> &mut Self::Target {
          &mut self.0
     }
}

impl AsRef<File> for FlockUnlock {
     #[inline(always)]
     fn as_ref(&self) -> &File {
          &self.0
     }
}
impl AsMut<File> for FlockUnlock {
     #[inline(always)]
     fn as_mut(&mut self) -> &mut File {
          &mut self.0
     }
}

impl Drop for FlockUnlock {
     fn drop(&mut self) {
          let _e = crate::sys::unlock(&self.0);
     }
}

impl ExclusiveFlockFn for File {
     type ExclusiveLockFn = FlockUnlock;

     fn wait_lock_fn<A: FnMut(Self::ExclusiveLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::wait_lock_exclusive::<(Self::ExclusiveLockFn, _, _)>((self, f))
     }
     
     fn try_lock_fn<A: FnMut(Self::ExclusiveLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::try_lock_exclusive::<(Self::ExclusiveLockFn, _, _)>((self, f))
     }
}

impl SharedFlockFn for File {
     type SharedLockFn = FlockUnlock;

     fn wait_lock_fn<A: FnMut(Self::SharedLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::wait_lock_shared::<(Self::SharedLockFn, _, _)>((self, f))
     }
     
     fn try_lock_fn<A: FnMut(Self::SharedLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::try_lock_shared::<(Self::SharedLockFn, _, _)>((self, f))
     }
}




///The type used in closure automatically will unblock 'flock' at destruction of type.
#[derive(Debug)]
pub struct SliceFlockUnlock<'a>(&'a File);
impl<'a> FlockLock for SliceFlockUnlock<'a> {}


impl<'a, F: FnMut(SliceFlockUnlock<'a>) -> R, R> RawConstFlock for (SliceFlockUnlock<'a>, PhantomData<F>, PhantomData<R>) {
     type Lock = R;
     type Arg = (&'a File, F);

     #[inline(always)]
     fn next((file, mut f): Self::Arg) -> Self::Lock {
          f(SliceFlockUnlock(file))
     }
}

impl<'a> crate::FlockUnlock for SliceFlockUnlock<'a> {
     type ResultUnlock = ();


     fn unlock(self) -> Self::ResultUnlock {
          
     }
}



impl<'a> Deref for SliceFlockUnlock<'a> {
     type Target = File;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          self.0
     }
}

impl<'a> AsRef<File> for SliceFlockUnlock<'a> {
     #[inline(always)]
     fn as_ref(&self) -> &File {
          self.0
     }
}

impl<'a> Drop for SliceFlockUnlock<'a> {
     fn drop(&mut self) {
          let _e = crate::sys::unlock(self.0);
     }
}

impl<'a> ExclusiveFlockFn for &'a File {
     type ExclusiveLockFn = SliceFlockUnlock<'a>;

     fn wait_lock_fn<A: FnMut(Self::ExclusiveLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::wait_lock_exclusive::<(Self::ExclusiveLockFn, _, _)>((self, f))
     }
     
     fn try_lock_fn<A: FnMut(Self::ExclusiveLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::try_lock_exclusive::<(Self::ExclusiveLockFn, _, _)>((self, f))
     }
}

impl<'a> SharedFlockFn for &'a File {
     type SharedLockFn = SliceFlockUnlock<'a>;

     fn wait_lock_fn<A: FnMut(Self::SharedLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::wait_lock_shared::<(Self::SharedLockFn, _, _)>((self, f))
     }
     
     fn try_lock_fn<A: FnMut(Self::SharedLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::try_lock_shared::<(Self::SharedLockFn, _, _)>((self, f))
     }
}




///The type used in closure automatically will unblock 'flock' at destruction of type.
#[derive(Debug)]
pub struct MutSliceFlockUnlock<'a>(&'a mut File);
impl<'a> FlockLock for MutSliceFlockUnlock<'a> {}


impl<'a, F: FnMut(MutSliceFlockUnlock<'a>) -> R, R> RawConstFlock for (MutSliceFlockUnlock<'a>, PhantomData<F>, PhantomData<R>) {
     type Lock = R;
     type Arg = (&'a mut File, F);

     #[inline(always)]
     fn next((file, mut f): Self::Arg) -> Self::Lock {
          f(MutSliceFlockUnlock(file))
     }
}


impl<'a> crate::FlockUnlock for MutSliceFlockUnlock<'a> {
     type ResultUnlock = ();


     fn unlock(self) -> Self::ResultUnlock {
          
     }
}

impl<'a> Deref for MutSliceFlockUnlock<'a> {
     type Target = File;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          self.0
     }
}
impl<'a> DerefMut for MutSliceFlockUnlock<'a> {
     #[inline(always)]
     fn deref_mut(&mut self) -> &mut Self::Target {
          self.0
     }
}

impl<'a> AsRef<File> for MutSliceFlockUnlock<'a> {
     #[inline(always)]
     fn as_ref(&self) -> &File {
          self.0
     }
}
impl<'a> AsMut<File> for MutSliceFlockUnlock<'a> {
     #[inline(always)]
     fn as_mut(&mut self) -> &mut File {
          self.0
     }
}

impl<'a> Drop for MutSliceFlockUnlock<'a> {
     fn drop(&mut self) {
          let _e = crate::sys::unlock(&self.0);
     }
}


impl<'a> ExclusiveFlockFn for &'a mut File {
     type ExclusiveLockFn = MutSliceFlockUnlock<'a>;

     fn wait_lock_fn<A: FnMut(Self::ExclusiveLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::wait_lock_exclusive::<(Self::ExclusiveLockFn, _, _)>((self, f))
     }
     
     fn try_lock_fn<A: FnMut(Self::ExclusiveLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::try_lock_exclusive::<(Self::ExclusiveLockFn, _, _)>((self, f))
     }
}

impl<'a> SharedFlockFn for &'a mut File {
     type SharedLockFn = MutSliceFlockUnlock<'a>;

     fn wait_lock_fn<A: FnMut(Self::SharedLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::wait_lock_shared::<(Self::SharedLockFn, _, _)>((self, f))
     }
     
     fn try_lock_fn<A: FnMut(Self::SharedLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::try_lock_shared::<(Self::SharedLockFn, _, _)>((self, f))
     }
}
