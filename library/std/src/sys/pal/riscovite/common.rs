use crate::io as std_io;

use super::syscall::defs;

// SAFETY: must be called only once during runtime initialization.
// NOTE: this is not guaranteed to run, for example when Rust code is called externally.
pub unsafe fn init(_argc: isize, _argv: *const *const u8, _sigpipe: u8) {}

// SAFETY: must be called only once during runtime cleanup.
// NOTE: this is not guaranteed to run, for example when the program aborts.
pub unsafe fn cleanup() {}

pub fn unsupported<T>() -> std_io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> std_io::Error {
    std_io::Error::UNSUPPORTED_PLATFORM
}

pub fn is_interrupted(code: u64) -> bool {
    code == defs::errno::EINTR
}

pub fn decode_error_kind(code: u64) -> crate::io::ErrorKind {
    use defs::errno;
    use crate::io::ErrorKind;
    match code {
        errno::ENOENT => ErrorKind::NotFound,
        errno::EPERM | errno::EACCES => ErrorKind::PermissionDenied,
        errno::ECONNREFUSED => ErrorKind::ConnectionRefused,
        errno::ECONNRESET => ErrorKind::ConnectionReset,
        errno::ECONNABORTED => ErrorKind::ConnectionAborted,
        errno::EHOSTUNREACH => ErrorKind::HostUnreachable,
        errno::ENETUNREACH => ErrorKind::NetworkUnreachable,
        errno::ENOTCONN => ErrorKind::NotConnected,
        errno::EADDRINUSE => ErrorKind::AddrInUse,
        errno::EADDRNOTAVAIL => ErrorKind::AddrNotAvailable,
        errno::ENETDOWN => ErrorKind::NetworkDown,
        errno::EPIPE => ErrorKind::BrokenPipe,
        errno::EEXIST => ErrorKind::AlreadyExists,
        errno::EWOULDBLOCK => ErrorKind::WouldBlock,
        errno::ENOTDIR => ErrorKind::NotADirectory,
        errno::EISDIR => ErrorKind::IsADirectory,
        errno::ENOTEMPTY => ErrorKind::DirectoryNotEmpty,
        errno::EROFS => ErrorKind::ReadOnlyFilesystem,
        errno::ELOOP => ErrorKind::FilesystemLoop,
        errno::ESTALE => ErrorKind::StaleNetworkFileHandle,
        errno::EINVAL => ErrorKind::InvalidInput,
        errno::ETIMEDOUT => ErrorKind::TimedOut,
        errno::ENOSPC => ErrorKind::StorageFull,
        errno::ESPIPE => ErrorKind::NotSeekable,
        errno::EDQUOT => ErrorKind::QuotaExceeded,
        errno::EFBIG => ErrorKind::FileTooLarge,
        errno::EBUSY => ErrorKind::ResourceBusy,
        errno::ETXTBSY => ErrorKind::ExecutableFileBusy,
        errno::EDEADLK => ErrorKind::Deadlock,
        errno::EXDEV => ErrorKind::CrossesDevices,
        errno::EMLINK => ErrorKind::TooManyLinks,
        errno::ENAMETOOLONG => ErrorKind::InvalidFilename,
        errno::E2BIG => ErrorKind::ArgumentListTooLong,
        errno::EINTR => ErrorKind::Interrupted,
        errno::ENOSYS | errno::ENOTSUP | errno::EOPNOTSUPP => ErrorKind::Unsupported,
        errno::ENOMEM => ErrorKind::OutOfMemory,
        errno::EINPROGRESS => ErrorKind::InProgress,
        _ => crate::io::ErrorKind::Uncategorized,
    }
}

pub fn abort_internal() -> ! {
    core::intrinsics::abort();
}
