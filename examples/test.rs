

extern crate cluFlock;

use cluFlock::Flock;
use std::fs::File;

fn main() {
     let file = File::create("/tmp/1").unwrap();

     let lock = file.try_exclusive_lock();


     println!("{:?}", lock);

     #[allow(deprecated)]
     ::std::thread::sleep_ms(3000);

     drop(lock);
}
