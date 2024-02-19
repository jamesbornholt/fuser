use libc::{c_int, ENOSYS, EPERM};
use log::{debug, warn};
use std::cell::RefCell;
use std::ffi::OsStr;
use std::path::Path;
use std::time::SystemTime;

use crate::ll::TimeOrNow;
use crate::KernelConfig;
#[cfg(feature = "abi-7-16")]
use crate::ll::fuse_abi::fuse_forget_one;
#[cfg(feature = "abi-7-11")]
use crate::reply::ReplyPoll;
#[cfg(target_os = "macos")]
use crate::reply::ReplyXTimes;
use crate::reply::ReplyXattr;
use crate::reply::{ReplyAttr, ReplyData, ReplyEmpty, ReplyEntry, ReplyOpen};
use crate::reply::{
    ReplyBmap, ReplyCreate, ReplyDirectory, ReplyDirectoryPlus, ReplyIoctl, ReplyLock, ReplyLseek,
    ReplyStatfs, ReplyWrite,
};
use crate::request::Request;

macro_rules! filesystem_trait {
    ($name:ident, $self: ty) => {
/// Filesystem trait.
///
/// This trait must be implemented to provide a userspace filesystem via FUSE.
/// These methods correspond to fuse_lowlevel_ops in libfuse. Reasonable default
/// implementations are provided here to get a mountable filesystem that does
/// nothing.
#[allow(clippy::too_many_arguments)]
pub trait $name {
    /// Initialize filesystem.
    /// Called before any other filesystem method.
    /// The kernel module connection can be configured using the KernelConfig object
    fn init(self: $self, _req: &Request<'_>, _config: &mut KernelConfig) -> Result<(), c_int> {
        Ok(())
    }

    /// Clean up filesystem.
    /// Called on filesystem exit.
    fn destroy(self: $self) {}

    /// Look up a directory entry by name and get its attributes.
    fn lookup(self: $self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEntry) {
        warn!(
            "[Not Implemented] lookup(parent: {:#x?}, name {:?})",
            parent, name
        );
        reply.error(ENOSYS);
    }

    /// Forget about an inode.
    /// The nlookup parameter indicates the number of lookups previously performed on
    /// this inode. If the filesystem implements inode lifetimes, it is recommended that
    /// inodes acquire a single reference on each lookup, and lose nlookup references on
    /// each forget. The filesystem may ignore forget calls, if the inodes don't need to
    /// have a limited lifetime. On unmount it is not guaranteed, that all referenced
    /// inodes will receive a forget message.
    fn forget(self: $self, _req: &Request<'_>, _ino: u64, _nlookup: u64) {}

    /// Like forget, but take multiple forget requests at once for performance. The default
    /// implementation will fallback to forget.
    #[cfg(feature = "abi-7-16")]
    fn batch_forget(self: $self, req: &Request<'_>, nodes: &[fuse_forget_one]) {
        for node in nodes {
            self.forget(req, node.nodeid, node.nlookup);
        }
    }

    /// Get file attributes.
    fn getattr(self: $self, _req: &Request<'_>, ino: u64, reply: ReplyAttr) {
        warn!("[Not Implemented] getattr(ino: {:#x?})", ino);
        reply.error(ENOSYS);
    }

    /// Set file attributes.
    fn setattr(
        self: $self,
        _req: &Request<'_>,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        _atime: Option<TimeOrNow>,
        _mtime: Option<TimeOrNow>,
        _ctime: Option<SystemTime>,
        fh: Option<u64>,
        _crtime: Option<SystemTime>,
        _chgtime: Option<SystemTime>,
        _bkuptime: Option<SystemTime>,
        flags: Option<u32>,
        reply: ReplyAttr,
    ) {
        debug!(
            "[Not Implemented] setattr(ino: {:#x?}, mode: {:?}, uid: {:?}, \
            gid: {:?}, size: {:?}, fh: {:?}, flags: {:?})",
            ino, mode, uid, gid, size, fh, flags
        );
        reply.error(ENOSYS);
    }

    /// Read symbolic link.
    fn readlink(self: $self, _req: &Request<'_>, ino: u64, reply: ReplyData) {
        debug!("[Not Implemented] readlink(ino: {:#x?})", ino);
        reply.error(ENOSYS);
    }

    /// Create file node.
    /// Create a regular file, character device, block device, fifo or socket node.
    fn mknod(
        self: $self,
        _req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        mode: u32,
        umask: u32,
        rdev: u32,
        reply: ReplyEntry,
    ) {
        debug!(
            "[Not Implemented] mknod(parent: {:#x?}, name: {:?}, mode: {}, \
            umask: {:#x?}, rdev: {})",
            parent, name, mode, umask, rdev
        );
        reply.error(ENOSYS);
    }

    /// Create a directory.
    fn mkdir(
        self: $self,
        _req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        mode: u32,
        umask: u32,
        reply: ReplyEntry,
    ) {
        debug!(
            "[Not Implemented] mkdir(parent: {:#x?}, name: {:?}, mode: {}, umask: {:#x?})",
            parent, name, mode, umask
        );
        reply.error(ENOSYS);
    }

    /// Remove a file.
    fn unlink(self: $self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        debug!(
            "[Not Implemented] unlink(parent: {:#x?}, name: {:?})",
            parent, name,
        );
        reply.error(ENOSYS);
    }

    /// Remove a directory.
    fn rmdir(self: $self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        debug!(
            "[Not Implemented] rmdir(parent: {:#x?}, name: {:?})",
            parent, name,
        );
        reply.error(ENOSYS);
    }

    /// Create a symbolic link.
    fn symlink(
        self: $self,
        _req: &Request<'_>,
        parent: u64,
        link_name: &OsStr,
        target: &Path,
        reply: ReplyEntry,
    ) {
        debug!(
            "[Not Implemented] symlink(parent: {:#x?}, link_name: {:?}, target: {:?})",
            parent, link_name, target,
        );
        reply.error(EPERM);
    }

    /// Rename a file.
    fn rename(
        self: $self,
        _req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        newparent: u64,
        newname: &OsStr,
        flags: u32,
        reply: ReplyEmpty,
    ) {
        debug!(
            "[Not Implemented] rename(parent: {:#x?}, name: {:?}, newparent: {:#x?}, \
            newname: {:?}, flags: {})",
            parent, name, newparent, newname, flags,
        );
        reply.error(ENOSYS);
    }

    /// Create a hard link.
    fn link(
        self: $self,
        _req: &Request<'_>,
        ino: u64,
        newparent: u64,
        newname: &OsStr,
        reply: ReplyEntry,
    ) {
        debug!(
            "[Not Implemented] link(ino: {:#x?}, newparent: {:#x?}, newname: {:?})",
            ino, newparent, newname
        );
        reply.error(EPERM);
    }

    /// Open a file.
    /// Open flags (with the exception of O_CREAT, O_EXCL, O_NOCTTY and O_TRUNC) are
    /// available in flags. Filesystem may store an arbitrary file handle (pointer, index,
    /// etc) in fh, and use this in other all other file operations (read, write, flush,
    /// release, fsync). Filesystem may also implement stateless file I/O and not store
    /// anything in fh. There are also some flags (direct_io, keep_cache) which the
    /// filesystem may set, to change the way the file is opened. See fuse_file_info
    /// structure in <fuse_common.h> for more details.
    fn open(self: $self, _req: &Request<'_>, _ino: u64, _flags: i32, reply: ReplyOpen) {
        reply.opened(0, 0);
    }

    /// Read data.
    /// Read should send exactly the number of bytes requested except on EOF or error,
    /// otherwise the rest of the data will be substituted with zeroes. An exception to
    /// this is when the file has been opened in 'direct_io' mode, in which case the
    /// return value of the read system call will reflect the return value of this
    /// operation. fh will contain the value set by the open method, or will be undefined
    /// if the open method didn't set any value.
    ///
    /// flags: these are the file flags, such as O_SYNC. Only supported with ABI >= 7.9
    /// lock_owner: only supported with ABI >= 7.9
    fn read(
        self: $self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        size: u32,
        flags: i32,
        lock_owner: Option<u64>,
        reply: ReplyData,
    ) {
        warn!(
            "[Not Implemented] read(ino: {:#x?}, fh: {}, offset: {}, size: {}, \
            flags: {:#x?}, lock_owner: {:?})",
            ino, fh, offset, size, flags, lock_owner
        );
        reply.error(ENOSYS);
    }

    /// Write data.
    /// Write should return exactly the number of bytes requested except on error. An
    /// exception to this is when the file has been opened in 'direct_io' mode, in
    /// which case the return value of the write system call will reflect the return
    /// value of this operation. fh will contain the value set by the open method, or
    /// will be undefined if the open method didn't set any value.
    ///
    /// write_flags: will contain FUSE_WRITE_CACHE, if this write is from the page cache. If set,
    /// the pid, uid, gid, and fh may not match the value that would have been sent if write cachin
    /// is disabled
    /// flags: these are the file flags, such as O_SYNC. Only supported with ABI >= 7.9
    /// lock_owner: only supported with ABI >= 7.9
    fn write(
        self: $self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        data: &[u8],
        write_flags: u32,
        flags: i32,
        lock_owner: Option<u64>,
        reply: ReplyWrite,
    ) {
        debug!(
            "[Not Implemented] write(ino: {:#x?}, fh: {}, offset: {}, data.len(): {}, \
            write_flags: {:#x?}, flags: {:#x?}, lock_owner: {:?})",
            ino,
            fh,
            offset,
            data.len(),
            write_flags,
            flags,
            lock_owner
        );
        reply.error(ENOSYS);
    }

    /// Flush method.
    /// This is called on each close() of the opened file. Since file descriptors can
    /// be duplicated (dup, dup2, fork), for one open call there may be many flush
    /// calls. Filesystems shouldn't assume that flush will always be called after some
    /// writes, or that if will be called at all. fh will contain the value set by the
    /// open method, or will be undefined if the open method didn't set any value.
    /// NOTE: the name of the method is misleading, since (unlike fsync) the filesystem
    /// is not forced to flush pending writes. One reason to flush data, is if the
    /// filesystem wants to return write errors. If the filesystem supports file locking
    /// operations (setlk, getlk) it should remove all locks belonging to 'lock_owner'.
    fn flush(self: $self, _req: &Request<'_>, ino: u64, fh: u64, lock_owner: u64, reply: ReplyEmpty) {
        debug!(
            "[Not Implemented] flush(ino: {:#x?}, fh: {}, lock_owner: {:?})",
            ino, fh, lock_owner
        );
        reply.error(ENOSYS);
    }

    /// Release an open file.
    /// Release is called when there are no more references to an open file: all file
    /// descriptors are closed and all memory mappings are unmapped. For every open
    /// call there will be exactly one release call. The filesystem may reply with an
    /// error, but error values are not returned to close() or munmap() which triggered
    /// the release. fh will contain the value set by the open method, or will be undefined
    /// if the open method didn't set any value. flags will contain the same flags as for
    /// open.
    fn release(
        self: $self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _flags: i32,
        _lock_owner: Option<u64>,
        _flush: bool,
        reply: ReplyEmpty,
    ) {
        reply.ok();
    }

    /// Synchronize file contents.
    /// If the datasync parameter is non-zero, then only the user data should be flushed,
    /// not the meta data.
    fn fsync(self: $self, _req: &Request<'_>, ino: u64, fh: u64, datasync: bool, reply: ReplyEmpty) {
        debug!(
            "[Not Implemented] fsync(ino: {:#x?}, fh: {}, datasync: {})",
            ino, fh, datasync
        );
        reply.error(ENOSYS);
    }

    /// Open a directory.
    /// Filesystem may store an arbitrary file handle (pointer, index, etc) in fh, and
    /// use this in other all other directory stream operations (readdir, releasedir,
    /// fsyncdir). Filesystem may also implement stateless directory I/O and not store
    /// anything in fh, though that makes it impossible to implement standard conforming
    /// directory stream operations in case the contents of the directory can change
    /// between opendir and releasedir.
    fn opendir(self: $self, _req: &Request<'_>, _ino: u64, _flags: i32, reply: ReplyOpen) {
        reply.opened(0, 0);
    }

    /// Read directory.
    /// Send a buffer filled using buffer.fill(), with size not exceeding the
    /// requested size. Send an empty buffer on end of stream. fh will contain the
    /// value set by the opendir method, or will be undefined if the opendir method
    /// didn't set any value.
    fn readdir(
        self: $self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        reply: ReplyDirectory,
    ) {
        warn!(
            "[Not Implemented] readdir(ino: {:#x?}, fh: {}, offset: {})",
            ino, fh, offset
        );
        reply.error(ENOSYS);
    }

    /// Read directory.
    /// Send a buffer filled using buffer.fill(), with size not exceeding the
    /// requested size. Send an empty buffer on end of stream. fh will contain the
    /// value set by the opendir method, or will be undefined if the opendir method
    /// didn't set any value.
    fn readdirplus(
        self: $self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        reply: ReplyDirectoryPlus,
    ) {
        debug!(
            "[Not Implemented] readdirplus(ino: {:#x?}, fh: {}, offset: {})",
            ino, fh, offset
        );
        reply.error(ENOSYS);
    }

    /// Release an open directory.
    /// For every opendir call there will be exactly one releasedir call. fh will
    /// contain the value set by the opendir method, or will be undefined if the
    /// opendir method didn't set any value.
    fn releasedir(
        self: $self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _flags: i32,
        reply: ReplyEmpty,
    ) {
        reply.ok();
    }

    /// Synchronize directory contents.
    /// If the datasync parameter is set, then only the directory contents should
    /// be flushed, not the meta data. fh will contain the value set by the opendir
    /// method, or will be undefined if the opendir method didn't set any value.
    fn fsyncdir(
        self: $self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        datasync: bool,
        reply: ReplyEmpty,
    ) {
        debug!(
            "[Not Implemented] fsyncdir(ino: {:#x?}, fh: {}, datasync: {})",
            ino, fh, datasync
        );
        reply.error(ENOSYS);
    }

    /// Get file system statistics.
    fn statfs(self: $self, _req: &Request<'_>, _ino: u64, reply: ReplyStatfs) {
        reply.statfs(0, 0, 0, 0, 0, 512, 255, 0);
    }

    /// Set an extended attribute.
    fn setxattr(
        self: $self,
        _req: &Request<'_>,
        ino: u64,
        name: &OsStr,
        _value: &[u8],
        flags: i32,
        position: u32,
        reply: ReplyEmpty,
    ) {
        debug!(
            "[Not Implemented] setxattr(ino: {:#x?}, name: {:?}, flags: {:#x?}, position: {})",
            ino, name, flags, position
        );
        reply.error(ENOSYS);
    }

    /// Get an extended attribute.
    /// If `size` is 0, the size of the value should be sent with `reply.size()`.
    /// If `size` is not 0, and the value fits, send it with `reply.data()`, or
    /// `reply.error(ERANGE)` if it doesn't.
    fn getxattr(
        self: $self,
        _req: &Request<'_>,
        ino: u64,
        name: &OsStr,
        size: u32,
        reply: ReplyXattr,
    ) {
        debug!(
            "[Not Implemented] getxattr(ino: {:#x?}, name: {:?}, size: {})",
            ino, name, size
        );
        reply.error(ENOSYS);
    }

    /// List extended attribute names.
    /// If `size` is 0, the size of the value should be sent with `reply.size()`.
    /// If `size` is not 0, and the value fits, send it with `reply.data()`, or
    /// `reply.error(ERANGE)` if it doesn't.
    fn listxattr(self: $self, _req: &Request<'_>, ino: u64, size: u32, reply: ReplyXattr) {
        debug!(
            "[Not Implemented] listxattr(ino: {:#x?}, size: {})",
            ino, size
        );
        reply.error(ENOSYS);
    }

    /// Remove an extended attribute.
    fn removexattr(self: $self, _req: &Request<'_>, ino: u64, name: &OsStr, reply: ReplyEmpty) {
        debug!(
            "[Not Implemented] removexattr(ino: {:#x?}, name: {:?})",
            ino, name
        );
        reply.error(ENOSYS);
    }

    /// Check file access permissions.
    /// This will be called for the access() system call. If the 'default_permissions'
    /// mount option is given, this method is not called. This method is not called
    /// under Linux kernel versions 2.4.x
    fn access(self: $self, _req: &Request<'_>, ino: u64, mask: i32, reply: ReplyEmpty) {
        debug!("[Not Implemented] access(ino: {:#x?}, mask: {})", ino, mask);
        reply.error(ENOSYS);
    }

    /// Create and open a file.
    /// If the file does not exist, first create it with the specified mode, and then
    /// open it. Open flags (with the exception of O_NOCTTY) are available in flags.
    /// Filesystem may store an arbitrary file handle (pointer, index, etc) in fh,
    /// and use this in other all other file operations (read, write, flush, release,
    /// fsync). There are also some flags (direct_io, keep_cache) which the
    /// filesystem may set, to change the way the file is opened. See fuse_file_info
    /// structure in <fuse_common.h> for more details. If this method is not
    /// implemented or under Linux kernel versions earlier than 2.6.15, the mknod()
    /// and open() methods will be called instead.
    fn create(
        self: $self,
        _req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        mode: u32,
        umask: u32,
        flags: i32,
        reply: ReplyCreate,
    ) {
        debug!(
            "[Not Implemented] create(parent: {:#x?}, name: {:?}, mode: {}, umask: {:#x?}, \
            flags: {:#x?})",
            parent, name, mode, umask, flags
        );
        reply.error(ENOSYS);
    }

    /// Test for a POSIX file lock.
    fn getlk(
        self: $self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        lock_owner: u64,
        start: u64,
        end: u64,
        typ: i32,
        pid: u32,
        reply: ReplyLock,
    ) {
        debug!(
            "[Not Implemented] getlk(ino: {:#x?}, fh: {}, lock_owner: {}, start: {}, \
            end: {}, typ: {}, pid: {})",
            ino, fh, lock_owner, start, end, typ, pid
        );
        reply.error(ENOSYS);
    }

    /// Acquire, modify or release a POSIX file lock.
    /// For POSIX threads (NPTL) there's a 1-1 relation between pid and owner, but
    /// otherwise this is not always the case.  For checking lock ownership,
    /// 'fi->owner' must be used. The l_pid field in 'struct flock' should only be
    /// used to fill in this field in getlk(). Note: if the locking methods are not
    /// implemented, the kernel will still allow file locking to work locally.
    /// Hence these are only interesting for network filesystems and similar.
    fn setlk(
        self: $self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        lock_owner: u64,
        start: u64,
        end: u64,
        typ: i32,
        pid: u32,
        sleep: bool,
        reply: ReplyEmpty,
    ) {
        debug!(
            "[Not Implemented] setlk(ino: {:#x?}, fh: {}, lock_owner: {}, start: {}, \
            end: {}, typ: {}, pid: {}, sleep: {})",
            ino, fh, lock_owner, start, end, typ, pid, sleep
        );
        reply.error(ENOSYS);
    }

    /// Map block index within file to block index within device.
    /// Note: This makes sense only for block device backed filesystems mounted
    /// with the 'blkdev' option
    fn bmap(self: $self, _req: &Request<'_>, ino: u64, blocksize: u32, idx: u64, reply: ReplyBmap) {
        debug!(
            "[Not Implemented] bmap(ino: {:#x?}, blocksize: {}, idx: {})",
            ino, blocksize, idx,
        );
        reply.error(ENOSYS);
    }

    /// control device
    fn ioctl(
        self: $self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        flags: u32,
        cmd: u32,
        in_data: &[u8],
        out_size: u32,
        reply: ReplyIoctl,
    ) {
        debug!(
            "[Not Implemented] ioctl(ino: {:#x?}, fh: {}, flags: {}, cmd: {}, \
            in_data.len(): {}, out_size: {})",
            ino,
            fh,
            flags,
            cmd,
            in_data.len(),
            out_size,
        );
        reply.error(ENOSYS);
    }

    /// Poll for events
    #[cfg(feature = "abi-7-11")]
    fn poll(
        self: $self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        kh: u64,
        events: u32,
        flags: u32,
        reply: ReplyPoll,
    ) {
        debug!(
            "[Not Implemented] poll(ino: {:#x?}, fh: {}, kh: {}, events: {}, flags: {})",
            ino, fh, kh, events, flags
        );
        reply.error(ENOSYS);
    }

    /// Preallocate or deallocate space to a file
    fn fallocate(
        self: $self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        length: i64,
        mode: i32,
        reply: ReplyEmpty,
    ) {
        debug!(
            "[Not Implemented] fallocate(ino: {:#x?}, fh: {}, offset: {}, \
            length: {}, mode: {})",
            ino, fh, offset, length, mode
        );
        reply.error(ENOSYS);
    }

    /// Reposition read/write file offset
    fn lseek(
        self: $self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        whence: i32,
        reply: ReplyLseek,
    ) {
        debug!(
            "[Not Implemented] lseek(ino: {:#x?}, fh: {}, offset: {}, whence: {})",
            ino, fh, offset, whence
        );
        reply.error(ENOSYS);
    }

    /// Copy the specified range from the source inode to the destination inode
    fn copy_file_range(
        self: $self,
        _req: &Request<'_>,
        ino_in: u64,
        fh_in: u64,
        offset_in: i64,
        ino_out: u64,
        fh_out: u64,
        offset_out: i64,
        len: u64,
        flags: u32,
        reply: ReplyWrite,
    ) {
        debug!(
            "[Not Implemented] copy_file_range(ino_in: {:#x?}, fh_in: {}, \
            offset_in: {}, ino_out: {:#x?}, fh_out: {}, offset_out: {}, \
            len: {}, flags: {})",
            ino_in, fh_in, offset_in, ino_out, fh_out, offset_out, len, flags
        );
        reply.error(ENOSYS);
    }

    /// macOS only: Rename the volume. Set fuse_init_out.flags during init to
    /// FUSE_VOL_RENAME to enable
    #[cfg(target_os = "macos")]
    fn setvolname(self: $self, _req: &Request<'_>, name: &OsStr, reply: ReplyEmpty) {
        debug!("[Not Implemented] setvolname(name: {:?})", name);
        reply.error(ENOSYS);
    }

    /// macOS only (undocumented)
    #[cfg(target_os = "macos")]
    fn exchange(
        self: $self,
        _req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        newparent: u64,
        newname: &OsStr,
        options: u64,
        reply: ReplyEmpty,
    ) {
        debug!(
            "[Not Implemented] exchange(parent: {:#x?}, name: {:?}, newparent: {:#x?}, \
            newname: {:?}, options: {})",
            parent, name, newparent, newname, options
        );
        reply.error(ENOSYS);
    }

    /// macOS only: Query extended times (bkuptime and crtime). Set fuse_init_out.flags
    /// during init to FUSE_XTIMES to enable
    #[cfg(target_os = "macos")]
    fn getxtimes(self: $self, _req: &Request<'_>, ino: u64, reply: ReplyXTimes) {
        debug!("[Not Implemented] getxtimes(ino: {:#x?})", ino);
        reply.error(ENOSYS);
    }
}
    };
}

filesystem_trait!(Filesystem, &mut Self);
filesystem_trait!(SharedFilesystem, &Self);

#[derive(Debug)]
pub(crate) struct FilesystemAdapter<FS: Filesystem> {
    fs: RefCell<FS>,
}

impl<FS: Filesystem> FilesystemAdapter<FS> {
    pub fn new(fs: FS) -> Self {
        FilesystemAdapter {
            fs: RefCell::new(fs),
        }
    }
}

impl<FS: Filesystem> SharedFilesystem for FilesystemAdapter<FS> {
    fn init(&self, req: &Request<'_>, config: &mut KernelConfig) -> Result<(), c_int> {
        self.fs.borrow_mut().init(req, config)
    }

    fn destroy(&self) {
        self.fs.borrow_mut().destroy()
    }

    fn lookup(&self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEntry) {
        self.fs.borrow_mut().lookup(_req, parent, name, reply)
    }

    fn forget(&self, _req: &Request<'_>, _ino: u64, _nlookup: u64) {
        self.fs.borrow_mut().forget(_req, _ino, _nlookup)
    }

    #[cfg(feature = "abi-7-16")]
    fn batch_forget(&self, req: &Request<'_>, nodes: &[fuse_forget_one]) {
        self.fs.borrow_mut().batch_forget(req, nodes)
    }

    fn getattr(&self, _req: &Request<'_>, ino: u64, reply: ReplyAttr) {
        self.fs.borrow_mut().getattr(_req, ino, reply)
    }

    fn setattr(
        &self,
        _req: &Request<'_>,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        _atime: Option<TimeOrNow>,
        _mtime: Option<TimeOrNow>,
        _ctime: Option<SystemTime>,
        fh: Option<u64>,
        _crtime: Option<SystemTime>,
        _chgtime: Option<SystemTime>,
        _bkuptime: Option<SystemTime>,
        flags: Option<u32>,
        reply: ReplyAttr,
    ) {
        self.fs
            .borrow_mut()
            .setattr(_req, ino, mode, uid, gid, size, _atime, _mtime, _ctime, fh, _crtime, _chgtime, _bkuptime, flags, reply)
    }

    fn readlink(&self, _req: &Request<'_>, ino: u64, reply: ReplyData) {
        self.fs.borrow_mut().readlink(_req, ino, reply)
    }

    fn mknod(
        &self,
        _req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        mode: u32,
        umask: u32,
        rdev: u32,
        reply: ReplyEntry,
    ) {
        self.fs
            .borrow_mut()
            .mknod(_req, parent, name, mode, umask, rdev, reply)
    }

    fn mkdir(
        &self,
        _req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        mode: u32,
        umask: u32,
        reply: ReplyEntry,
    ) {
        self.fs
            .borrow_mut()
            .mkdir(_req, parent, name, mode, umask, reply)
    }

    fn unlink(&self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        self.fs.borrow_mut().unlink(_req, parent, name, reply)
    }

    fn rmdir(&self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        self.fs.borrow_mut().rmdir(_req, parent, name, reply)
    }

    fn symlink(
        &self,
        _req: &Request<'_>,
        parent: u64,
        link_name: &OsStr,
        target: &Path,
        reply: ReplyEntry,
    ) {
        self.fs
            .borrow_mut()
            .symlink(_req, parent, link_name, target, reply)
    }

    fn rename(
        &self,
        _req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        newparent: u64,
        newname: &OsStr,
        flags: u32,
        reply: ReplyEmpty,
    ) {
        self.fs
            .borrow_mut()
            .rename(_req, parent, name, newparent, newname, flags, reply)
    }

    fn link(
        &self,
        _req: &Request<'_>,
        ino: u64,
        newparent: u64,
        newname: &OsStr,
        reply: ReplyEntry,
    ) {
        self.fs.borrow_mut().link(_req, ino, newparent, newname, reply)
    }

    fn open(&self, _req: &Request<'_>, _ino: u64, _flags: i32, reply: ReplyOpen) {
        self.fs.borrow_mut().open(_req, _ino, _flags, reply)
    }

    fn read(
        &self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        size: u32,
        flags: i32,
        lock_owner: Option<u64>,
        reply: ReplyData,
    ) {
        self.fs
            .borrow_mut()
            .read(_req, ino, fh, offset, size, flags, lock_owner, reply)
    }

    fn write(
        &self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        data: &[u8],
        write_flags: u32,
        flags: i32,
        lock_owner: Option<u64>,
        reply: ReplyWrite,
    ) {
        self.fs
            .borrow_mut()
            .write(_req, ino, fh, offset, data, write_flags, flags, lock_owner, reply)
    }

    fn flush(&self, _req: &Request<'_>, ino: u64, fh: u64, lock_owner: u64, reply: ReplyEmpty) {
        self.fs.borrow_mut().flush(_req, ino, fh, lock_owner, reply)
    }

    fn release(
        &self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _flags: i32,
        _lock_owner: Option<u64>,
        _flush: bool,
        reply: ReplyEmpty,
    ) {
        self.fs
            .borrow_mut()
            .release(_req, _ino, _fh, _flags, _lock_owner, _flush, reply)
    }

    fn fsync(&self, _req: &Request<'_>, ino: u64, fh: u64, datasync: bool, reply: ReplyEmpty) {
        self.fs.borrow_mut().fsync(_req, ino, fh, datasync, reply)
    }

    fn opendir(&self, _req: &Request<'_>, _ino: u64, _flags: i32, reply: ReplyOpen) {
        self.fs.borrow_mut().opendir(_req, _ino, _flags, reply)
    }

    fn readdir(
        &self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        reply: ReplyDirectory,
    ) {
        self.fs.borrow_mut().readdir(_req, ino, fh, offset, reply)
    }

    fn readdirplus(
        &self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        reply: ReplyDirectoryPlus,
    ) {
        self.fs.borrow_mut().readdirplus(_req, ino, fh, offset, reply)
    }

    fn releasedir(
        &self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _flags: i32,
        reply: ReplyEmpty,
    ) {
        self.fs.borrow_mut().releasedir(_req, _ino, _fh, _flags, reply)
    }

    fn fsyncdir(
        &self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        datasync: bool,
        reply: ReplyEmpty,
    ) {
        self.fs.borrow_mut().fsyncdir(_req, ino, fh, datasync, reply)
    }

    fn statfs(&self, _req: &Request<'_>, _ino: u64, reply: ReplyStatfs) {
        self.fs.borrow_mut().statfs(_req, _ino, reply)
    }

    fn setxattr(
        &self,
        _req: &Request<'_>,
        ino: u64,
        name: &OsStr,
        _value: &[u8],
        flags: i32,
        position: u32,
        reply: ReplyEmpty,
    ) {
        self.fs
            .borrow_mut()
            .setxattr(_req, ino, name, _value, flags, position, reply)
    }

    fn getxattr(
        &self,
        _req: &Request<'_>,
        ino: u64,
        name: &OsStr,
        size: u32,
        reply: ReplyXattr,
    ) {
        self.fs.borrow_mut().getxattr(_req, ino, name, size, reply)
    }

    fn listxattr(&self, _req: &Request<'_>, ino: u64, size: u32, reply: ReplyXattr) {
        self.fs.borrow_mut().listxattr(_req, ino, size, reply)
    }

    fn removexattr(&self, _req: &Request<'_>, ino: u64, name: &OsStr, reply: ReplyEmpty) {
        self.fs.borrow_mut().removexattr(_req, ino, name, reply)
    }

    fn access(&self, _req: &Request<'_>, ino: u64, mask: i32, reply: ReplyEmpty) {
        self.fs.borrow_mut().access(_req, ino, mask, reply)
    }

    fn create(
        &self,
        _req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        mode: u32,
        umask: u32,
        flags: i32,
        reply: ReplyCreate,
    ) {
        self.fs
            .borrow_mut()
            .create(_req, parent, name, mode, umask, flags, reply)
    }

    fn getlk(
        &self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        lock_owner: u64,
        start: u64,
        end: u64,
        typ: i32,
        pid: u32,
        reply: ReplyLock,
    ) {
        self.fs
            .borrow_mut()
            .getlk(_req, ino, fh, lock_owner, start, end, typ, pid, reply)
    }

    fn setlk(
        &self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        lock_owner: u64,
        start: u64,
        end: u64,
        typ: i32,
        pid: u32,
        sleep: bool,
        reply: ReplyEmpty,
    ) {
        self.fs
            .borrow_mut()
            .setlk(_req, ino, fh, lock_owner, start, end, typ, pid, sleep, reply)
    }

    fn bmap(&self, _req: &Request<'_>, ino: u64, blocksize: u32, idx: u64, reply: ReplyBmap) {
        self.fs.borrow_mut().bmap(_req, ino, blocksize, idx, reply)
    }

    fn ioctl(
        &self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        flags: u32,
        cmd: u32,
        in_data: &[u8],
        out_size: u32,
        reply: ReplyIoctl,
    ) {
        self.fs
            .borrow_mut()
            .ioctl(_req, ino, fh, flags, cmd, in_data, out_size, reply)
    }

    #[cfg(feature = "abi-7-11")]
    fn poll(
        &self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        kh: u64,
        events: u32,
        flags: u32,
        reply: ReplyPoll,
    ) {
        self.fs
            .borrow_mut()
            .poll(_req, ino, fh, kh, events, flags, reply)
    }

    fn fallocate(
        &self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        length: i64,
        mode: i32,
        reply: ReplyEmpty,
    ) {
        self.fs
            .borrow_mut()
            .fallocate(_req, ino, fh, offset, length, mode, reply)
    }

    fn lseek(
        &self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        whence: i32,
        reply: ReplyLseek,
    ) {
        self.fs
            .borrow_mut()
            .lseek(_req, ino, fh, offset, whence, reply)
    }

    fn copy_file_range(
        &self,
        _req: &Request<'_>,
        ino_in: u64,
        fh_in: u64,
        offset_in: i64,
        ino_out: u64,
        fh_out: u64,
        offset_out: i64,
        len: u64,
        flags: u32,
        reply: ReplyWrite,
    ) {
        self.fs
            .borrow_mut()
            .copy_file_range(_req, ino_in, fh_in, offset_in, ino_out, fh_out, offset_out, len, flags, reply)
    }

    #[cfg(target_os = "macos")]
    fn setvolname(&self, _req: &Request<'_>, name: &OsStr, reply: ReplyEmpty) {
        self.fs.borrow_mut().setvolname(_req, name, reply)
    }

    #[cfg(target_os = "macos")]
    fn exchange(
        &self,
        _req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        newparent: u64,
        newname: &OsStr,
        options: u64,
        reply: ReplyEmpty,
    ) {
        self.fs
            .borrow_mut()
            .exchange(_req, parent, name, newparent, newname, options, reply)
    }

    #[cfg(target_os = "macos")]
    fn getxtimes(&self, _req: &Request<'_>, ino: u64, reply: ReplyXTimes) {
        self.fs.borrow_mut().getxtimes(_req, ino, reply)
    }
}