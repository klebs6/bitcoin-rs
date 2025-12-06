crate::ix!();

pub trait GetApproximateSizes {

    /**
      | For each i in [0,n-1], store in "sizes[i]",
      | the approximate file system space used by
      | keys in "[range[i].start .. range[i].limit)".
      |
      | Note that the returned sizes measure file
      | system space usage, so if the user data
      | compresses by a factor of ten, the returned
      | sizes will be one-tenth the size of the
      | corresponding user data size.
      |
      | The results may not include the sizes of
      | recently written data.
      */
    fn get_approximate_sizes(&mut self, 
            range: *const Range,
            n:     i32,
            sizes: *mut u64);
}
