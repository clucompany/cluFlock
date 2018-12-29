
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::fs::File;

pub trait UnixRawFd {
     fn as_raw_fd(&self) -> RawFd;
}

impl<'a, A: UnixRawFd> UnixRawFd for &'a A {
     #[inline(always)]
     fn as_raw_fd(&self) -> RawFd {
          A::as_raw_fd(self)
     }
}

impl<'a, A: UnixRawFd> UnixRawFd for &'a mut A {
     #[inline(always)]
     fn as_raw_fd(&self) -> RawFd {
          A::as_raw_fd(self)
     }
}

impl UnixRawFd for File {
     #[inline(always)]
     fn as_raw_fd(&self) -> RawFd {
          AsRawFd::as_raw_fd(self)
     }
}