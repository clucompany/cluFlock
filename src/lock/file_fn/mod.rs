
use crate::FlockUnlock;
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
pub struct FlockFnUnlock(File);
impl FlockLock for FlockFnUnlock {}


impl<F: FnMut(FlockFnUnlock) -> R, R> RawConstFlock for (FlockFnUnlock, PhantomData<F>, PhantomData<R>) {
     type Lock = R;
     type Arg = (File, F);

     #[inline(always)]
     fn next((file, mut f): Self::Arg) -> Self::Lock {
          f(FlockFnUnlock(file))
     }
}

impl FlockUnlock for FlockFnUnlock {
     type ResultUnlock = ();


     fn unlock(self) -> Self::ResultUnlock {
          
     }
}


impl Deref for FlockFnUnlock {
     type Target = File;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          &self.0
     }
}
impl DerefMut for FlockFnUnlock {
     #[inline(always)]
     fn deref_mut(&mut self) -> &mut Self::Target {
          &mut self.0
     }
}

impl AsRef<File> for FlockFnUnlock {
     #[inline(always)]
     fn as_ref(&self) -> &File {
          &self.0
     }
}
impl AsMut<File> for FlockFnUnlock {
     #[inline(always)]
     fn as_mut(&mut self) -> &mut File {
          &mut self.0
     }
}

impl Drop for FlockFnUnlock {
     fn drop(&mut self) {
          let _e = crate::sys::unlock(&self.0);
     }
}

impl ExclusiveFlockFn for File {
     type ExclusiveLockFn = FlockFnUnlock;

     fn wait_lock_fn<A: FnMut(Self::ExclusiveLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::wait_lock_exclusive::<(Self::ExclusiveLockFn, _, _)>((self, f))
     }
     
     fn try_lock_fn<A: FnMut(Self::ExclusiveLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::try_lock_exclusive::<(Self::ExclusiveLockFn, _, _)>((self, f))
     }
}

impl SharedFlockFn for File {
     type SharedLockFn = FlockFnUnlock;

     fn wait_lock_fn<A: FnMut(Self::SharedLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::wait_lock_shared::<(Self::SharedLockFn, _, _)>((self, f))
     }
     
     fn try_lock_fn<A: FnMut(Self::SharedLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::try_lock_shared::<(Self::SharedLockFn, _, _)>((self, f))
     }
}




///The type used in closure automatically will unblock 'flock' at destruction of type.
#[derive(Debug)]
pub struct SliceFlockFnUnlock<'a>(&'a File);
impl<'a> FlockLock for SliceFlockFnUnlock<'a> {}


impl<'a, F: FnMut(SliceFlockFnUnlock<'a>) -> R, R> RawConstFlock for (SliceFlockFnUnlock<'a>, PhantomData<F>, PhantomData<R>) {
     type Lock = R;
     type Arg = (&'a File, F);

     #[inline(always)]
     fn next((file, mut f): Self::Arg) -> Self::Lock {
          f(SliceFlockFnUnlock(file))
     }
}

impl<'a> FlockUnlock for SliceFlockFnUnlock<'a> {
     type ResultUnlock = ();


     fn unlock(self) -> Self::ResultUnlock {
          
     }
}



impl<'a> Deref for SliceFlockFnUnlock<'a> {
     type Target = File;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          self.0
     }
}

impl<'a> AsRef<File> for SliceFlockFnUnlock<'a> {
     #[inline(always)]
     fn as_ref(&self) -> &File {
          self.0
     }
}

impl<'a> Drop for SliceFlockFnUnlock<'a> {
     fn drop(&mut self) {
          let _e = crate::sys::unlock(self.0);
     }
}

impl<'a> ExclusiveFlockFn for &'a File {
     type ExclusiveLockFn = SliceFlockFnUnlock<'a>;

     fn wait_lock_fn<A: FnMut(Self::ExclusiveLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::wait_lock_exclusive::<(Self::ExclusiveLockFn, _, _)>((self, f))
     }
     
     fn try_lock_fn<A: FnMut(Self::ExclusiveLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::try_lock_exclusive::<(Self::ExclusiveLockFn, _, _)>((self, f))
     }
}

impl<'a> SharedFlockFn for &'a File {
     type SharedLockFn = SliceFlockFnUnlock<'a>;

     fn wait_lock_fn<A: FnMut(Self::SharedLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::wait_lock_shared::<(Self::SharedLockFn, _, _)>((self, f))
     }
     
     fn try_lock_fn<A: FnMut(Self::SharedLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::try_lock_shared::<(Self::SharedLockFn, _, _)>((self, f))
     }
}




///The type used in closure automatically will unblock 'flock' at destruction of type.
#[derive(Debug)]
pub struct MutSliceFlockFnUnlock<'a>(&'a mut File);
impl<'a> FlockLock for MutSliceFlockFnUnlock<'a> {}


impl<'a, F: FnMut(MutSliceFlockFnUnlock<'a>) -> R, R> RawConstFlock for (MutSliceFlockFnUnlock<'a>, PhantomData<F>, PhantomData<R>) {
     type Lock = R;
     type Arg = (&'a mut File, F);

     #[inline(always)]
     fn next((file, mut f): Self::Arg) -> Self::Lock {
          f(MutSliceFlockFnUnlock(file))
     }
}


impl<'a> FlockUnlock for MutSliceFlockFnUnlock<'a> {
     type ResultUnlock = ();


     fn unlock(self) -> Self::ResultUnlock {
          
     }
}

impl<'a> Deref for MutSliceFlockFnUnlock<'a> {
     type Target = File;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          self.0
     }
}
impl<'a> DerefMut for MutSliceFlockFnUnlock<'a> {
     #[inline(always)]
     fn deref_mut(&mut self) -> &mut Self::Target {
          self.0
     }
}

impl<'a> AsRef<File> for MutSliceFlockFnUnlock<'a> {
     #[inline(always)]
     fn as_ref(&self) -> &File {
          self.0
     }
}
impl<'a> AsMut<File> for MutSliceFlockFnUnlock<'a> {
     #[inline(always)]
     fn as_mut(&mut self) -> &mut File {
          self.0
     }
}

impl<'a> Drop for MutSliceFlockFnUnlock<'a> {
     fn drop(&mut self) {
          let _e = crate::sys::unlock(&self.0);
     }
}


impl<'a> ExclusiveFlockFn for &'a mut File {
     type ExclusiveLockFn = MutSliceFlockFnUnlock<'a>;

     fn wait_lock_fn<A: FnMut(Self::ExclusiveLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::wait_lock_exclusive::<(Self::ExclusiveLockFn, _, _)>((self, f))
     }
     
     fn try_lock_fn<A: FnMut(Self::ExclusiveLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::try_lock_exclusive::<(Self::ExclusiveLockFn, _, _)>((self, f))
     }
}

impl<'a> SharedFlockFn for &'a mut File {
     type SharedLockFn = MutSliceFlockFnUnlock<'a>;

     fn wait_lock_fn<A: FnMut(Self::SharedLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::wait_lock_shared::<(Self::SharedLockFn, _, _)>((self, f))
     }
     
     fn try_lock_fn<A: FnMut(Self::SharedLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> {
          crate::sys::try_lock_shared::<(Self::SharedLockFn, _, _)>((self, f))
     }
}
