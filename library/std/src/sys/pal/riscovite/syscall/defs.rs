// This file is auto-generated from metadata in the riscovite-meta crate. DO NOT EDIT.

use crate::sys::RawOsError;

/// Struct types used as arguments or results from some system functions.
pub mod structs {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct DirEntry {
        pub resource_kind: u8,
        pub name_len: u8,
        pub name: [u8; 254],
    }
}

/// Miscellaneous constants for use when calling some system functions.
pub mod consts {
    pub const OPEN_DIR: u64 = 0x1;
    pub const OPEN_FILE: u64 = 0x0;
    pub const OPEN_TO_READ: u64 = 0x2;
    pub const OPEN_TO_WRITE: u64 = 0x4;
    pub const OPEN_TO_READWRITE: u64 = 0x6;
    pub const OPEN_TO_EXECUTE: u64 = 0x8;
    pub const OPEN_APPEND: u64 = 0x100;
    pub const OPEN_CREATE: u64 = 0x10000;
    pub const OPEN_NONEXIST: u64 = 0x20000;
    pub const OPEN_TRUNCATE: u64 = 0x40000;
    pub const OPEN_CREATE_READONLY: u64 = 0x1000000;
    pub const OPEN_CREATE_SYSTEM: u64 = 0x2000000;
    pub const TRAVERSE_REQ_EXIST: u64 = 0x1;
    pub const TRAVERSE_REQ_DIR: u64 = 0x3;
    pub const HND_STDIN: u64 = 0x0;
    pub const HND_STDOUT: u64 = 0x1;
    pub const HND_STDERR: u64 = 0x2;
    pub const HND_CWD: u64 = 0x3;
    pub const SEEK_SET: u64 = 0;
    pub const SEEK_CUR: u64 = 1;
    pub const SEEK_END: u64 = 2;
}

/// System functions in the `core` group.
pub mod core {
    /// System function number for `set_heap_size`.
    pub const SYS_SET_HEAP_SIZE: u64 = 0x010;
    /// System function number for `close`.
    pub const SYS_CLOSE: u64 = 0x020;
    /// System function number for `select_interface`.
    pub const SYS_SELECT_INTERFACE: u64 = 0x030;
    /// System function number for `set_timer_interrupt`.
    pub const SYS_SET_TIMER_INTERRUPT: u64 = 0x040;
    /// System function number for `get_current_timestamp`.
    pub const SYS_GET_CURRENT_TIMESTAMP: u64 = 0x050;
    /// System function number for `set_interrupt_priority_floor`.
    pub const SYS_SET_INTERRUPT_PRIORITY_FLOOR: u64 = 0x060;
    /// System function number for `exit`.
    pub const SYS_EXIT: u64 = 0x800;
    /// System function number for `get_heap_base`.
    pub const SYS_GET_HEAP_BASE: u64 = 0x810;
    /// System function number for `get_heap_size`.
    pub const SYS_GET_HEAP_SIZE: u64 = 0x820;
    /// System function number for `set_fault_handler`.
    pub const SYS_SET_FAULT_HANDLER: u64 = 0x830;
    /// System function number for `set_interrupt_stack`.
    pub const SYS_SET_INTERRUPT_STACK: u64 = 0x840;
    /// System function number for `get_interface`.
    pub const SYS_GET_INTERFACE: u64 = 0x850;
} // core

/// System functions in the `io` group.
pub mod io {
    /// System function number for `read`.
    pub const SYS_READ: u64 = 0x011;
    /// System function number for `write`.
    pub const SYS_WRITE: u64 = 0x021;
    /// System function number for `sync`.
    pub const SYS_SYNC: u64 = 0x031;
    /// System function number for `open`.
    pub const SYS_OPEN: u64 = 0x041;
    /// System function number for `open_replace`.
    pub const SYS_OPEN_REPLACE: u64 = 0x051;
    /// System function number for `open_resource`.
    pub const SYS_OPEN_RESOURCE: u64 = 0x061;
    /// System function number for `traverse`.
    pub const SYS_TRAVERSE: u64 = 0x071;
    /// System function number for `traverse_replace`.
    pub const SYS_TRAVERSE_REPLACE: u64 = 0x081;
    /// System function number for `dup`.
    pub const SYS_DUP: u64 = 0x091;
    /// System function number for `read_dir`.
    pub const SYS_READ_DIR: u64 = 0x0a1;
    /// System function number for `reset_read_dir`.
    pub const SYS_RESET_READ_DIR: u64 = 0x0b1;
    /// System function number for `dup_replace`.
    pub const SYS_DUP_REPLACE: u64 = 0x0c1;
    /// System function number for `seek`.
    pub const SYS_SEEK: u64 = 0x0d1;
    /// System function number for `get_dos_path`.
    pub const SYS_GET_DOS_PATH: u64 = 0x801;
} // io

/// System functions in the `process` group.
pub mod process {
    /// System function number for `exec_child`.
    pub const SYS_EXEC_CHILD: u64 = 0x012;
    /// System function number for `exec_replace`.
    pub const SYS_EXEC_REPLACE: u64 = 0x022;
    /// System function number for `monitor_ctx_create`.
    pub const SYS_MONITOR_CTX_CREATE: u64 = 0x032;
    /// System function number for `monitor_ctx_load_exec`.
    pub const SYS_MONITOR_CTX_LOAD_EXEC: u64 = 0x042;
} // process

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
