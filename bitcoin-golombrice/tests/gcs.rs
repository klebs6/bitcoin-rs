// ---------------- [ File: bitcoin-golombrice/tests/gcs.rs ]
use bitcoin_imports::*;
use bitcoin_golombrice::*;

use std::iter::FromIterator;

fn make_vec(byte: u8) -> GcsFilterElement {
    let mut v = vec![0u8; 32];
    v[0] = byte;
    v
}

#[traced_test]
fn gcsfilter_inclusion_and_matchany() {
    let params = GcsFilterParams::new(Some(0), Some(0), Some(10), Some(1 << 10));

    let mut included = GcsFilterElementSet::default();
    let mut excluded = GcsFilterElementSet::default();

    for i in 0u8..100 {
        included.insert(make_vec(i));
        let mut v = vec![0u8; 32];
        v[1] = i;
        excluded.insert(v);
    }

    let filter = GCSFilter::new_with_element_set(&params, &included);

    for element in &included {
        assert!(filter.match_(element));

        excluded.insert(element.clone());
        assert!(filter.match_any(&excluded));
        excluded.remove(element);
    }
}

#[traced_test]
fn gcsfilter_default_constructor() {
    let filter: GCSFilter = GCSFilter::from(None);

    assert_eq!(filter.getn(), 0);
    assert_eq!(filter.get_encoded().len(), 1);

    let params = filter.get_params();
    assert_eq!(*params.siphash_k0(), 0);
    assert_eq!(*params.siphash_k1(), 0);
    assert_eq!(*params.p(), 0);
    assert_eq!(*params.m(), 1);
}

