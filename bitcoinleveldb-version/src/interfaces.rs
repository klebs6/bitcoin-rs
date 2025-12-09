// ---------------- [ File: bitcoinleveldb-version/src/interfaces.rs ]
crate::ix!();

pub trait VersionSetVersionInterface
: CurrentVersion
+ AppendVersion
+ ApproximateOffsetOf
+ FinalizeVersionSet
{}

pub trait CurrentVersion {

    /**
      | Return the current version.
      |
      */
    fn current(&self) -> *mut Version;
}

pub trait AppendVersion {
    
    fn append_version(&mut self, v: *mut Version);
}

pub trait ApproximateOffsetOf {
    
    /**
      | Return the approximate offset in the
      | database of the data for "key" as of version
      | "v".
      |
      */
    fn approximate_offset_of(
        &mut self, 
        v:    *mut Version,
        ikey_: &InternalKey
    ) -> u64;
}

pub trait FinalizeVersionSet {
    
    fn finalize(&mut self, v: *mut Version);
}
