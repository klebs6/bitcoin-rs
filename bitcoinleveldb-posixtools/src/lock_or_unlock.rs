// ---------------- [ File: bitcoinleveldb-posixtools/src/lock_or_unlock.rs ]
crate::ix!();

pub fn lock_or_unlock(fd: i32, lock: bool) -> i32 {

    trace!(
        fd,
        lock,
        "lock_or_unlock: preparing POSIX file lock operation"
    );

    unsafe {
        // SAFETY: We zero-initialize the flock structure before use, then
        // populate the fields required by fcntl(F_SETLK, ...).
        let mut file_lock_info: libc::flock = std::mem::zeroed();

        file_lock_info.l_type = if lock {
            libc::F_WRLCK as libc::c_short
        } else {
            libc::F_UNLCK as libc::c_short
        };
        file_lock_info.l_whence = libc::SEEK_SET as libc::c_short;
        file_lock_info.l_start = 0;
        file_lock_info.l_len = 0; // Lock/unlock entire file.

        let result = libc::fcntl(fd, libc::F_SETLK, &file_lock_info);

        if result == 0 {
            debug!(
                fd,
                lock,
                "lock_or_unlock: fcntl(F_SETLK) succeeded"
            );
        } else {
            warn!(
                fd,
                lock,
                result,
                "lock_or_unlock: fcntl(F_SETLK) failed"
            );
        }

        result
    }
}
