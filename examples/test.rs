

extern crate cluFlock;
extern crate libc;

use cluFlock::Flock;
use std::fs::File;

fn main() {

     let file = File::create("/tmp/1").unwrap();

     //let e = flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB);

     let lock = file.unique_lock();


     println!("{:?}", lock);

     ::std::thread::sleep_ms(3000);

}
