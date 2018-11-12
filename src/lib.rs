//Copyright 2018 #UlinProject Денис Котляров

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//       http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.


//#Ulin Project 1718
//

/*!
Control of lock of the file using the 'flock' functions.


# Capabilities

1. Convenient and transparent trait of a call of locks.
2. Automatic unlocking of lock.

# Locks

1. ExclusiveLock - Only one process can retain exclusive lock of the file.
2. SharedLock - Can retain the general lock on the given file more than one process.

# Use

1. LockSliceFile
```
extern crate cluFlock;

use cluFlock::Flock;
use std::fs::File;

fn main() {
     let file = File::create("/tmp/1").unwrap();

     let lock = file.exclusive_lock();
     //Only one process can retain exclusive lock of the file.

     println!("{:?}", lock);

     drop(lock);
}
```

2. TrySliceLockFile
```
extern crate cluFlock;

use cluFlock::Flock;
use std::fs::File;
use std::time::Duration;

fn main() {
     let file = match File::create("/tmp/ulin.lock") {
          Ok(a) => a,
          Err(e) => panic!("Panic, err create file {:?}", e),
     };

     println!("Try_Exclusive_Lock, {:?}", file);
     let lock = match file.try_exclusive_lock() {
          //Success, we blocked the file.
          Ok(Some(lock)) => {
               println!("File {:?} successfully locked.", file);
               lock
          },
          
          //File already locked.
          Ok(None) => {
               println!("File {:?} already locked.", file);

               println!("!Exclusive_Lock, {:?}", file);
               //Lock the current thread to such an extent until your file is unlocked.
               file.exclusive_lock().unwrap()
          },
          
          Err(e) => panic!("Panic, err lock file {:?}", e)

     };

     println!("Sleep, 5s");
     ::std::thread::sleep(Duration::from_secs(5));

     println!("Unlock, {:?}", file);
     drop(lock);
}
```

3. FileLock (BufLockFile + try_lock)

```
extern crate cluFlock;

use std::io::ErrorKind::AlreadyExists;
use cluFlock::ExclusiveLock;
use cluFlock::Flock;
use std::path::Path;
use std::fs;
use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::fs::OpenOptions;

#[derive(Debug)]
pub struct MyLockFile<'a>(ExclusiveLock, Option<&'a Path>);

impl<'a> MyLockFile<'a> {
     pub fn new(p: &'a Path) -> Result<Self, io::Error> {
          let (lock, path) = match OpenOptions::new().write(true).create_new(true).open(p) {
               Ok(file) => (file.file_exclusive_lock()?, Some(p)),
               Err(ref e) if e.kind() == AlreadyExists => {
                    let f = OpenOptions::new().read(true).open(p)?; 

                    match f.try_file_exclusive_lock() {
                         Ok(Some(lock)) => (lock, None),
                         Ok(None) => return Err(Error::new(ErrorKind::Other, "the file is already locked")),
                         Err(e) => return Err(e),
                    }
               },
               Err(e) => return Err(e),
          };

          Ok( MyLockFile(lock, path) )
     }
}

impl<'a> Drop for MyLockFile<'a> {
     fn drop(&mut self) {
          if let Some(path) = self.1 {
               let _e = fs::remove_file(path);
          }
     }
}


pub fn main() -> Result<(), io::Error> {
     let path = Path::new("/tmp/flock.lock");
     println!("LockFile {:?}", path);
     let lock_file = MyLockFile::new(path)?;

     println!("OK! FileLock {:?}", lock_file);
     for a in 0..4 {
          println!("Sleep {}", a);
          ::std::thread::sleep(::std::time::Duration::from_secs(1));
     }

     drop(lock_file);

     Ok( () )
}
```

*/



#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub (crate) use unix as sys;


use std::fmt::Debug;
use std::fs::File;
use std::io;

mod lock;
pub use self::lock::*;


pub trait Flock<'a>: Debug {
     type ExclusiveLock: FlockLock + 'a;
     type ExclusiveSliceLock: FlockLock + 'a;

     type SharedLock: FlockLock + 'a;
     type SharedSliceLock: FlockLock + 'a;

     fn try_exclusive_lock(&'a self) -> Result<Option<Self::ExclusiveSliceLock>, io::Error>;

     ///Set exclusive lock. Lock current thread in case of file lock. Only one process can retain exclusive lock of the file.
     fn exclusive_lock(&'a self) -> Result<Self::ExclusiveSliceLock, io::Error>;

     /*#[inline]
     fn exclusive_lock_fn<N: FnMut(Self::ExclusiveSliceLock) -> A, A>(&'a self, mut f: N) -> Result<A, io::Error> {
          match self.exclusive_lock() {
               Ok(a) => Ok( f(a) ),
               Err(e) => Err(e),
          }
     }

     #[inline]
     fn try_exclusive_lock_fn<F: FnMut(Self::ExclusiveLock) -> A, A>(&'a mut self, mut f: F) -> Result<Option<A>, io::Error> {
          match self.try_file_exclusive_lock() {
               Ok(Some(lock)) => {
                    let result = f(lock);  

                    Ok( Some(result))
               },
               Ok(None) => Ok(None),

               Err(e) => Err(e),
          }
     }*/
     
     ///Set exclusive lock. Lock current thread in case of file lock. Only one process can retain exclusive lock of the file.
     fn file_exclusive_lock(self) -> Result<Self::ExclusiveLock, io::Error>;
     fn try_file_exclusive_lock(self) -> Result<Option<Self::ExclusiveLock>, io::Error>;
     
     

     fn try_shared_lock(&'a self) -> Result<Option<Self::SharedSliceLock>, io::Error>;

     ///Set shared lock. Lock current thread in case of file lock. Can retain the general lock on the given file more than one process.
     fn shared_lock(&'a self) -> Result<Self::SharedSliceLock, io::Error>;

     /*#[inline]
     fn shared_lock_fn<N: FnMut(Self::SharedSliceLock) -> A, A>(&'a self, mut f: N) -> Result<A, io::Error> {
          match self.shared_lock() {
               Ok(a) => Ok( f(a) ),
               Err(e) => Err(e),
          }
     }

     #[inline]
     fn try_shared_lock_fn<F: FnMut(Self::SharedSliceLock) -> A, A>(&'a self, mut f: F) -> Result<Option<A>, io::Error> {
          match self.try_shared_lock() {
               Ok(Some(a)) => Ok(Some( f(a) )),
               Ok(None) => Ok(None),

               Err(e) => return Err(e),
          }
     }*/
     

     ///Set shared lock. Lock current thread in case of file lock. Can retain the general lock on the given file more than one process.
     fn file_shared_lock(self) -> Result<Self::SharedLock, io::Error>;

     fn try_file_shared_lock(self) -> Result<Option<Self::SharedLock>, io::Error>;
}
/*
impl<'l, 'a, F: Flock<'a>> Flock<'a> for &'l F {
     type ExclusiveSliceLock = F::ExclusiveSliceLock;
     type SharedSliceLock = F::SharedSliceLock;

     #[inline(always)]
     fn try_exclusive_lock(&'a self) -> Result<Option<Self::ExclusiveSliceLock>, io::Error> {
          F::try_exclusive_lock(self)
     }

     #[inline(always)]
     fn exclusive_lock(&'a self) -> Result<Self::ExclusiveSliceLock, io::Error> {
          F::exclusive_lock(self)
     }

     #[inline(always)]
     fn exclusive_lock_fn<N: FnMut(Self::ExclusiveSliceLock) -> A, A>(&'a self, f: N) -> Result<A, io::Error> {
          F::exclusive_lock_fn(self, f)
     }

     #[inline(always)]
     fn try_shared_lock(&'a self) -> Result<Option<Self::SharedSliceLock>, io::Error> {
          F::try_shared_lock(self)
     }

     #[inline(always)]
     fn shared_lock(&'a self) -> Result<Self::SharedSliceLock, io::Error> {
          F::shared_lock(self)
     }    

     #[inline(always)]
     fn shared_lock_fn<N: FnMut(Self::SharedSliceLock) -> A, A>(&'a self, f: N) -> Result<A, io::Error> {
          F::shared_lock_fn(self, f)
     }
}

impl<'l, 'a, F: Flock<'a>> Flock<'a> for &'l mut F {
     type ExclusiveSliceLock = F::ExclusiveSliceLock;
     type SharedSliceLock = F::SharedSliceLock;

     #[inline(always)]
     fn try_exclusive_lock(&'a self) -> Result<Option<Self::ExclusiveSliceLock>, io::Error> {
          F::try_exclusive_lock(self)
     }

     #[inline(always)]
     fn exclusive_lock(&'a self) -> Result<Self::ExclusiveSliceLock, io::Error> {
          F::exclusive_lock(self)
     }

     #[inline(always)]
     fn exclusive_lock_fn<N: FnMut(Self::ExclusiveSliceLock) -> A, A>(&'a self, f: N) -> Result<A, io::Error> {
          F::exclusive_lock_fn(self, f)
     }

     #[inline(always)]
     fn try_shared_lock(&'a self) -> Result<Option<Self::SharedSliceLock>, io::Error> {
          F::try_shared_lock(self)
     }

     #[inline(always)]
     fn shared_lock(&'a self) -> Result<Self::SharedSliceLock, io::Error> {
          F::shared_lock(self)
     }    

     #[inline(always)]
     fn shared_lock_fn<N: FnMut(Self::SharedSliceLock) -> A, A>(&'a self, f: N) -> Result<A, io::Error> {
          F::shared_lock_fn(self, f)
     }
}*/

impl<'a> Flock<'a> for File {
     type ExclusiveLock = ExclusiveLock;
     type ExclusiveSliceLock = ExclusiveSliceLock<'a>;

     type SharedLock = SharedLock;
     type SharedSliceLock = SharedSliceLock<'a>;

     fn try_exclusive_lock(&'a self) -> Result<Option<Self::ExclusiveSliceLock>, io::Error> {
          Self::ExclusiveSliceLock::try_lock(self)
     }

     fn exclusive_lock(&'a self) -> Result<Self::ExclusiveSliceLock, io::Error> {
          Self::ExclusiveSliceLock::lock(self)
     }

     fn file_exclusive_lock(self) -> Result<Self::ExclusiveLock, io::Error> {
          Self::ExclusiveLock::lock(self)
     }
     fn try_file_exclusive_lock(self) -> Result<Option<Self::ExclusiveLock>, io::Error> {
          Self::ExclusiveLock::try_lock(self)
     }

     fn try_shared_lock(&'a self) -> Result<Option<Self::SharedSliceLock>, io::Error> {
          Self::SharedSliceLock::try_lock(self)
     }

     fn shared_lock(&'a self) -> Result<Self::SharedSliceLock, io::Error> {
          Self::SharedSliceLock::lock(self)
     }

     fn file_shared_lock(self) -> Result<Self::SharedLock, io::Error> {
          Self::SharedLock::lock(self)
     }
     fn try_file_shared_lock(self) -> Result<Option<Self::SharedLock>, io::Error> {
          Self::SharedLock::try_lock(self)
     }
}


pub trait FlockLock: Drop + Debug {
     fn unlock(self) where Self: Sized {}
     fn box_unlock(self: Box<Self>) where Self: Sized {}
}


pub (crate) trait InitFlockLock {
     type Lock: FlockLock;
     type Arg;
     
     fn new(f: Self::Arg) -> Self::Lock;
}