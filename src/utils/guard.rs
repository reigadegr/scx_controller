use libc::{c_int, close};

pub struct FileGuard(c_int);

impl FileGuard {
    pub const fn new(fd: c_int) -> Self {
        Self(fd)
    }
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        unsafe { close(self.0) };
    }
}
