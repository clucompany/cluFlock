

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
