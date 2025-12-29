// ---------------- [ File: bitcoinleveldb-batch/src/handler.rs ]
crate::ix!();

pub trait WriteBatchPut {
    fn put(&mut self, 
            key_:   &Slice,
            value: &Slice);
}

pub trait WriteBatchDelete {
    fn delete(&mut self, key_: &Slice);
}

pub trait WriteBatchHandler: 
WriteBatchPut 
+ WriteBatchDelete { }

#[cfg(test)]
mod handler_rs_exhaustive_contract_suite {
    use super::*;
    use crate::write_batch_test_harness_utilities::*;

    fn accept_write_batch_handler_by_trait_bound<T: WriteBatchHandler>(_h: &mut T) {
        trace!("accept_write_batch_handler_by_trait_bound: invoked");
    }

    struct MinimalHandler {
        puts: usize,
        dels: usize,
    }

    impl WriteBatchHandler for MinimalHandler {}

    impl WriteBatchPut for MinimalHandler {
        fn put(&mut self, _key_: &Slice, _value: &Slice) {
            trace!("MinimalHandler::put");
            self.puts += 1;
        }
    }

    impl WriteBatchDelete for MinimalHandler {
        fn delete(&mut self, _key_: &Slice) {
            trace!("MinimalHandler::delete");
            self.dels += 1;
        }
    }

    #[traced_test]
    fn write_batch_handler_trait_composition_is_satisfied_by_types_implementing_put_and_delete() {
        trace!(
            "write_batch_handler_trait_composition_is_satisfied_by_types_implementing_put_and_delete: begin"
        );

        let mut h = MinimalHandler { puts: 0, dels: 0 };
        accept_write_batch_handler_by_trait_bound(&mut h);

        let mut batch = WriteBatch::new();
        batch.put(&Slice::from("a"), &Slice::from("va"));
        batch.delete(&Slice::from("b"));

        let st = batch.iterate(&mut h as *mut dyn WriteBatchHandler);
        assert!(st.is_ok());
        assert_eq!(h.puts, 1);
        assert_eq!(h.dels, 1);

        trace!(
            "write_batch_handler_trait_composition_is_satisfied_by_types_implementing_put_and_delete: end"
        );
    }
}
