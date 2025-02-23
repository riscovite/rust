use alloc::sync::Arc;
use core::mem::MaybeUninit;

use super::syscall::{defs, syscall};
use crate::ffi::OsString;
use crate::fmt;
use crate::hash::{Hash, Hasher};
use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut, SeekFrom};
use crate::path::{Path, PathBuf};
use crate::sys::common::small_c_string::run_path_with_cstr;
use crate::sys::os_str::Buf;
use crate::sys::time::SystemTime;
use crate::sys::unsupported;
use crate::sys_common::FromInner;

#[repr(transparent)]
pub struct File(u64);

pub struct FileAttr(!);

struct ReadDirInner {
    hnd_num: u64,
    orig_path: PathBuf,
}

pub struct ReadDir {
    buf: [u8; 512],
    remain: core::ops::Range<usize>,
    shared: Arc<ReadDirInner>,
}

pub struct DirEntry {
    kind: FileType,
    name: OsString,
    shared: Arc<ReadDirInner>,
}

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct OpenOptions(u64);

#[derive(Copy, Clone, Debug, Default)]
pub struct FileTimes {}

pub struct FilePermissions(!);

#[repr(transparent)]
pub struct FileType(u8);

#[derive(Debug)]
pub struct DirBuilder {}

impl FileAttr {
    pub fn size(&self) -> u64 {
        self.0
    }

    pub fn perm(&self) -> FilePermissions {
        self.0
    }

    pub fn file_type(&self) -> FileType {
        self.0
    }

    pub fn modified(&self) -> io::Result<SystemTime> {
        self.0
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        self.0
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        self.0
    }
}

impl Clone for FileAttr {
    fn clone(&self) -> FileAttr {
        self.0
    }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        self.0
    }

    pub fn set_readonly(&mut self, _readonly: bool) {
        self.0
    }
}

impl Clone for FilePermissions {
    fn clone(&self) -> FilePermissions {
        self.0
    }
}

impl PartialEq for FilePermissions {
    fn eq(&self, _other: &FilePermissions) -> bool {
        self.0
    }
}

impl Eq for FilePermissions {}

impl fmt::Debug for FilePermissions {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

impl FileTimes {
    pub fn set_accessed(&mut self, _t: SystemTime) {}
    pub fn set_modified(&mut self, _t: SystemTime) {}
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        self.0 == 2 // "directory-like"
    }

    pub fn is_file(&self) -> bool {
        self.0 == 1 // "file-like"
    }

    pub fn is_symlink(&self) -> bool {
        false // no symlinks on RISCovite
    }
}

impl Clone for FileType {
    fn clone(&self) -> FileType {
        FileType(self.0)
    }
}

impl Copy for FileType {}

impl PartialEq for FileType {
    fn eq(&self, other: &FileType) -> bool {
        self.0 == other.0
    }
}

impl Eq for FileType {}

impl Hash for FileType {
    fn hash<H: Hasher>(&self, h: &mut H) {
        h.write_u8(self.0)
    }
}

impl fmt::Debug for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("FileType").field(&self.0).finish()
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ReadDir").field(&self.shared.hnd_num).finish()
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        if self.remain.is_empty() {
            // We'll try to fill our buffer with some more entries.
            let result = unsafe {
                syscall!(
                    defs::io::SYS_READ_DIR,
                    self.shared.hnd_num,
                    self.buf.as_mut_ptr() as u64,
                    self.buf.len() as u64,
                    0,
                )
                .as_io::<u64>()
            };
            match result {
                Ok(len) => {
                    self.remain = 0..(len as usize);
                }
                Err(e) => return Some(Err(e)),
            }
        }
        if self.remain.is_empty() {
            // If we're still empty even after trying to read then
            // that suggests we've reached the end of the directory.
            return None;
        }
        // If we get here then we should definitely have at least
        // one directory entry in our buffer. A directory entry
        // starts with a "kind" byte and then a "name_len" byte,
        // where the second also tells us how much of the buffer
        // represents our entry.
        let name_len = self.buf[self.remain.start + 1] as usize;
        let entry_len = name_len + 3; // kind, name_len, and null terminator
        let entry_range = self.remain.start..self.remain.start + entry_len;
        self.remain.start += entry_len;
        let entry_raw = &self.buf[entry_range];
        let name_raw = &entry_raw[2..entry_raw.len() - 1];
        let name =
            OsString::from_inner(unsafe { Buf::from_encoded_bytes_unchecked(name_raw.to_vec()) });
        let entry =
            DirEntry { kind: FileType(entry_raw[0]), name, shared: Arc::clone(&self.shared) };
        Some(Ok(entry))
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        let mut base = self.shared.orig_path.clone();
        base.push(self.file_name());
        base
    }

    pub fn file_name(&self) -> OsString {
        self.name.clone()
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        unsupported() // TODO: Actually support this
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        Ok(self.kind)
    }
}

impl OpenOptions {
    #[inline(always)]
    pub fn new() -> OpenOptions {
        OpenOptions(defs::consts::OPEN_FILE)
    }

    #[inline(always)]
    pub fn read(&mut self, read: bool) {
        self.write_mask::<{ defs::consts::OPEN_TO_READ }>(read);
    }

    #[inline(always)]
    pub fn write(&mut self, write: bool) {
        self.write_mask::<{ defs::consts::OPEN_TO_WRITE }>(write);
    }

    #[inline(always)]
    pub fn append(&mut self, append: bool) {
        self.write_mask::<{ defs::consts::OPEN_APPEND }>(append);
    }

    #[inline(always)]
    pub fn truncate(&mut self, truncate: bool) {
        self.write_mask::<{ defs::consts::OPEN_TRUNCATE }>(truncate);
    }

    #[inline(always)]
    pub fn create(&mut self, create: bool) {
        self.write_mask::<{ defs::consts::OPEN_CREATE }>(create);
    }

    #[inline(always)]
    pub fn create_new(&mut self, create_new: bool) {
        self.write_mask::<{ defs::consts::OPEN_NONEXIST }>(create_new);
    }

    fn for_syscall_arg(&self) -> u64 {
        let mut ret = self.0;
        if (ret & defs::consts::OPEN_NONEXIST) != 0 {
            // We use OPEN_NONEXIST to represent create_new, which is
            // documented to cause "create" to be ignored. We "ignore"
            // it by just forcing it to be true in this case, since
            // OPEN_NONEXIST only makes sense in combination with create.
            ret |= defs::consts::OPEN_CREATE;
        }
        ret
    }

    #[inline(always)]
    fn write_mask<const MASK: u64>(&mut self, v: bool) {
        if v {
            self.0 |= MASK;
        } else {
            self.0 &= !MASK;
        }
    }
}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        let opts_raw = opts.for_syscall_arg();
        run_path_with_cstr(path, &|path| {
            unsafe {
                syscall!(defs::io::SYS_OPEN, defs::consts::HND_CWD, path.as_ptr() as u64, opts_raw)
            }
            .as_io::<u64>()
        })
        .map(|hnd_num| Self(hnd_num))
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        Err(io::Error::from_raw_os_error(defs::Error::EOPNOTSUPP.to_raw()))
    }

    #[inline]
    pub fn fsync(&self) -> io::Result<()> {
        unsafe { syscall!(defs::io::SYS_SYNC, self.0,) }.as_io::<u64>().map(|_| ())
    }

    #[inline]
    pub fn datasync(&self) -> io::Result<()> {
        self.fsync()
    }

    pub fn lock(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn lock_shared(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn try_lock(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn try_lock_shared(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn unlock(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn truncate(&self, _size: u64) -> io::Result<()> {
        unsupported()
    }

    #[inline]
    unsafe fn read_raw(&self, ptr: *mut MaybeUninit<u8>, count: usize) -> io::Result<usize> {
        unsafe { syscall!(defs::io::SYS_READ, self.0, ptr as u64, count as u64,) }
            .as_io::<u64>()
            .map(|n| n as usize)
    }

    #[inline]
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        unsafe { self.read_raw(buf.as_mut_ptr().cast(), buf.len()) }
    }

    pub fn read_vectored(&self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        Err(io::Error::from_raw_os_error(defs::Error::EOPNOTSUPP.to_raw()))
    }

    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn read_buf(&self, mut cursor: BorrowedCursor<'_>) -> io::Result<()> {
        let into = unsafe { cursor.as_mut() };
        let written = unsafe { self.read_raw(into.as_mut_ptr(), into.len()) }?;
        unsafe { cursor.advance_unchecked(written) };
        Ok(())
    }

    #[inline]
    unsafe fn write_raw(&self, ptr: *const u8, count: usize) -> io::Result<usize> {
        unsafe { syscall!(defs::io::SYS_WRITE, self.0, ptr as u64, count as u64,) }
            .as_io::<u64>()
            .map(|n| n as usize)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        unsafe { self.write_raw(buf.as_ptr(), buf.len()) }
    }

    pub fn write_vectored(&self, _bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        unsupported()
    }

    pub fn is_write_vectored(&self) -> bool {
        false
    }

    #[inline]
    pub fn flush(&self) -> io::Result<()> {
        self.fsync()
    }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        let (whence, offset) = match pos {
            SeekFrom::Start(n) => (defs::consts::SEEK_SET, n),
            SeekFrom::End(n) => (defs::consts::SEEK_END, n as u64),
            SeekFrom::Current(n) => (defs::consts::SEEK_CUR, n as u64),
        };
        unsafe { syscall!(defs::io::SYS_SEEK, self.0, offset, whence,) }.as_io::<u64>()
    }

    #[inline]
    pub fn duplicate(&self) -> io::Result<File> {
        unsafe { syscall!(defs::io::SYS_DUP, self.0,) }.as_io::<u64>().map(|hn| Self(hn))
    }

    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> {
        unsupported()
    }

    pub fn set_times(&self, _times: FileTimes) -> io::Result<()> {
        unsupported()
    }
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder {}
    }

    pub fn mkdir(&self, _p: &Path) -> io::Result<()> {
        unsupported()
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("File").field(&self.0).finish()
    }
}

pub fn readdir(p: &Path) -> io::Result<ReadDir> {
    let orig_path = p.to_owned();
    let hnd_num = run_path_with_cstr(p, &|path| {
        unsafe {
            syscall!(
                defs::io::SYS_OPEN,
                defs::consts::HND_CWD,
                path.as_ptr() as u64,
                defs::consts::OPEN_DIR
            )
        }
        .as_io::<u64>()
    })?;

    Ok(ReadDir {
        shared: Arc::new(ReadDirInner { hnd_num, orig_path }),
        buf: [0; 512],
        remain: 0..0,
    })
}

pub fn unlink(_p: &Path) -> io::Result<()> {
    unsupported()
}

pub fn rename(_old: &Path, _new: &Path) -> io::Result<()> {
    unsupported()
}

pub fn set_perm(_p: &Path, perm: FilePermissions) -> io::Result<()> {
    match perm.0 {}
}

pub fn rmdir(_p: &Path) -> io::Result<()> {
    unsupported()
}

pub fn remove_dir_all(_path: &Path) -> io::Result<()> {
    unsupported()
}

pub fn exists(_path: &Path) -> io::Result<bool> {
    unsupported()
}

pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn symlink(_original: &Path, _link: &Path) -> io::Result<()> {
    unsupported()
}

pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    unsupported()
}

pub fn stat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn lstat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn canonicalize(p: &Path) -> io::Result<PathBuf> {
    // For RISCovite we implement this by creating a temporary DOS node
    // handle and asking the system for its path.
    let hnd_num = run_path_with_cstr(p, &|path| {
        unsafe {
            syscall!(
                defs::io::SYS_TRAVERSE,
                defs::consts::HND_CWD,
                path.as_ptr() as u64,
                0b01 // must exist, but does not need to be a directory
            )
        }
        .as_io::<u64>()
    })?;
    // After this point we must close hnd_num before we return, even if
    // we fail.
    let ret = super::os::dos_name_for_handle(hnd_num);
    unsafe {
        syscall!(
            defs::core::SYS_CLOSE,
            hnd_num
        );
    };
    ret
}

pub fn copy(_from: &Path, _to: &Path) -> io::Result<u64> {
    unsupported()
}
