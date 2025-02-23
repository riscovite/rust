use core::ffi::{CStr, c_char, c_int};

use super::syscall::{defs, syscall};
use super::unsupported;
use crate::error::Error as StdError;
use crate::ffi::{OsStr, OsString};
use crate::marker::PhantomData;
use crate::path::{self, PathBuf};
use crate::sys::common::small_c_string::run_path_with_cstr;
use crate::sys::os_str::Buf;
use crate::sys_common::FromInner;
use crate::{fmt, io};

pub fn errno() -> u64 {
    // libc's errno is available only via a C macro, so we can't actually
    // access it from here.
    // TODO: Does this matter? What actually uses this?
    0
}

pub fn error_string(errno: u64) -> String {
    extern "C" {
        fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: libc::size_t) -> c_int;
    }

    let mut buf = [0 as c_char; 64];
    let p = buf.as_mut_ptr();
    unsafe {
        if strerror_r(errno as c_int, p, buf.len()) < 0 {
            panic!("strerror_r failure");
        }

        let p = p as *const _;
        String::from_utf8_lossy(CStr::from_ptr(p).to_bytes()).into()
    }
}

pub(super) fn dos_name_for_handle(hnd_num: u64) -> io::Result<PathBuf> {
    // The RISCovite function for getting the DOS path for a handle requires
    // us to pass a buffer for it to write into, which means we need to
    // predict how much buffer we need. We'll start with a relatively small
    // buffer and then grow if the supervisor tells us we need to.
    let mut buf = Vec::<u8>::with_capacity(128);
    let start_addr = loop {
        let ptr = buf.as_mut_ptr();
        let cap = buf.capacity();
        let result = unsafe {
            syscall!(defs::io::SYS_GET_DOS_PATH, hnd_num, ptr as u64, cap as u64)
        };
        if result.error.is(defs::Error::E2BIG) {
            // Supervisor says that we need a bigger buffer, so we'll grow
            // our buffer and retry. The supervisor might have suggested
            // a specific buffer size to try in result.value.
            let suggestion: usize = result.value.try_into().unwrap_or(0);
            let reserve_count = if suggestion > cap {
                suggestion - cap
            } else {
                // If we didn't get a sensible suggestion then we'll grow by
                // a fixed amount instead. (In practice the supervisor should
                // only return either a reasonable value or return zero to
                // represent that it has no suggestion to make, but we
                // additonally require a value greater than what we already
                // tried to make sure we can make progress even if the
                // supervisor suggests something nonsensical.)
                64
            };
            buf.reserve_exact(reserve_count);
            continue;
        }
        // When successful, get_dos_path returns a pointer to a byte
        // _somewhere_ in the buffer that is the start of a null-terminated
        // string. It's not guaranteed to be at the start of the buffer.
        break result.as_io::<u64>()?;
    };

    // If we get here then start_addr points to somewhere in buf and should
    // be the start of a null-terminated string, so we need to find its length.
    let start_ptr = crate::ptr::with_exposed_provenance::<u8>(start_addr as usize);
    let path_cstr = unsafe { CStr::from_ptr(start_ptr) };
    let path_len = path_cstr.count_bytes();

    let path_osstr = if buf.as_ptr() == start_ptr {
        // If we ended up with the path at the very start of our buffer then
        // we can use our already-allocated buffer directly as the backing
        // data for the path.
        unsafe { buf.set_len(path_len) };
        OsString::from_inner(unsafe { Buf::from_encoded_bytes_unchecked(buf) })
    } else {
        // If the path begins at some other position then we need to allocate
        // a new Vec and copy the returned path into it.
        let new_buf: Vec<u8> = path_cstr.to_bytes().into();
        OsString::from_inner(unsafe { Buf::from_encoded_bytes_unchecked(new_buf) })
    };
    Ok(path_osstr.into())
}

pub fn getcwd() -> io::Result<PathBuf> {
    dos_name_for_handle(defs::consts::HND_CWD)
}

#[inline]
pub fn chdir(p: &path::Path) -> io::Result<()> {
    run_path_with_cstr(p, &|cp| {
        unsafe {
            syscall!(
                defs::io::SYS_TRAVERSE_REPLACE,
                defs::consts::HND_CWD,
                cp.as_ptr() as u64,
                0b11, // must exist, must be directory
            )
        }
        .as_io::<u64>()
        .map(|_| ())
    })
}

pub struct SplitPaths<'a>(!, PhantomData<&'a ()>);

pub fn split_paths(_unparsed: &OsStr) -> SplitPaths<'_> {
    panic!("unsupported")
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        self.0
    }
}

#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(_paths: I) -> Result<OsString, JoinPathsError>
where
    I: Iterator<Item = T>,
    T: AsRef<OsStr>,
{
    Err(JoinPathsError)
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "not supported on this platform yet".fmt(f)
    }
}

impl StdError for JoinPathsError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "not supported on this platform yet"
    }
}

pub fn current_exe() -> io::Result<PathBuf> {
    unsupported()
}

pub struct Env(!);

impl Env {
    // FIXME(https://github.com/rust-lang/rust/issues/114583): Remove this when <OsStr as Debug>::fmt matches <str as Debug>::fmt.
    pub fn str_debug(&self) -> impl fmt::Debug + '_ {
        let Self(inner) = self;
        match *inner {}
    }
}

impl fmt::Debug for Env {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;
        match *inner {}
    }
}

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<(OsString, OsString)> {
        let Self(inner) = self;
        match *inner {}
    }
}

pub fn env() -> Env {
    panic!("not supported on this platform")
}

pub fn getenv(_: &OsStr) -> Option<OsString> {
    None
}

pub unsafe fn setenv(_: &OsStr, _: &OsStr) -> io::Result<()> {
    Err(io::const_error!(io::ErrorKind::Unsupported, "cannot set env vars on this platform"))
}

pub unsafe fn unsetenv(_: &OsStr) -> io::Result<()> {
    Err(io::const_error!(io::ErrorKind::Unsupported, "cannot unset env vars on this platform"))
}

pub fn temp_dir() -> PathBuf {
    panic!("no filesystem on this platform")
}

pub fn home_dir() -> Option<PathBuf> {
    None
}

pub fn exit(code: i32) -> ! {
    // We delegate to libc here to make sure that we deal with atexit, etc.
    unsafe { libc::exit(code) }
}

pub fn getpid() -> u32 {
    panic!("no pids on this platform")
}
