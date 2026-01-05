// ---------------- [ File: bitcoinleveldb-dbinterface/src/get_property.rs ]
crate::ix!();

pub trait DBGetProperty {

    /// DB implementations can export properties about their state via this method.  
    ///
    /// If "property" is a valid property understood by this DB implementation, fills "*value" with its current value and returns true.
    ///
    /// Otherwise returns false.
    /// 
    /// Valid property names include:
    /// 
    ///  "leveldb.num-files-at-level<N>" - return the number of files at level <N>, where <N> is an ASCII representation of a level number (e.g. "0").
    /// 
    ///  "leveldb.stats" - returns a multi-line string that describes statistics about the internal operation of the DB.
    /// 
    ///  "leveldb.sstables" - returns a multi-line string that describes all of the sstables that make up the db contents.
    /// 
    ///  "leveldb.approximate-memory-usage" - returns the approximate number of bytes of memory in use by the DB.
    ///
    fn get_property(&mut self, 
        property: &str,
        value:    *mut String) -> bool;
}

#[cfg(test)]
mod get_property_contract_suite {
    use super::*;
    use tracing::{info, trace};

    struct FixedPropertyProvider;

    impl DBGetProperty for FixedPropertyProvider {
        fn get_property(&mut self, property: &str, value: *mut String) -> bool {
            assert!(!value.is_null(), "value must not be null");

            trace!(property, "get_property called");

            if property == "leveldb.approximate-memory-usage" {
                unsafe {
                    *value = Slice::from("42").to_string();
                }
                return true;
            }

            return false;
        }
    }

    #[traced_test]
    fn get_property_unknown_property_returns_false_and_leaves_value_unchanged() {
        let mut db = FixedPropertyProvider;

        let unchanged = Slice::from("unchanged").to_string();
        let mut out = unchanged.clone();

        trace!("calling get_property with an unknown property");
        let ok = db.get_property("leveldb.unknown-property", &mut out as *mut String);

        assert!(!ok);
        assert_eq!(out, unchanged);

        info!("verified unknown properties return false and preserve *value");
    }

    #[traced_test]
    fn get_property_known_property_returns_true_and_sets_value() {
        let mut db = FixedPropertyProvider;

        let mut out = Slice::from("old").to_string();

        trace!("calling get_property with known property");
        let ok = db.get_property("leveldb.approximate-memory-usage", &mut out as *mut String);

        assert!(ok);
        assert_eq!(out, Slice::from("42").to_string());

        info!("verified known properties return true and overwrite *value");
    }

    #[traced_test]
    fn get_property_empty_property_string_returns_false_and_preserves_value() {
        let mut db = FixedPropertyProvider;

        let unchanged = Slice::from("still").to_string();
        let mut out = unchanged.clone();

        trace!("calling get_property with empty property name");
        let ok = db.get_property("", &mut out as *mut String);

        assert!(!ok);
        assert_eq!(out, unchanged);

        info!("verified empty property name is treated as unknown and preserves *value");
    }
}
