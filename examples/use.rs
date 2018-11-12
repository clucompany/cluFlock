

extern crate cluFlock;

use cluFlock::Flock;
use std::fs::File;

fn main() {
     let file = File::create("/tmp/1").unwrap();

     let lock = file.exclusive_lock();
          //lock...
     println!("{:?}", lock);

     drop(lock);
}