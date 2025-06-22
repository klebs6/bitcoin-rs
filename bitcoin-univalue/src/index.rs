// ---------------- [ File: bitcoin-univalue/src/index.rs ]
crate::ix!();

impl Index<&str> for UniValue {
    type Output = UniValue;

    #[inline]
    fn index(&self, key: &str) -> &Self::Output {
        if *self.typ() != uni_value::VType::VOBJ {
            return &NULL_UNI_VALUE;
        }

        for (idx, k) in self.keys().iter().enumerate() {
            if k == key {
                return &self.values()[idx];
            }
        }
        &NULL_UNI_VALUE
    }
}

impl Index<usize> for UniValue {
    type Output = UniValue;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match *self.typ() {
            uni_value::VType::VOBJ | uni_value::VType::VARR => {
                self.values().get(index).unwrap_or(&NULL_UNI_VALUE)
            }
            _ => &NULL_UNI_VALUE,
        }
    }
}

#[cfg(test)]
mod index_spec {
    use super::*;

    #[traced_test]
    fn str_indexer() {
        let mut obj = UniValue::new(uni_value::VType::VOBJ, None);
        obj.keys_mut().push("answer".into());
        obj.values_mut().push(42u64.into());

        assert_eq!(obj["answer"].get_int64(), 42);
        assert!(obj["missing"].is_null());
    }

    #[traced_test]
    fn usize_indexer() {
        // array
        let mut arr = UniValue::new(uni_value::VType::VARR, None);
        arr.values_mut().extend([1u64.into(), 2u64.into(), 3u64.into()]);
        assert_eq!(arr[1].get_int64(), 2);
        assert!(arr[99].is_null());

        // object (values sequence directly addressable)
        let mut obj = UniValue::new(uni_value::VType::VOBJ, None);
        obj.keys_mut().extend(["a".into(), "b".into()]);
        obj.values_mut().extend([10u64.into(), 20u64.into()]);
        assert_eq!(obj[0].get_int64(), 10);
    }
}
