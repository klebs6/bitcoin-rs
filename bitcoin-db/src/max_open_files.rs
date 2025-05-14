// ---------------- [ File: bitcoin-db/src/max_open_files.rs ]
crate::ix!();

pub fn set_max_open_files(options: *mut leveldb::Options)  {
    
    todo!();
        /*
            // On most platforms the default setting of max_open_files (which is 1000)
        // is optimal. On Windows using a large file count is OK because the handles
        // do not interfere with select() loops. On 64-bit Unix hosts this value is
        // also OK, because up to that amount LevelDB will use an mmap
        // implementation that does not use extra file descriptors (the fds are
        // closed after being mmap'ed).
        //
        // Increasing the value beyond the default is dangerous because LevelDB will
        // fall back to a non-mmap implementation when the file count is too large.
        // On 32-bit Unix host we should decrease the value because the handles use
        // up real fds, and we want to avoid fd exhaustion issues.
        //
        // See PR #12495 for further discussion.

        int default_open_files = options->max_open_files;
    #ifndef WIN32
        if (sizeof(c_void*) < 8) {
            options->max_open_files = 64;
        }
    #endif
        LogPrint(BCLog::LEVELDB, "LevelDB using max_open_files=%d (default=%d)\n",
                 options->max_open_files, default_open_files);
        */
}
