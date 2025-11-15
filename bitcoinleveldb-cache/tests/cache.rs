// ---------------- [ File: bitcoinleveldb-cache/tests/cache.rs ]
use bitcoinleveldb_cache::*;
use bitcoin_imports::*;

#[traced_test]
fn cache_test_hit_and_miss() {
    let mut fixture = CacheTest::new(cache_test::CACHE_SIZE);
    cache_test::set_current(&mut fixture as *mut CacheTest);

    assert_eq!(-1, fixture.lookup(100));

    fixture.insert(100, 101, None);
    assert_eq!(101, fixture.lookup(100));
    assert_eq!(-1, fixture.lookup(200));
    assert_eq!(-1, fixture.lookup(300));

    fixture.insert(200, 201, None);
    assert_eq!(101, fixture.lookup(100));
    assert_eq!(201, fixture.lookup(200));
    assert_eq!(-1, fixture.lookup(300));

    fixture.insert(100, 102, None);
    assert_eq!(102, fixture.lookup(100));
    assert_eq!(201, fixture.lookup(200));
    assert_eq!(-1, fixture.lookup(300));

    assert_eq!(1, fixture.deleted_keys.len());
    assert_eq!(100, fixture.deleted_keys[0]);
    assert_eq!(101, fixture.deleted_values[0]);
}

#[traced_test]
fn cache_test_erase() {
    let mut fixture = CacheTest::new(cache_test::CACHE_SIZE);
    cache_test::set_current(&mut fixture as *mut CacheTest);

    fixture.erase(200);
    assert_eq!(0, fixture.deleted_keys.len());

    fixture.insert(100, 101, None);
    fixture.insert(200, 201, None);

    fixture.erase(100);
    assert_eq!(-1, fixture.lookup(100));
    assert_eq!(201, fixture.lookup(200));

    assert_eq!(1, fixture.deleted_keys.len());
    assert_eq!(100, fixture.deleted_keys[0]);
    assert_eq!(101, fixture.deleted_values[0]);

    fixture.erase(100);
    assert_eq!(-1, fixture.lookup(100));
    assert_eq!(201, fixture.lookup(200));
    assert_eq!(1, fixture.deleted_keys.len());
}

#[traced_test]
fn cache_test_entries_are_pinned() {
    let mut fixture = CacheTest::new(cache_test::CACHE_SIZE);
    cache_test::set_current(&mut fixture as *mut CacheTest);

    fixture.insert(100, 101, None);

    let h1 = unsafe {
        let encoded = encode_key(100);
        let key_slice: Slice = (&encoded[..]).into();
        let cache = &mut *fixture.cache;
        cache.lookup(&key_slice)
    };
    assert!(!h1.is_null());
    let v1 = unsafe {
        let cache = &mut *fixture.cache;
        decode_value(cache.value(h1))
    };
    assert_eq!(101, v1);

    fixture.insert(100, 102, None);

    let h2 = unsafe {
        let encoded = encode_key(100);
        let key_slice: Slice = (&encoded[..]).into();
        let cache = &mut *fixture.cache;
        cache.lookup(&key_slice)
    };
    assert!(!h2.is_null());
    let v2 = unsafe {
        let cache = &mut *fixture.cache;
        decode_value(cache.value(h2))
    };
    assert_eq!(102, v2);

    assert_eq!(0, fixture.deleted_keys.len());

    unsafe {
        let cache = &mut *fixture.cache;
        cache.release(h1);
    }
    assert_eq!(1, fixture.deleted_keys.len());
    assert_eq!(100, fixture.deleted_keys[0]);
    assert_eq!(101, fixture.deleted_values[0]);

    fixture.erase(100);
    assert_eq!(-1, fixture.lookup(100));
    assert_eq!(1, fixture.deleted_keys.len());

    unsafe {
        let cache = &mut *fixture.cache;
        cache.release(h2);
    }
    assert_eq!(2, fixture.deleted_keys.len());
    assert_eq!(100, fixture.deleted_keys[1]);
    assert_eq!(102, fixture.deleted_values[1]);
}

#[traced_test]
fn cache_test_eviction_policy() {
    let mut fixture = CacheTest::new(cache_test::CACHE_SIZE);
    cache_test::set_current(&mut fixture as *mut CacheTest);

    fixture.insert(100, 101, None);
    fixture.insert(200, 201, None);
    fixture.insert(300, 301, None);

    let h = unsafe {
        let encoded = encode_key(300);
        let key_slice: Slice = (&encoded[..]).into();
        let cache = &mut *fixture.cache;
        cache.lookup(&key_slice)
    };
    assert!(!h.is_null());

    for i in 0..(cache_test::CACHE_SIZE as i32 + 100) {
        let key = 1000 + i;
        let value = 2000 + i;
        fixture.insert(key, value, None);
        assert_eq!(value, fixture.lookup(key));
        assert_eq!(101, fixture.lookup(100));
    }

    assert_eq!(101, fixture.lookup(100));
    assert_eq!(-1, fixture.lookup(200));
    assert_eq!(301, fixture.lookup(300));

    unsafe {
        let cache = &mut *fixture.cache;
        cache.release(h);
    }
}

#[traced_test]
fn cache_test_use_exceeds_size() {
    let mut fixture = CacheTest::new(cache_test::CACHE_SIZE);
    cache_test::set_current(&mut fixture as *mut CacheTest);

    let mut handles: Vec<*mut CacheHandle> = Vec::new();
    for i in 0..(cache_test::CACHE_SIZE as i32 + 100) {
        let key = 1000 + i;
        let value = 2000 + i;
        let h = fixture.insert_and_return_handle(key, value, None);
        handles.push(h);
    }

    for i in 0..handles.len() as i32 {
        let key = 1000 + i;
        let expected = 2000 + i;
        assert_eq!(expected, fixture.lookup(key));
    }

    unsafe {
        let cache = &mut *fixture.cache;
        for h in handles {
            cache.release(h);
        }
    }
}

#[traced_test]
fn cache_test_heavy_entries() {
    let mut fixture = CacheTest::new(cache_test::CACHE_SIZE);
    cache_test::set_current(&mut fixture as *mut CacheTest);

    const LIGHT: i32 = 1;
    const HEAVY: i32 = 10;
    let mut added = 0;
    let mut index = 0;

    while added < 2 * cache_test::CACHE_SIZE as i32 {
        let weight = if (index & 1) != 0 { LIGHT } else { HEAVY };
        fixture.insert(index, 1000 + index, Some(weight));
        added += weight;
        index += 1;
    }

    let mut cached_weight = 0;
    for i in 0..index {
        let weight = if (i & 1) != 0 { LIGHT } else { HEAVY };
        let r = fixture.lookup(i);
        if r >= 0 {
            cached_weight += weight;
            assert_eq!(1000 + i, r);
        }
    }

    let limit = cache_test::CACHE_SIZE as i32 + cache_test::CACHE_SIZE as i32 / 10;
    assert!(
        cached_weight <= limit,
        "cached_weight={} limit={}",
        cached_weight,
        limit
    );
}

#[traced_test]
fn cache_test_new_id() {
    let mut fixture = CacheTest::new(cache_test::CACHE_SIZE);
    cache_test::set_current(&mut fixture as *mut CacheTest);

    let a = unsafe {
        let cache = &mut *fixture.cache;
        cache.new_id()
    };
    let b = unsafe {
        let cache = &mut *fixture.cache;
        cache.new_id()
    };
    assert_ne!(a, b);
}

#[traced_test]
fn cache_test_prune() {
    let mut fixture = CacheTest::new(cache_test::CACHE_SIZE);
    cache_test::set_current(&mut fixture as *mut CacheTest);

    fixture.insert(1, 100, None);
    fixture.insert(2, 200, None);

    let handle = unsafe {
        let encoded = encode_key(1);
        let key_slice: Slice = (&encoded[..]).into();
        let cache = &mut *fixture.cache;
        cache.lookup(&key_slice)
    };
    assert!(!handle.is_null());

    unsafe {
        let cache = &mut *fixture.cache;
        cache.prune();
        cache.release(handle);
    }

    assert_eq!(100, fixture.lookup(1));
    assert_eq!(-1, fixture.lookup(2));
}

#[traced_test]
fn cache_test_zero_size() {
    let mut fixture = CacheTest::new(cache_test::CACHE_SIZE);
    cache_test::set_current(&mut fixture as *mut CacheTest);

    fixture.reset_cache(0);

    fixture.insert(1, 100, None);
    assert_eq!(-1, fixture.lookup(1));
}
