use libc::*;

pub struct CWriter{
    fd: i32
}

impl CWriter {
    pub fn new(fd: i32) -> Self {
        Self { fd }
    }

    pub fn stdout() -> Self {
        Self::new(1)
    }

    pub fn stderr() -> Self {
        Self::new(2)
    }
}

impl core::fmt::Write for CWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let count = s.len() as u32;
        let buf = s.as_ptr() as *const c_void;
        let bytes_written = unsafe {
            libc::write(self.fd, buf, count)
        };
        if bytes_written == -1 || bytes_written as u32 != count {
            Err(core::fmt::Error)
        } else {
            Ok(())
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            write!($crate::io::CWriter::stdout(), "{}", format_args!($($arg)*)).unwrap();
        }
    };
}

#[macro_export]
macro_rules! println {
    () => {
        {
            use core::fmt::Write;
            writeln!($crate::io::CWriter::stdout()).unwrap()
        }
    };

    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            writeln!($crate::io::CWriter::stdout(), "{}", format_args!($($arg)*)).unwrap();
        }
    };
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            write!($crate::io::CWriter::stderr(), "{}", format_args!($($arg)*)).unwrap();
        }
    };
}

#[macro_export]
macro_rules! eprintln {
    () => {
        {
            use core::fmt::Write;
            writeln!($crate::io::CWriter::stderr()).unwrap()
        }
    };

    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            writeln!($crate::io::CWriter::stderr(), "{}", format_args!($($arg)*)).unwrap();
        }
    };
}

#[macro_export]
macro_rules! dbg {
    () => {
        $crate::eprintln!("[{}:{}:{}]", file!(), line!(), column!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                $crate::eprintln!("[{}:{}:{}] {} = {:#?}",
                    file!(), line!(), column!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}