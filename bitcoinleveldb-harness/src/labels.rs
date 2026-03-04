// ---------------- [ File: bitcoinleveldb-harness/src/labels.rs ]
crate::ix!();

/// Bit-mask for storing a small constructor-kind tag in the low bits of an aligned pointer.
///
/// Invariant: all constructor allocations used with this mask must have alignment >= 4,
/// so the low 2 bits are always zero on a valid allocation pointer.
pub const BITCOINLEVELDB_HARNESS_CONSTRUCTOR_PTR_TAG_MASK: usize = 0b11;

/// Stable machine label for `TestType` without requiring `Debug`.
///
/// Invariant: this mapping is exhaustive over `TestType` variants and must remain stable
/// across refactors so tracing topology does not drift.
pub fn bitcoinleveldb_harness_test_type_machine_label(ty: &TestType) -> &'static str {
    match ty {
        TestType::TABLE_TEST => "table_test",
        TestType::BLOCK_TEST => "block_test",
        TestType::MEMTABLE_TEST => "memtable_test",
        TestType::DB_TEST => "db_test",
    }
}

/// Stable machine label for a tagged constructor pointer kind.
///
/// Invariant: tag values 0..=3 correspond to the tagged constructor layout used by `Harness`.
pub fn bitcoinleveldb_harness_constructor_tag_machine_label(tag: usize) -> &'static str {
    match tag {
        0 => "table_constructor",
        1 => "block_constructor",
        2 => "memtable_constructor",
        3 => "db_constructor",
        _ => "invalid_constructor_tag",
    }
}
