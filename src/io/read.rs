use core::str::Utf8Error;

use alloc::{string::{FromUtf8Error, String}, vec::Vec};
use libc::read;

#[derive(Debug)]
pub struct Error {
    buf: Vec<u8>,
    utf8_error: Option<Utf8Error>
}

impl Error {
    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }

    pub fn utf8_error(&self) -> Option<Utf8Error> {
        self.utf8_error
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Self {
            utf8_error: Some(value.utf8_error()),
            buf: value.into_bytes()
        }
    }
}

type Result<T> = core::result::Result<T, Error>;

pub trait Read {
    fn read_bytes(&self, buf: &mut [u8]) -> Option<usize>;

    fn read_str(&self, mut buf: Vec<u8>) -> Result<String> {
        let Some(len) = self.read_bytes(&mut buf) else { 
            return Err(Error { buf, utf8_error: None }) 
        };
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