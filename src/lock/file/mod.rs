
mod buf;
mod slice;

pub use self::buf::*;
pub use self::slice::*;


use Flock;
use SharedFlock;
use ExclusiveFlock;
use std::fs::File;
use std::io;

impl<'a> Flock for File {}



impl ExclusiveFlock for File {
     type ExclusiveLock = FileFlock;

     fn wait_lock(self) -> Result<Self::ExclusiveLock, io::Error> {
          Self::ExclusiveLock::wait_lock_exclusive(self)
     }

     fn try_lock(self) -> Result<Self::ExclusiveLock, io::Error> {
          Self::ExclusiveLock::try_lock_exclusive(self)
     }
}

impl<'a> ExclusiveFlock for &'a File {
     type ExclusiveLock = FileSliceFlock<'a>;

     fn wait_lock(self) -> Result<Self::ExclusiveLock, io::Error> {
          Self::ExclusiveLock::wait_lock_exclusive(self)
     }

     fn try_lock(self) -> Result<Self::ExclusiveLock, io::Error> {
          Self::ExclusiveLock::try_lock_exclusive(self)
     }
}

impl<'a> ExclusiveFlock for &'a mut File {
     type ExclusiveLock = FileSliceFlock<'a>;

     fn wait_lock(self) -> Result<Self::ExclusiveLock, io::Error> {
          Self::ExclusiveLock::wait_lock_exclusive(self)
     }

     fn try_lock(self) -> Result<Self::ExclusiveLock, io::Error> {
          Self::ExclusiveLock::try_lock_exclusive(self)
     }
}




impl SharedFlock for File {
     type SharedLock = FileFlock;

     fn wait_lock(self) -> Result<Self::SharedLock, io::Error> {
          Self::SharedLock::wait_lock_exclusive(self)
     }

     fn try_lock(self) -> Result<Self::SharedLock, io::Error> {
          Self::SharedLock::try_lock_exclusive(self)
     }
}

impl<'a> SharedFlock for &'a File {
     type SharedLock = FileSliceFlock<'a>;

     fn wait_lock(self) -> Result<Self::SharedLock, io::Error> {
          Self::SharedLock::wait_lock_exclusive(self)
     }

     fn try_lock(self) -> Result<Self::SharedLock, io::Error> {
          Self::SharedLock::try_lock_exclusive(self)
     }
}

impl<'a> SharedFlock for &'a mut File {
     type SharedLock = FileSliceFlock<'a>;

     fn wait_lock(self) -> Result<Self::SharedLock, io::Error> {
          Self::SharedLock::wait_lock_exclusive(self)
     }

     fn try_lock(self) -> Result<Self::SharedLock, io::Error> {
          Self::SharedLock::try_lock_exclusive(self)
     }
}
