// ---------------- [ File: bitcoinleveldb-lru/src/mock.rs ]
crate::ix!();

pub fn mock_test_deleter(_: &Slice, _: *mut c_void) -> c_void {
    unsafe { core::mem::zeroed() }
}

pub unsafe fn mock_make_heap_sentinel_node() -> *mut LRUHandle {
    let node = libc::malloc(core::mem::size_of::<LRUHandle>()) as *mut LRUHandle;
    assert!(!node.is_null(), "mock_make_heap_sentinel_node: alloc failed");

    (*node).set_value_ptr(core::ptr::null_mut());
    (*node).set_deleter_fn(mock_test_deleter);
    (*node).set_charge_value(0);
    (*node).set_key_length(0);
    (*node).set_hash_value(0);
    (*node).set_in_cache(false);
    (*node).set_refs(0);
    (*node).set_next_hash_ptr(core::ptr::null_mut());
    (*node).set_next_ptr(node);
    (*node).set_prev_ptr(node);

    node
}

pub unsafe fn mock_make_list_node() -> *mut LRUHandle {
    let node = libc::malloc(core::mem::size_of::<LRUHandle>()) as *mut LRUHandle;
    assert!(!node.is_null(), "mock_make_list_node: allocation failed");

    (*node).set_value_ptr(core::ptr::null_mut());
    (*node).set_deleter_fn(mock_test_deleter);
    (*node).set_charge_value(0);
    (*node).set_key_length(0);
    (*node).set_hash_value(0);
    (*node).set_in_cache(false);
    (*node).set_refs(1);
    (*node).set_next_hash_ptr(core::ptr::null_mut());
    (*node).set_next_ptr(core::ptr::null_mut());
    (*node).set_prev_ptr(core::ptr::null_mut());

    node
}
