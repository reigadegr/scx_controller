use core::ptr::NonNull;
use libc::{DIR, c_int, close, closedir};

pub struct DirGuard(Option<NonNull<DIR>>);

impl DirGuard {
    /// 创建一个新的 `DirGuard`，包装一个 `DIR*` 指针。
    pub const fn new(dir: *mut DIR) -> Self {
        Self(NonNull::new(dir))
    }
}

impl Drop for DirGuard {
    /// 关闭目录并释放资源。
    fn drop(&mut self) {
        if let Some(dir) = self.0.take() {
            let _ = unsafe { closedir(dir.as_ptr()) };
        }
    }
}

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
