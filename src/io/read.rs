use alloc::{string::{FromUtf8Error, String}, vec::Vec};
use libc::read;

#[derive(Debug)]
pub struct Error {
    buf: Vec<u8>
}

impl Error {
    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Self {
            buf: value.into_bytes()
        }
    }
}

type Result<T> = core::result::Result<T, Error>;

pub trait Read {
    fn read_bytes(&self, buf: &mut [u8]) -> Option<usize>;

    fn read_str(&self, mut buf: Vec<u8>) -> Result<String> {
        let Some(len) = self.read_bytes(&mut buf) else { return Err(Error { buf }) };
        buf.truncate(len);
        Ok(String::from_utf8(buf)?)
    }
}

pub struct Stdin;

impl Stdin {
    const FD: libc::c_int = 0;
}

impl Read for Stdin {
    fn read_bytes(&self, buf: &mut [u8]) -> Option<usize> {
        let result = unsafe {
            read(
                Self::FD, 
                buf.as_mut_ptr() as *mut libc::c_void, 
                buf.len() as libc::c_uint
            )
        };
        if result < 0 {
            None
        } else {
            Some(result as usize)
        }
    }
}