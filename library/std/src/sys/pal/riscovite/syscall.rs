//! Raw system calls on RISCovite.
#![allow(dead_code)]
#![allow(unused_imports)]

//pub(super) mod defs;
pub(super) mod defs {
    pub use riscovite::sys::{consts, core, io, process};

    use crate::sys::RawOsError;

    macro_rules! error_codes {
        {
            $($name:ident = $code:literal),+
        } => {
            #[repr(u64)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum Error {
                $($name = $code),+
            }

            impl Error {
                const EAGAIN: Self = Self::EWOULDBLOCK;

                #[inline(always)]
                pub fn to_raw(self) -> RawOsError {
                    self as u64
                }
            }

            pub mod errno {
                $(pub const $name: u64 = $code);+;

                const EAGAIN: u64 = self::EWOULDBLOCK;
            }
        };
    }

    error_codes! {
        EPERM = 1,
        ENOENT = 2,
        ESRCH = 3,
        EINTR = 4,
        EIO = 5,
        ENXIO = 6,
        E2BIG = 7,
        ENOEXEC = 8,
        EBADF = 9,
        ECHILD = 10,
        EWOULDBLOCK = 11,
        ENOMEM = 12,
        EACCES = 13,
        EFAULT = 14,
        EBUSY = 16,
        EEXIST = 17,
        EXDEV = 18,
        ENODEV = 19,
        ENOTDIR = 20,
        EISDIR = 21,
        EINVAL = 22,
        ENFILE = 23,
        EMFILE = 24,
        ENOTTY = 25,
        ETXTBSY = 26,
        EFBIG = 27,
        ENOSPC = 28,
        ESPIPE = 29,
        EROFS = 30,
        EMLINK = 31,
        EPIPE = 32,
        EDOM = 33,
        ERANGE = 34,
        ENOMSG = 35,
        EIDRM = 36,
        EDEADLK = 45,
        ENOLCK = 46,
        ENOSTR = 60,
        ENODATA = 61,
        ETIME = 62,
        ENOSR = 63,
        ENONET = 64,
        ENOLINK = 67,
        EPROTO = 71,
        EMULTIHOP = 74,
        EBADMSG = 77,
        EFTYPE = 79,
        EBADFD = 81,
        ENOSYS = 88,
        ENMFILE = 89,
        ENOTEMPTY = 90,
        ENAMETOOLONG = 91,
        ELOOP = 92,
        EOPNOTSUPP = 95,
        EPFNOSUPPORT = 96,
        ECONNRESET = 104,
        ENOBUFS = 105,
        EAFNOSUPPORT = 106,
        EPROTOTYPE = 107,
        ENOTSOCK = 108,
        ENOPROTOOPT = 109,
        ECONNREFUSED = 111,
        EADDRINUSE = 112,
        ECONNABORTED = 113,
        ENETUNREACH = 114,
        ENETDOWN = 115,
        ETIMEDOUT = 116,
        EHOSTDOWN = 117,
        EHOSTUNREACH = 118,
        EINPROGRESS = 119,
        EALREADY = 120,
        EDESTADDRREQ = 121,
        EMSGSIZE = 122,
        EPROTONOSUPPORT = 123,
        EADDRNOTAVAIL = 125,
        ENETRESET = 126,
        EISCONN = 127,
        ENOTCONN = 128,
        ETOOMANYREFS = 129,
        EDQUOT = 132,
        ESTALE = 133,
        ENOTSUP = 134,
        ECASECLASH = 137,
        EILSEQ = 138,
        EOVERFLOW = 139,
        ECANCELED = 140,
        ENOTRECOVERABLE = 141,
        EOWNERDEAD = 142
    }
}

pub(super) use riscovite::sys::Error;
pub(super) use riscovite::sys::raw::{
    IFACE_SLOT, Result, V, interface_func_num, syscall, syscall0, syscall1, syscall2, syscall3,
    syscall4, syscall5, syscall6, syscall7,
};

#[inline(always)]
pub(super) fn ior<T: From<V>>(r: Result) -> crate::io::Result<T> {
    ior_map(
        r,
        #[inline]
        |v| v.into(),
    )
}

pub(super) fn ior_map<T>(r: Result, m: impl FnOnce(V) -> T) -> crate::io::Result<T> {
    if r.error.0 != 0 {
        crate::io::Result::Err(crate::io::Error::from_raw_os_error(r.error.0))
    } else {
        crate::io::Result::Ok(m(r.value))
    }
}
