

/*!


Устанавливает или снимает консультативную блокировку на открытом файле.



*/



#[cfg(unix)]
mod unix;
#[cfg(unix)]
use unix as sys;

use std::fmt::Debug;
use std::fs::File;
use std::io;

mod lock;
pub use self::lock::*;


pub trait Flock<'a> {
     type ExclusiveLock: FlockLock + 'a;
     type SharedLock: FlockLock + 'a;

     fn try_exclusive_lock(&'a self) -> Result<Self::ExclusiveLock, io::Error>;
     
     //Установить эксклюзивную блокировку. Только один процесс может держать эксклюзивную блокировку файла. 
     fn exclusive_lock(&'a self) -> Result<Self::ExclusiveLock, io::Error>;

     
     fn try_shared_lock(&'a self) -> Result<Self::SharedLock, io::Error>;

     //Установить разделяемую блокировку. Разделяемую блокировку на заданный файл может держать более чем один процесс. 
     fn shared_lock(&'a self) -> Result<Self::SharedLock, io::Error>;
}

impl<'l, 'a, F: Flock<'a>> Flock<'a> for &'l F {
     type ExclusiveLock = F::ExclusiveLock;
     type SharedLock = F::SharedLock;

     #[inline(always)]
     fn try_exclusive_lock(&'a self) -> Result<Self::ExclusiveLock, io::Error> {(**self).try_exclusive_lock()}

     #[inline(always)]
     fn exclusive_lock(&'a self) -> Result<Self::ExclusiveLock, io::Error> {(**self).exclusive_lock()}

     #[inline(always)]
     fn try_shared_lock(&'a self) -> Result<Self::SharedLock, io::Error> {(**self).try_shared_lock()}

     #[inline(always)]
     fn shared_lock(&'a self) -> Result<Self::SharedLock, io::Error> {(**self).shared_lock()}
}

impl<'l, 'a, F: Flock<'a>> Flock<'a> for &'l mut F {
     type ExclusiveLock = F::ExclusiveLock;
     type SharedLock = F::SharedLock;

     #[inline(always)]
     fn try_exclusive_lock(&'a self) -> Result<Self::ExclusiveLock, io::Error> {(**self).try_exclusive_lock()}

     #[inline(always)]
     fn exclusive_lock(&'a self) -> Result<Self::ExclusiveLock, io::Error> {(**self).exclusive_lock()}

     #[inline(always)]
     fn try_shared_lock(&'a self) -> Result<Self::SharedLock, io::Error> {(**self).try_shared_lock()}

     #[inline(always)]
     fn shared_lock(&'a self) -> Result<Self::SharedLock, io::Error> {(**self).shared_lock()}
}


/*
impl<'l, A: AsRef<File>> Flock for &'l A {
     #[inline]
     fn try_exclusive_lock<'a>(&'a self) -> Result<UniqueFlockLock<'a>, io::Error> {
          UniqueFlockLock::try_lock(self.as_ref())
     }

     #[inline]
     fn exclusive_lock<'a>(&'a self) -> Result<UniqueFlockLock<'a>, io::Error> {
          UniqueFlockLock::lock(self.as_ref())
     }

     #[inline]
     fn try_shared_lock<'a>(&'a self) -> Result<SharedFlockLock<'a>, io::Error> {
          SharedFlockLock::try_lock(self.as_ref())
     }

     #[inline]
     fn shared_lock<'a>(&'a self) -> Result<SharedFlockLock<'a>, io::Error> {
          SharedFlockLock::lock(self.as_ref())
     }
}*/

impl<'a> Flock<'a> for File {
     type ExclusiveLock = UniqueFlockLock<'a>;
     type SharedLock = SharedFlockLock<'a>;

     #[inline]
     fn try_exclusive_lock(&'a self) -> Result<Self::ExclusiveLock, io::Error> {
          UniqueFlockLock::try_lock(self)
     }

     #[inline]
     fn exclusive_lock(&'a self) -> Result<Self::ExclusiveLock, io::Error> {
          UniqueFlockLock::lock(self)
     }

     #[inline]
     fn try_shared_lock(&'a self) -> Result<Self::SharedLock, io::Error> {
          SharedFlockLock::try_lock(self)
     }

     #[inline]
     fn shared_lock(&'a self) -> Result<Self::SharedLock, io::Error> {
          SharedFlockLock::lock(self)
     }
}


pub trait FlockLock: Drop + Debug {
     fn unlock(self) where Self: Sized {}
     fn box_unlock(self: Box<Self>) where Self: Sized {}
}
