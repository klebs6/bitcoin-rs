// ---------------- [ File: bitcoinleveldb-env/src/get_test_directory.rs ]
crate::ix!();

pub trait GetTestDirectory {

    /**
      | *path is set to a temporary directory that
      | can be used for testing. It may or may not
      | have just been created. The directory may or
      | may not differ between runs of the same
      | process, but subsequent calls will return the
      | same directory.
      */
    fn get_test_directory(&mut self, path: *mut String) -> crate::Status;
}
