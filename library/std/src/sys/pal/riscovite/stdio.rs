use super::syscall::{defs, ior_map, syscall};
use crate::io;

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    const NUM: u64 = defs::consts::HND_STDIN;

    #[inline]
    pub const fn new() -> Stdin {
        Stdin
    }
}

impl io::Read for Stdin {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        ior_map(
            unsafe {
                syscall!(defs::io::SYS_READ, Self::NUM, buf.as_mut_ptr() as u64, buf.len() as u64,)
            },
            |v| v as usize,
        )
    }
}

impl Stdout {
    const NUM: u64 = defs::consts::HND_STDOUT;

    #[inline]
    pub const fn new() -> Stdout {
        Stdout
    }
}

impl io::Write for Stdout {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        ior_map(
            unsafe {
                syscall!(defs::io::SYS_WRITE, Self::NUM, buf.as_ptr() as u64, buf.len() as u64,)
            },
            |v| v as usize,
        )
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        ior_map(unsafe { syscall!(defs::io::SYS_SYNC, Self::NUM) }, |_| ())
    }
}

impl Stderr {
    const NUM: u64 = defs::consts::HND_STDERR;

    #[inline]
    pub const fn new() -> Stderr {
        Stderr
    }
}

impl io::Write for Stderr {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        ior_map(
            unsafe {
                syscall!(defs::io::SYS_WRITE, Self::NUM, buf.as_ptr() as u64, buf.len() as u64,)
            },
            |v| v as usize,
        )
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        ior_map(unsafe { syscall!(defs::io::SYS_SYNC, Self::NUM) }, |_| ())
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

pub fn is_ebadf(err: &io::Error) -> bool {
    match err.raw_os_error() {
        Some(code) => code == (defs::Error::EBADF as u64),
        None => false,
    }
}

pub fn panic_output() -> Option<impl io::Write> {
    Some(Stderr::new())
}
