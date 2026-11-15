use bitcoinleveldb_db::{LevelDB, LevelDBSnapshot};
use bitcoinleveldb_dbinterface::*;
use bitcoinleveldb_iterator::LevelDBIterator;
use bitcoinleveldb_iteratorinner::*;
use bitcoinleveldb_modeldb::ModelDB;
use bitcoinleveldb_options::{Options, ReadOptions, WriteOptions};
use bitcoinleveldb_slice::Slice;
use bitcoinleveldb_snapshot::{Snapshot, SnapshotImpl, SnapshotDispatchConcreteImplementationKind};
use bitcoinleveldbt_snapshot::dbtest_read_options_from_snapshot_ref;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;

fn model_rep() -> Rc<RefCell<Box<dyn DB>>> {
    Rc::new(RefCell::new(Box::new(ModelDB::new(&Options::default())) as Box<dyn DB>))
}

#[test]
fn c_style_wrapper_and_dbtest_bridge_preserve_model_binary_snapshot_payload() {
    let rep = model_rep();
    let key = [0xff, 0, 0x80]; let old: Vec<u8> = (0u8..=255).collect();
    let wo = WriteOptions::default();
    assert!(rep.borrow_mut().put(&wo, &Slice::from(&key[..]), &Slice::from(old.as_slice())).is_ok());
    let snapshot = rep.borrow_mut().get_snapshot();
    let wrapper = LevelDBSnapshot::new(rep.clone(), Some(snapshot));
    assert_eq!(wrapper.snapshot_model_byte_map().unwrap().get(key.as_slice()), Some(&old));
    assert_eq!(wrapper.snapshot_sequence_number_for_read_reconstruction(), None);
    let ro = dbtest_read_options_from_snapshot_ref(&wrapper);
    let cloned = wrapper.snapshot_read_arc_clone().expect("binary clone");
    assert!(rep.borrow_mut().delete(&wo, &Slice::from(&key[..])).is_ok());
    drop(wrapper);
    let mut output = vec![1];
    assert!(rep.borrow_mut().get_bytes(&ro, &Slice::from(&key[..]), &mut output).is_ok());
    assert_eq!(output, old);
    let mut iter: Box<LevelDBIterator> = unsafe { Box::from_raw(rep.borrow_mut().new_iterator(&ro)) };
    assert_eq!(cloned.snapshot_model_byte_map().unwrap().get(key.as_slice()), Some(&old));
    drop(ro); drop(cloned); drop(rep);
    iter.seek_to_first(); assert!(iter.valid());
    assert_eq!(iter.key().as_bytes(), key); assert_eq!(iter.value().as_bytes(), old.as_slice());
    iter.next(); assert!(!iter.valid()); assert!(iter.status().is_ok());
}

#[test]
fn wrapper_reconstructs_padded_sequence_snapshot_metadata_and_releases_once() {
    struct PaddedSequence {
        padding: [u64; 8],
        sequence: u64,
        drops: Rc<Cell<usize>>,
    }
    impl Snapshot for PaddedSequence {
        fn snapshot_runtime_implementation_kind(&self) -> SnapshotDispatchConcreteImplementationKind {
            SnapshotDispatchConcreteImplementationKind::SnapshotImpl
        }
        fn snapshot_sequence_number_for_read_reconstruction(&self) -> Option<u64> {
            assert_eq!(self.padding, [17; 8]); Some(self.sequence)
        }
        fn snapshot_read_arc_clone(&self) -> Option<Arc<dyn Snapshot>> {
            Some(Arc::new(SnapshotImpl::new(self.sequence)))
        }
    }
    impl Drop for PaddedSequence {
        fn drop(&mut self) { self.drops.set(self.drops.get() + 1); }
    }
    let drops = Rc::new(Cell::new(0));
    let snapshot: Box<dyn Snapshot> = Box::new(PaddedSequence {
        padding: [17; 8], sequence: 723, drops: drops.clone(),
    });
    let wrapper = LevelDBSnapshot::new(model_rep(), Some(snapshot));
    assert_eq!(*wrapper.shadow().sequence_number(), 723);
    assert_eq!(wrapper.snapshot_sequence_number_for_read_reconstruction(), Some(723));
    assert!(wrapper.snapshot_model_byte_map().is_none());
    let cloned = wrapper.snapshot_read_arc_clone().unwrap();
    drop(wrapper); assert_eq!(drops.get(), 1);
    assert_eq!(cloned.snapshot_sequence_number_for_read_reconstruction(), Some(723));
    drop(cloned); assert_eq!(drops.get(), 1);
}

#[test]
fn wrapper_unsupported_snapshot_never_exposes_placeholder_as_read_capability() {
    struct Unsupported; impl Snapshot for Unsupported {}
    let wrapper = LevelDBSnapshot::new(model_rep(), Some(Box::new(Unsupported)));
    assert_eq!(*wrapper.shadow().sequence_number(), 0);
    assert_eq!(wrapper.snapshot_sequence_number_for_read_reconstruction(), None);
    assert!(wrapper.snapshot_model_byte_map().is_none());
    assert!(wrapper.snapshot_read_arc_clone().is_none());
    let empty = LevelDBSnapshot::new(LevelDB::new(Box::new(ModelDB::new(&Options::default()))).rep().clone(), None);
    assert_eq!(empty.snapshot_sequence_number_for_read_reconstruction(), Some(0));
    assert!(empty.snapshot_model_byte_map().is_none());
}
