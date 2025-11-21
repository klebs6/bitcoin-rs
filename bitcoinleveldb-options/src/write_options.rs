// ---------------- [ File: bitcoinleveldb-options/src/write_options.rs ]
crate::ix!();

/**
  | Options that control write operations
  |
  */
pub struct WriteOptions {

    /**
      | If true, the write will be flushed from the
      | operating system buffer cache (by calling
      | WritableFile::Sync()) before the write is
      | considered complete.  If this flag is true,
      | writes will be slower.
      |
      | If this flag is false, and the machine
      | crashes, some recent writes may be lost.
      | Note that if it is just the process that
      | crashes (i.e., the machine does not reboot),
      | no writes will be lost even if sync==false.
      |
      | In other words, a DB write with sync==false
      | has similar crash semantics as the "write()"
      | system call.  A DB write with sync==true has
      | similar crash semantics to a "write()" system
      | call followed by "fsync()".
      */
    sync: bool,
}

impl Default for WriteOptions {

    fn default() -> Self {
        trace!("WriteOptions::default: initializing default write options");

        WriteOptions {
            sync: false,
        }
    }
}
