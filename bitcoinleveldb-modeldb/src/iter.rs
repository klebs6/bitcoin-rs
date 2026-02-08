// ---------------- [ File: bitcoinleveldb-modeldb/src/iter.rs ]
crate::ix!();

pub struct ModelIter<'a> {
    map: *const KVMap,

    /**
      | Do we own map_
      |
      */
    owned: bool,

    keys: Vec<String>,

    // Position semantics:
    // - 0..keys.len()-1 => valid
    // - keys.len()      => end()   (matches C++ iter_ = map_->end())
    // - -1              => before-first (represents undefined C++ --begin(); kept invalid)
    pos: isize,

    _marker: core::marker::PhantomData<&'a KVMap>,
}

impl<'a> Drop for ModelIter<'a> {
    fn drop(&mut self) {
        tracing::trace!(
            owned = self.owned,
            map_ptr = ?self.map,
            key_count = self.keys.len(),
            pos = self.pos,
            "ModelIter::drop"
        );

        if self.owned {
            // SAFETY: When `owned_` is true, `map_` was produced by `Box::into_raw`
            // (matching the C++ `new KVMap` / `delete map_` pairing).
            unsafe {
                if !self.map.is_null() {
                    drop(Box::from_raw(self.map as *mut KVMap));
                } else {
                    // C++ `delete nullptr;` is a no-op; keep that behavior.
                    tracing::warn!("ModelIter::drop called with owned=true but map is null");
                }
            }
        }
    }
}

impl<'a> ModelIter<'a> {
    
    pub fn new(map: *const KVMap, owned: bool) -> Self {
        let mut keys: Vec<String> = unsafe {
            if map.is_null() {
                Vec::new()
            } else {
                (*map).keys().cloned().collect()
            }
        };

        // HashMap has no ordering; to approximate the C++ std::map iteration order,
        // we iterate in lexicographic key order.
        keys.sort();

        let end_pos: isize = keys.len() as isize;

        tracing::debug!(
            owned = owned,
            map_ptr = ?map,
            key_count = keys.len(),
            pos = end_pos,
            "ModelIter::new"
        );

        Self {
            map,
            owned,
            keys,
            pos: end_pos,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<'a> LevelDBIteratorInterface for ModelIter<'a> {}

impl<'a> LevelDBIteratorValid for ModelIter<'a> {
    fn valid(&self) -> bool {
        let len: isize = self.keys.len() as isize;
        let v = self.pos >= 0 && self.pos < len;

        tracing::trace!(
            valid = v,
            pos = self.pos,
            key_count = self.keys.len(),
            "ModelIter::valid"
        );

        v
    }
}

impl<'a> LevelDBIteratorSeekToFirst for ModelIter<'a> {
    fn seek_to_first(&mut self) {
        let len: isize = self.keys.len() as isize;
        if len == 0 {
            self.pos = len; // end()
        } else {
            self.pos = 0;
        }

        tracing::trace!(
            pos = self.pos,
            key_count = self.keys.len(),
            "ModelIter::seek_to_first"
        );
    }
}

impl<'a> LevelDBIteratorSeekToLast for ModelIter<'a> {
    fn seek_to_last(&mut self) {
        let len_usize: usize = self.keys.len();
        let len: isize = len_usize as isize;

        if len_usize == 0 {
            self.pos = len; // end()
        } else {
            self.pos = len - 1;
        }

        tracing::trace!(
            pos = self.pos,
            key_count = self.keys.len(),
            "ModelIter::seek_to_last"
        );
    }
}

impl<'a> LevelDBIteratorSeek for ModelIter<'a> {
    fn seek(&mut self, target: &Slice) {
        let t: String = target.to_string();
        let idx: usize = match self.keys.binary_search(&t) {
            Ok(i)  => i,
            Err(i) => i,
        };

        let len: isize = self.keys.len() as isize;

        if idx >= self.keys.len() {
            self.pos = len; // end()
        } else {
            self.pos = idx as isize;
        }

        tracing::trace!(
            target_len = target.as_bytes().len(),
            pos = self.pos,
            key_count = self.keys.len(),
            "ModelIter::seek"
        );
    }
}

impl<'a> LevelDBIteratorNext for ModelIter<'a> {
    fn next(&mut self) {
        let len: isize = self.keys.len() as isize;

        if self.pos >= 0 && self.pos < len {
            self.pos += 1;
        }

        tracing::trace!(
            pos = self.pos,
            key_count = self.keys.len(),
            "ModelIter::next"
        );
    }
}

impl<'a> LevelDBIteratorPrev for ModelIter<'a> {
    fn prev(&mut self) {
        let len_usize: usize = self.keys.len();
        let len: isize = len_usize as isize;

        if self.pos == len {
            // C++ std::map: --end() yields last element (if any).
            if len_usize == 0 {
                self.pos = len; // end()
            } else {
                self.pos = len - 1;
            }
        } else if self.pos > 0 {
            self.pos -= 1;
        } else if self.pos == 0 {
            // Represents undefined C++ --begin(); keep invalid state.
            self.pos = -1;
        }

        tracing::trace!(
            pos = self.pos,
            key_count = self.keys.len(),
            "ModelIter::prev"
        );
    }
}

impl<'a> LevelDBIteratorKey for ModelIter<'a> {
    fn key(&self) -> Slice {
        debug_assert!(
            <Self as LevelDBIteratorValid>::valid(self),
            "ModelIter::key called when invalid"
        );

        let idx: usize = self.pos as usize;
        let k: &String = &self.keys[idx];

        tracing::trace!(
            key_len = k.as_bytes().len(),
            pos = self.pos,
            "ModelIter::key"
        );

        Slice::from(k)
    }
}

impl<'a> LevelDBIteratorValue for ModelIter<'a> {
    fn value(&self) -> Slice {
        debug_assert!(
            <Self as LevelDBIteratorValid>::valid(self),
            "ModelIter::value called when invalid"
        );

        let idx: usize = self.pos as usize;
        let k: &String = &self.keys[idx];

        let v: &String = unsafe {
            debug_assert!(!self.map.is_null(), "ModelIter::value map is null");
            (*self.map)
                .get(k)
                .expect("ModelIter::value key missing from map")
        };

        tracing::trace!(
            value_len = v.as_bytes().len(),
            pos = self.pos,
            "ModelIter::value"
        );

        Slice::from(v)
    }
}

impl<'a> LevelDBIteratorStatus for ModelIter<'a> {
    fn status(&self) -> crate::Status {
        tracing::trace!("ModelIter::status");
        crate::Status::ok()
    }
}

#[cfg(test)]
mod model_iter_trait_contract_suite {
    use super::*;

    fn collect_all_kvs_from_model_iter(iter: &mut ModelIter<'static>) -> Vec<(String, String)> {
        tracing::debug!("collect_all_kvs_from_model_iter");

        <ModelIter<'static> as LevelDBIteratorSeekToFirst>::seek_to_first(iter);

        let mut out: Vec<(String, String)> = Vec::new();
        while <ModelIter<'static> as LevelDBIteratorValid>::valid(iter) {
            let k: String = <ModelIter<'static> as LevelDBIteratorKey>::key(iter).to_string();
            let v: String = <ModelIter<'static> as LevelDBIteratorValue>::value(iter).to_string();
            out.push((k, v));
            <ModelIter<'static> as LevelDBIteratorNext>::next(iter);
        }

        let st: crate::Status = <ModelIter<'static> as LevelDBIteratorStatus>::status(iter);
        assert!(st.is_ok(), "status not ok: {}", st.to_string());

        out
    }

    #[traced_test]
    fn model_iter_empty_map_navigation_and_validity() {
        tracing::info!("starting model_iter_empty_map_navigation_and_validity");

        let map: KVMap = KVMap::default();
        let mut it: ModelIter<'static> = ModelIter::<'static>::new(&map as *const KVMap, false);

        assert!(!<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));

        <ModelIter<'static> as LevelDBIteratorSeekToFirst>::seek_to_first(&mut it);
        assert!(!<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));

        <ModelIter<'static> as LevelDBIteratorSeekToLast>::seek_to_last(&mut it);
        assert!(!<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));

        <ModelIter<'static> as LevelDBIteratorSeek>::seek(&mut it, &Slice::from("a"));
        assert!(!<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));

        <ModelIter<'static> as LevelDBIteratorPrev>::prev(&mut it);
        assert!(!<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));

        <ModelIter<'static> as LevelDBIteratorNext>::next(&mut it);
        assert!(!<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));

        let st: crate::Status = <ModelIter<'static> as LevelDBIteratorStatus>::status(&it);
        assert!(st.is_ok(), "status not ok: {}", st.to_string());

        let key_res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = <ModelIter<'static> as LevelDBIteratorKey>::key(&it);
        }));
        assert!(key_res.is_err(), "expected key() to panic when invalid");

        let value_res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = <ModelIter<'static> as LevelDBIteratorValue>::value(&it);
        }));
        assert!(value_res.is_err(), "expected value() to panic when invalid");
    }

    #[traced_test]
    fn model_iter_singleton_map_forward_backward_edges() {
        tracing::info!("starting model_iter_singleton_map_forward_backward_edges");

        let mut map: KVMap = KVMap::default();
        map.insert(String::from("k"), String::from("v"));

        let mut it: ModelIter<'static> = ModelIter::<'static>::new(&map as *const KVMap, false);

        <ModelIter<'static> as LevelDBIteratorSeekToFirst>::seek_to_first(&mut it);
        assert!(<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));

        assert_eq!(<ModelIter<'static> as LevelDBIteratorKey>::key(&it).to_string(), "k".to_string());
        assert_eq!(<ModelIter<'static> as LevelDBIteratorValue>::value(&it).to_string(), "v".to_string());

        <ModelIter<'static> as LevelDBIteratorNext>::next(&mut it);
        assert!(!<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));

        <ModelIter<'static> as LevelDBIteratorPrev>::prev(&mut it);
        assert!(<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));
        assert_eq!(<ModelIter<'static> as LevelDBIteratorKey>::key(&it).to_string(), "k".to_string());

        <ModelIter<'static> as LevelDBIteratorPrev>::prev(&mut it);
        assert!(!<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));

        <ModelIter<'static> as LevelDBIteratorNext>::next(&mut it);
        assert!(!<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));

        <ModelIter<'static> as LevelDBIteratorSeek>::seek(&mut it, &Slice::from("k"));
        assert!(<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));

        <ModelIter<'static> as LevelDBIteratorSeek>::seek(&mut it, &Slice::from("z"));
        assert!(!<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));

        let key_res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = <ModelIter<'static> as LevelDBIteratorKey>::key(&it);
        }));
        assert!(key_res.is_err(), "expected key() to panic when invalid");
    }

    #[traced_test]
    fn model_iter_multiple_keys_sorted_order_and_seek_behavior() {
        tracing::info!("starting model_iter_multiple_keys_sorted_order_and_seek_behavior");

        let mut map: KVMap = KVMap::default();
        map.insert(String::from("b"), String::from("vb"));
        map.insert(String::from("a"), String::from("va"));
        map.insert(String::from("aa"), String::from("vaa"));
        map.insert(String::from("c"), String::from("vc"));

        let mut it: ModelIter<'static> = ModelIter::<'static>::new(&map as *const KVMap, false);

        let kvs: Vec<(String, String)> = collect_all_kvs_from_model_iter(&mut it);
        assert_eq!(
            kvs,
            vec![
                (String::from("a"), String::from("va")),
                (String::from("aa"), String::from("vaa")),
                (String::from("b"), String::from("vb")),
                (String::from("c"), String::from("vc")),
            ]
        );

        <ModelIter<'static> as LevelDBIteratorSeek>::seek(&mut it, &Slice::from("ab"));
        assert!(<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));
        assert_eq!(<ModelIter<'static> as LevelDBIteratorKey>::key(&it).to_string(), "b".to_string());

        <ModelIter<'static> as LevelDBIteratorSeek>::seek(&mut it, &Slice::from("aa"));
        assert!(<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));
        assert_eq!(<ModelIter<'static> as LevelDBIteratorKey>::key(&it).to_string(), "aa".to_string());

        <ModelIter<'static> as LevelDBIteratorSeek>::seek(&mut it, &Slice::from("d"));
        assert!(!<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));

        <ModelIter<'static> as LevelDBIteratorSeekToLast>::seek_to_last(&mut it);
        assert!(<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));
        assert_eq!(<ModelIter<'static> as LevelDBIteratorKey>::key(&it).to_string(), "c".to_string());

        <ModelIter<'static> as LevelDBIteratorPrev>::prev(&mut it);
        assert!(<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));
        assert_eq!(<ModelIter<'static> as LevelDBIteratorKey>::key(&it).to_string(), "b".to_string());

        <ModelIter<'static> as LevelDBIteratorPrev>::prev(&mut it);
        assert!(<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));
        assert_eq!(<ModelIter<'static> as LevelDBIteratorKey>::key(&it).to_string(), "aa".to_string());

        <ModelIter<'static> as LevelDBIteratorPrev>::prev(&mut it);
        assert!(<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));
        assert_eq!(<ModelIter<'static> as LevelDBIteratorKey>::key(&it).to_string(), "a".to_string());

        <ModelIter<'static> as LevelDBIteratorPrev>::prev(&mut it);
        assert!(!<ModelIter<'static> as LevelDBIteratorValid>::valid(&it));
    }

    #[traced_test]
    fn model_iter_owned_null_map_drop_is_safe() {
        tracing::info!("starting model_iter_owned_null_map_drop_is_safe");

        let it: ModelIter<'static> = ModelIter::<'static>::new(core::ptr::null(), true);
        drop(it);
    }
}
