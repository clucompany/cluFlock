use cluFlock::FlockLock;
use cluFlock::ToFlock;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::ErrorKind::AlreadyExists;
use std::path::Path;
use std::time::Duration;

// Example
// The temporary file for interprogram synchronization.

#[derive(Debug)]
#[allow(dead_code)]
pub struct RuntimeLockFile<'a> {
	exclusive_flock: FlockLock<File>,
	maybe_autoremove_file: Option<AutoRemovePath<'a>>,
}

impl<'a> RuntimeLockFile<'a> {
	pub fn new(path: &'a Path) -> Result<Self, std::io::Error> {
		let new_lock_file = OpenOptions::new()
			.read(true)
			.write(true)
			.create_new(true)
			.open(path);
		let (lock, auto_remove_path) = match new_lock_file {
			Ok(file) => {
				let lock = file.wait_exclusive_lock()?;
				let auto_remove_file = AutoRemovePath::new(path);

				(lock, Some(auto_remove_file))
			}

			Err(ref e) if e.kind() == AlreadyExists => {
				let open_file = OpenOptions::new().read(true).write(false).open(path)?;

				let lock = open_file.try_exclusive_lock()?;
				(lock, None)
			}
			Err(e) => return Err(e),
		};

		let sself = RuntimeLockFile {
			exclusive_flock: lock,
			maybe_autoremove_file: auto_remove_path,
		};
		Ok(sself)
	}

	pub fn is_exists(path: &'a Path) -> Result<bool, std::io::Error> {
		let file = OpenOptions::new().read(true).write(false).open(path)?;

		FlockLock::try_exclusive_lock_fn(
			&file,
			|_| Ok(false),
			|e| match e.kind() {
				ErrorKind::WouldBlock => Ok(true), // ignore err
				_ => Err(e.into_err()),            // into_err: FlockErr -> std::io::Error
			},
		)
	}
}

#[derive(Debug)]
#[repr(transparent)]
pub struct AutoRemovePath<'a> {
	path: &'a Path,
}

impl<'a> AutoRemovePath<'a> {
	#[inline(always)]
	pub fn new(path: &'a Path) -> Self {
		Self { path }
	}
}

impl<'a> Drop for AutoRemovePath<'a> {
	#[inline(always)]
	fn drop(&mut self) {
		std::fs::remove_file(self.path).unwrap();
	}
}

pub fn main() -> Result<(), std::io::Error> {
	let path = Path::new("./test_file");

	println!("#RuntimeLockFile, path: {:?}", path);
	println!(
		"#RuntimeLockFile, is_exists_lock: {:?}",
		RuntimeLockFile::is_exists(path)
	);

	let lock_file = RuntimeLockFile::new(path)?;
	println!("#RuntimeLockFile, lock: {:?}", lock_file);

	for a in 0..12 {
		println!("Sleep {}", a);
		std::thread::sleep(Duration::from_secs(1));
	}

	Ok(())
}

// How to run correctly?
// cargo run --example runtime_lock_file --release
// ./target/release/examples/runtime_lock_file & ./target/release/examples/runtime_lock_file
