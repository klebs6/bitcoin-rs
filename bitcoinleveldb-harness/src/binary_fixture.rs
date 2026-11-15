// ---------------- [ File: bitcoinleveldb-harness/src/binary_fixture.rs ]
crate::ix!();

/// Structured equality keeps NULs, high bytes and delimiter-like text distinct.
pub(crate) fn harness_assert_byte_row(iter: &LevelDBIterator, expected: Option<(&[u8], &[u8])>) {
    assert_eq!(iter.valid(), expected.is_some());
    if let Some((key, value)) = expected {
        assert_eq!(iter.key().as_bytes(), key);
        assert_eq!(iter.value().as_bytes(), value);
    }
    assert!(iter.status().is_ok());
}

impl Harness {
    fn binary_fixture_iterator(&self) -> Box<LevelDBIterator> {
        let tagged = self.constructor as usize;
        let tag = tagged & BITCOINLEVELDB_HARNESS_CONSTRUCTOR_PTR_TAG_MASK;
        let raw = (tagged & !BITCOINLEVELDB_HARNESS_CONSTRUCTOR_PTR_TAG_MASK) as *mut Constructor;
        assert!(!raw.is_null());
        let ptr = unsafe { match tag {
            0 => (&*(raw as *mut TableConstructor)).new_iterator(),
            1 => (&*(raw as *mut BlockConstructor)).new_iterator(),
            2 => (&*(raw as *mut MemTableConstructor)).new_iterator(),
            3 => (&*(raw as *mut DBConstructor)).new_iterator(),
            _ => unreachable!(),
        }};
        assert!(!ptr.is_null());
        unsafe { Box::from_raw(ptr) }
    }

    /// Opt-in byte fixture path. Existing text test() keeps its random draw law.
    /// Iterators borrow this harness's concrete storage and die before return.
    pub fn test_bytes(&mut self, rnd: *mut Random) {
        let tagged = self.constructor as usize;
        let tag = tagged & BITCOINLEVELDB_HARNESS_CONSTRUCTOR_PTR_TAG_MASK;
        let raw = (tagged & !BITCOINLEVELDB_HARNESS_CONSTRUCTOR_PTR_TAG_MASK) as *mut Constructor;
        assert!(!raw.is_null());
        let mut keys = Vec::<Vec<u8>>::new(); let mut data = ConstructorByteMap::new();
        let status = unsafe { (&mut *raw).finish_bytes(&self.options, &mut keys, &mut data) };
        assert!(status.is_ok(), "byte fixture extraction failed: {}", status.to_string());
        let status = unsafe { match tag {
            0 => (&mut *(raw as *mut TableConstructor)).finish_impl_bytes(&self.options, &data),
            1 => (&mut *(raw as *mut BlockConstructor)).finish_impl_bytes(&self.options, &data),
            2 => (&mut *(raw as *mut MemTableConstructor)).finish_impl_bytes(&self.options, &data),
            3 => (&mut *(raw as *mut DBConstructor)).finish_impl_bytes(&self.options, &data),
            _ => unreachable!(),
        }};
        assert!(status.is_ok(), "byte fixture finalization failed: {}", status.to_string());
        let mut iter = self.binary_fixture_iterator();
        iter.seek_to_first();
        for key in &keys {
            harness_assert_byte_row(&iter, Some((key.as_slice(), data[key].as_slice()))); iter.next();
        }
        harness_assert_byte_row(&iter, None);
        iter.seek_to_last();
        for key in keys.iter().rev() {
            harness_assert_byte_row(&iter, Some((key.as_slice(), data[key].as_slice()))); iter.prev();
        }
        harness_assert_byte_row(&iter, None);
        let mut probes = vec![Vec::new(), vec![0xff, 0xff]];
        for key in &keys { probes.push(key.clone()); let mut after = key.clone(); after.push(0); probes.push(after); }
        if !rnd.is_null() && !keys.is_empty() {
            for _ in 0..32 {
                let index = unsafe { (*rnd).uniform(keys.len() as i32) } as usize;
                probes.push(keys[index].clone());
            }
        }
        for probe in probes {
            let expected = keys.iter().find(|key| self.options.comparator().compare(
                &Slice::from(key.as_slice()), &Slice::from(probe.as_slice())) >= 0);
            iter.seek(&Slice::from(probe.as_slice()));
            harness_assert_byte_row(&iter, expected.map(|key| (key.as_slice(), data[key].as_slice())));
        }
        drop(iter);
        if let Some(db) = self.db() {
            for key in &keys {
                let mut output = Vec::new();
                let status = unsafe { (&mut *db).get_bytes(&ReadOptions::default(), &Slice::from(key.as_slice()), &mut output) };
                assert!(status.is_ok()); assert_eq!(&output, &data[key]);
            }
        }
    }
}
