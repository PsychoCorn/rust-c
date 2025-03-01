use core::fmt::Write;

use alloc::vec::Vec;
use libc::{close, lseek64, open};

use crate::io::{CWriter, Read};
use libc::*;

#[derive(Debug, Clone, Copy)]
pub struct OpenFlags {
    flags: c_int
}

macro_rules! flags_fn {
    ($name:ident: $flag:expr) => {
        pub fn $name(self) -> Self {
            Self {
                flags: self.flags | $flag
            }
        }
    };
}

impl OpenFlags {

    pub fn with_flags(flags: c_int) -> Self {
        Self { flags }
    }

    pub fn new() -> Self {
        Self::with_flags(0)
    }

    flags_fn!(read_only: libc::O_RDONLY);
    flags_fn!(write_only: libc::O_WRONLY);
    flags_fn!(read_write: libc::O_RDWR);
    flags_fn!(create: libc::O_CREAT);
    flags_fn!(excl: libc::O_EXCL);
    flags_fn!(append: libc::O_APPEND);
    flags_fn!(trunc: libc::O_TRUNC);

    // Windows-specific flags
    #[cfg(windows)]
    flags_fn!(binary: libc::O_BINARY);
    #[cfg(windows)]
    flags_fn!(no_inherit: libc::O_NOINHERIT);
    #[cfg(windows)]
    flags_fn!(random: libc::O_RANDOM);
    #[cfg(windows)]
    flags_fn!(raw: libc::O_RAW);
    #[cfg(windows)]
    flags_fn!(sequential: libc::O_SEQUENTIAL);
    #[cfg(windows)]
    flags_fn!(temporary: libc::O_TEMPORARY);
    #[cfg(windows)]
    flags_fn!(ansi: libc::O_TEXT);
    #[cfg(windows)]
    flags_fn!(unicode: libc::_O_WTEXT);
    #[cfg(windows)]
    flags_fn!(utf8: libc::_O_U8TEXT);
    #[cfg(windows)]
    flags_fn!(utf16: libc::_O_U16TEXT);
    #[cfg(windows)]
    flags_fn!(short_lived: libc::_O_SHORT_LIVED);

    // Unix-specific flags
    #[cfg(unix)]
    flags_fn!(cloexec: libc::O_CLOEXEC);
    #[cfg(unix)]
    flags_fn!(directory: libc::O_DIRECTORY);
    #[cfg(unix)]
    flags_fn!(no_follow: libc::O_NOFOLLOW);
    #[cfg(unix)]
    flags_fn!(dsync: libc::O_DSYNC);
    #[cfg(unix)]
    flags_fn!(sync: libc::O_SYNC);
    #[cfg(unix)]
    flags_fn!(rsync: libc::O_RSYNC);
    #[cfg(unix)]
    flags_fn!(no_atime: libc::O_NOATIME);
    #[cfg(unix)]
    flags_fn!(path: libc::O_PATH);
}

#[repr(i32)]
pub enum Origin {
    Beginning = libc::SEEK_SET,
    Current = libc::SEEK_CUR,
    End = libc::SEEK_END,
}

#[derive(Clone, Debug)]
pub struct File {
    fd: i32
}

impl File {
    const MAX_BUF_STACK_SIZE: usize = 256;
    /// # Safety
    /// path must be null-terminated
    pub unsafe fn open_nt(path: &[u8], flags: OpenFlags) -> Option<Self> {
        let path = path.as_ptr() as *const c_char;
        let fd = unsafe {
            open(path, flags.flags)
        };
        if fd < 0 { 
            None 
        } else {
            Some(Self { fd })
        }
    }

    fn open_no_alloc(path: &[u8], flags: OpenFlags) -> Option<Self> {
        let mut buf = [0u8; File::MAX_BUF_STACK_SIZE];
        buf[..path.len()].copy_from_slice(path);
        buf[path.len()] = 0;
        unsafe { File::open_nt(&buf, flags) }
    }

    fn open_alloc(path: &[u8], flags: OpenFlags) -> Option<Self> {
        let mut buf = Vec::with_capacity(path.len() + 1);
        buf.extend_from_slice(path);
        buf.push(0);
        unsafe { File::open_nt(&buf, flags) }
    }

    pub fn open<P: AsRef<[u8]>>(path: P, flags: OpenFlags) -> Option<Self> {
        let path = path.as_ref();
        if *path.last()? == 0 {
            unsafe { File::open_nt(path, flags) }
        } else {
            if path.len() < File::MAX_BUF_STACK_SIZE {
                File::open_no_alloc(path, flags)
            } else {
                File::open_alloc(path, flags)
            }
        }
    }

    pub fn seek(&mut self, offset: isize, origin: Origin) -> Option<isize> {
        let res = unsafe {
            lseek64(self.fd, offset as c_longlong, origin as c_int)
        };

        if res < 0 { None } else { Some(res as isize) }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let res = unsafe {
            close(self.fd)
        };
        assert_eq!(res, 0, "Failed to close file descriptor");
    }
}

impl Read for File {
    fn read_bytes(&self, buf: &mut [u8]) -> Option<usize> {
        let result = unsafe {
            libc::read(
                self.fd, 
                buf.as_mut_ptr() as *mut libc::c_void, 
                buf.len() as c_uint
            )
        };
        if result < 0 {
            None
        } else {
            Some(result as usize)
        }
    }
}

impl Write for File {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write!(CWriter::new(self.fd), "{s}")
    }
}