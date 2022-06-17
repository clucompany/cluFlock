


#[cfg(unix)]
pub mod unix {
	use std::ffi::OsString;
	use std::borrow::Cow;
	use core::ops::Deref;
	use std::process::ExitStatus;
	use std::process::Child;
	use std::ffi::OsStr;
	use std::path::Path;
	use std::fs::File;
	use cluFlock::ExclusiveFlock;
	use cluFlock::SharedFlock;
	use std::process::Command;
	use std::time::Duration;
	
	struct AutoRemoveFile<'a>(&'a Path, File);

	impl<'a> AutoRemoveFile<'a> {
		pub fn file_create(path: &'a Path) -> Self {
			AutoRemoveFile(path, File::create(path).unwrap())
		}
		
		#[inline(always)]
		pub fn as_path(&self) -> &Path {
			self.0
		}
	}

	impl<'a> Deref for AutoRemoveFile<'a> {
		type Target = File;
		
		#[inline(always)]
		fn deref(&self) -> &Self::Target {
			&self.1
		}
	}

	impl<'a> Drop for AutoRemoveFile<'a> {
		#[inline]
		fn drop(&mut self) {
			std::fs::remove_file(self.0).unwrap();
		}
	}

	struct FlockProcess(Child);

	impl FlockProcess {
		pub fn new(path: &Path, ttype: &str, args: &[impl AsRef<OsStr>]) -> FlockProcess {
			let mut output = Command::new("flock");
			output.arg(ttype).arg("-w").arg("600").arg(path);
			for arg in args.into_iter() {
				output.arg(arg);
			}
				
			let child = output.spawn().expect("Failed to execute command");
			
			FlockProcess(child)
		}
		
		pub fn sleep_exclusive(path: &Path, i: usize) -> Self {
			Self::new(path, "-x", &[OsStr::new("sleep").into(), OsString::from(i.to_string()).into()] as &[Cow<'static, OsStr>])
		}
		#[allow(dead_code)]
		pub fn sleep_shared(path: &Path, i: usize) -> Self {
			Self::new(path, "-s", &[OsStr::new("sleep").into(), OsString::from(i.to_string()).into()] as &[Cow<'static, OsStr>])
		}
		
		#[inline(always)]
		pub fn wait(mut self) -> ExitStatus {
			self.0.wait().expect("command wasn't running")
		}
	}


	#[test]
	fn unix_exclusive_two_lock_behavior_onprocess() {
		let file = AutoRemoveFile::file_create(Path::new("./del_unix_two_lock_behavior"));
		
		let one_exclusive = ExclusiveFlock::try_lock(&*file);
		let one_exclusive2 = ExclusiveFlock::try_lock(&*file);
		let one_exclusive3 = ExclusiveFlock::try_lock(&*file);
		
		if let Err(e) = one_exclusive {
			panic!("Different platform behavior expected. {:?}", e);
		}
		if let Err(e) = one_exclusive2 {
			panic!("Different platform behavior expected. {:?}", e);
		}
		if let Err(e) = one_exclusive3 {
			panic!("Different platform behavior expected. {:?}", e);
		}
		
		// 1: Ok, 2: Ok, 3: Ok !
		
		// This behavior is found only in unix, 
		// be careful not to use double exclusive locks 
		// in the same process. It 'is the specifics of the platform.
		
		drop(one_exclusive);
		drop(one_exclusive2);
		drop(one_exclusive3);
	}


	#[test]
	fn unix_exclusive_check() {
		let file = AutoRemoveFile::file_create(Path::new("./del_unix_exclusive_check"));
		
		match ExclusiveFlock::wait_lock(&*file) {
			Ok(_a) => {},
			Err(e) => panic!("Strange behavior, failed to make a primary lock, {:?}", e),
		}
		if let Err(e) = ExclusiveFlock::try_lock(&*file) {
			panic!("Strange behavior, failed to make a primary lock, {:?}", e);
		}
		// This behavior because of the platform, 
		// inside the locking process does not work.
		
		
		let flock_process = FlockProcess::sleep_exclusive(file.as_path(), 4);
		std::thread::sleep(Duration::from_secs(2));
		
		
		if let Ok(a) = ExclusiveFlock::try_lock(&*file) {
			panic!("Strange behavior, we 've already made a lock in another process.., {:?}", a);
		}

		if !flock_process.wait().success() {
			panic!("Undefined behavior, the process should have ended correctly.");
		}
		//
		 
		// The process holding the lock had
		// to die, we check the work.
		
		if let Err(e) = ExclusiveFlock::try_lock(&*file) {
			panic!("Strange behavior, failed to make a primary lock, {:?}", e);
		}
		
		drop(file);
	}
	
	
	#[test]
	fn unix_shared_check() {
		let file = AutoRemoveFile::file_create(Path::new("./del_unix_shared_check"));
		
		let shared_flock = match SharedFlock::wait_lock(&*file) {
			Ok(a) => a,
			Err(e) => panic!("Strange behavior, failed to make a primary lock, {:?}", e),
		};
		
		//exclusive process
		let flock_process = FlockProcess::sleep_exclusive(file.as_path(), 4);
		// This process will wait until we close the shared lock.
		std::thread::sleep(Duration::from_secs(2));
		
		
		if let Err(a) = SharedFlock::try_lock(&*file) { //two shared, process wait shared!
			panic!("Strange behavior, we 've already made a lock in another process.., {:?}", a);
		}
		
		// The process holding the lock had
		// to die, we check the work.
		drop(shared_flock);
		
		
		if !flock_process.wait().success() {
			panic!("Undefined behavior, the process should have ended correctly.");
		}
		//
		
		if let Err(e) = ExclusiveFlock::try_lock(&*file) {
			panic!("Strange behavior, failed to make a primary lock, {:?}", e);
		}
		
		drop(file);
	}
}




