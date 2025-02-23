use crate::ffi::OsStr;
use crate::path::{Path, PathBuf, Prefix};
use crate::sys::pal::os::getcwd;
use crate::io;

#[inline]
pub fn is_sep_byte(b: u8) -> bool {
    b == b'/'
}

#[inline]
pub fn is_verbatim_sep(b: u8) -> bool {
    b == b'/'
}

#[inline]
pub fn parse_prefix(raw: &OsStr) -> Option<Prefix<'_>> {
    let raw = raw.as_encoded_bytes();
    for (i, b) in raw.iter().copied().enumerate() {
        if is_sep_byte(b) {
            break; // found a path segment separator before colon
        }
        if b == b':' {
            // Everything before this point seems like a volume name, then.
            let volume_bytes = &raw[..i];
            let volume_name = unsafe { OsStr::from_encoded_bytes_unchecked(volume_bytes) };
            // We borrow the "PrefixNS" variant that was originally intended
            // for the Windows \\.\NAME syntax to represent volume name prefixes
            // on RISCovite targets, since this path API was only really
            // designed to support Unix or Windows.
            // This relies on an OS-specific exception in Prefix::len which
            // makes Prefix::DeviceNS only add one to the "device name", rather
            // than four as on Windows, so that Path::components can understand
            // where the normal path components begin.
            return Some(Prefix::DeviceNS(volume_name));
        }
    }
    None
}

pub const MAIN_SEP_STR: &str = "/";
pub const MAIN_SEP: char = '/';

pub(crate) fn absolute(path: &Path) -> io::Result<PathBuf> {
    if path.is_absolute() {
        return Ok(path.to_owned());
    }

    // If we've been given a relative path then we need to know our
    // current working directory as a starting point.
    let mut ret = getcwd()?;
    ret.extend(path.components());
    Ok(ret)
}
