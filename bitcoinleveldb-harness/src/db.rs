// ---------------- [ File: bitcoinleveldb-harness/src/db.rs ]
crate::ix!();

impl Harness {

    /**
       Returns nullptr if not running against a DB
      */
    pub fn db(&self) -> Option<*mut dyn DB> {
        const TAG_MASK: usize = BITCOINLEVELDB_HARNESS_CONSTRUCTOR_PTR_TAG_MASK;

        let tagged: usize = self.constructor as usize;
        let tag: usize = tagged & TAG_MASK;
        let raw: *mut Constructor = (tagged & !TAG_MASK) as *mut Constructor;

        if raw.is_null() {
            return None;
        }

        unsafe {
            match tag {
                3 => {
                    let ptr: *mut dyn DB = (&*(raw as *mut DBConstructor)).db();
                    if ptr.is_null() {
                        None
                    } else {
                        Some(ptr)
                    }
                }
                _ => None,
            }
        }
    }
}
