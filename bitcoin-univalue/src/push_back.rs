// ---------------- [ File: bitcoin-univalue/src/push_back.rs ]
crate::ix!();

impl UniValue {

    /// Append a single element to a JSON array.
    /// Accepts any scalar with explicit matching. Returns false when called on non-array.
    #[instrument(level = "trace", skip(self, val))]
    pub fn push_back<T: Any>(&mut self, val: &T) -> bool {
        if *self.typ() != uni_value::VType::VARR {
            trace!("receiver is not an array – push_back rejected");
            return false;
        }

        if let Some(uv) = (val as &dyn Any).downcast_ref::<UniValue>() {
            self.values_mut().push(uv.clone());
            return true;
        }

        if let Some(v) = (val as &dyn Any).downcast_ref::<bool>() {
            self.values_mut().push(UniValue::from(*v));
            return true;
        }
        if let Some(v) = (val as &dyn Any).downcast_ref::<i32>() {
            self.values_mut().push(UniValue::from(*v));
            return true;
        }
        if let Some(v) = (val as &dyn Any).downcast_ref::<i64>() {
            self.values_mut().push(UniValue::from(*v));
            return true;
        }
        if let Some(v) = (val as &dyn Any).downcast_ref::<u64>() {
            self.values_mut().push(UniValue::from(*v));
            return true;
        }
        if let Some(v) = (val as &dyn Any).downcast_ref::<f64>() {
            self.values_mut().push(UniValue::from(*v));
            return true;
        }
        if let Some(v) = (val as &dyn Any).downcast_ref::<&str>() {
            self.values_mut().push(UniValue::from(*v));
            return true;
        }
        if let Some(v) = (val as &dyn Any).downcast_ref::<String>() {
            self.values_mut().push(UniValue::from(v.clone()));
            return true;
        }

        trace!("unsupported element type for push_back");
        false
    }

    /// Bulk append: extend the JSON array with *vec*’s elements.
    #[instrument(level = "trace", skip(self, vec))]
    pub fn push_backv(&mut self, vec: &Vec<UniValue>) -> bool {
        if *self.typ() != uni_value::VType::VARR {
            trace!("receiver is not an array – push_backv rejected");
            return false;
        }
        self.values_mut().extend(vec.clone());
        true
    }
}

#[cfg(test)]
mod push_back_corrected_spec {
    use super::*;

    fn empty_array() -> UniValue {
        UniValue::new(uni_value::VType::VARR, None)
    }

    #[traced_test]
    fn accepts_all_scalar_variants() {
        let mut arr = empty_array();

        assert!(arr.push_back(&true));
        assert!(arr.push_back(&123i64));
        assert!(arr.push_back(&123u64));
        assert!(arr.push_back(&3.14f64));
        assert!(arr.push_back(&"hello"));
        assert!(arr.push_back(&String::from("owned")));
        assert!(arr.push_back(&UniValue::from("hi")));

        assert_eq!(arr.size(), 7);
        assert_eq!(arr[0].get_bool(), true);
        assert_eq!(arr[4].get_str(), "hello");
        assert_eq!(arr[5].get_str(), "owned");
    }

    #[traced_test]
    fn rejects_when_not_array() {
        let mut obj = UniValue::new(uni_value::VType::VOBJ, None);
        assert!(!obj.push_back(&42u64));
        assert!(obj.is_object());
        assert_eq!(obj.size(), 0);
    }

    #[traced_test]
    fn push_back_vector_bulk_insertion() {
        let mut arr = empty_array();
        let vec = vec![UniValue::from(1u64), UniValue::from(2u64), UniValue::from(3u64)];
        assert!(arr.push_backv(&vec));
        assert_eq!(arr.size(), 3);
        assert_eq!(arr[1].get_int64(), 2);
    }
}
