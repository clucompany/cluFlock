

use std::fs::File;
use std::os::unix::io::RawFd;
use std::os::unix::io::AsRawFd;

pub trait UnixRawFd {
     fn as_raw_fd(&self) -> RawFd;
}


impl<'a, A: UnixRawFd> UnixRawFd for &'a A {
     #[inline(always)]
     fn as_raw_fd(&self) -> RawFd {
          (**self).as_raw_fd()
     }
}

impl<'a, A: UnixRawFd> UnixRawFd for &'a mut A {
     #[inline(always)]
     fn as_raw_fd(&self) -> RawFd {
          (**self).as_raw_fd()
     }
}


impl UnixRawFd for File {
     #[inline(always)]
     fn as_raw_fd(&self) -> RawFd {
          AsRawFd::as_raw_fd(self)
     }
}
