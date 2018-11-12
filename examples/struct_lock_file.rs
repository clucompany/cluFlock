
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

//How to run correctly?
//cargo run --example struct_lock_file --release
//./target/release/examples/struct_lock_file & ./target/release/examples/struct_lock_file

