// ---------------- [ File: bitcoinleveldb-merger/src/status.rs ]
crate::ix!();

impl LevelDBIteratorStatus for MergingIterator {

    fn status(&self) -> Status {
        trace!(
            "MergingIterator::status: aggregating status from {} children",
            self.children().len()
        );

        let mut status = Status::ok();

        for (idx, child) in self.children().iter().enumerate() {
            let st = child.status();
            trace!(
                "MergingIterator::status: child_index={} status_code={:?}",
                idx,
                st.code()
            );

            if !st.is_ok() {
                status = st;
                break;
            }
        }

        trace!(
            "MergingIterator::status: final status_code={:?}",
            status.code()
        );

        status
    }
}

#[cfg(test)]
mod merging_iterator_status_tests {
    use super::*;

    #[derive(Default)]
    struct ErrorIterator {
        status: Status,
    }

    impl LevelDBIteratorInterface for ErrorIterator {}

    impl LevelDBIteratorValid for ErrorIterator {
        fn valid(&self) -> bool {
            false
        }
    }

    impl LevelDBIteratorSeekToFirst for ErrorIterator {
        fn seek_to_first(&mut self) {}
    }

    impl LevelDBIteratorSeekToLast for ErrorIterator {
        fn seek_to_last(&mut self) {}
    }

    impl LevelDBIteratorSeek for ErrorIterator {
        fn seek(&mut self, _target: &Slice) {}
    }

    impl LevelDBIteratorNext for ErrorIterator {
        fn next(&mut self) {}
    }

    impl LevelDBIteratorPrev for ErrorIterator {
        fn prev(&mut self) {}
    }

    impl LevelDBIteratorStatus for ErrorIterator {
        fn status(&self) -> Status {
            Status::new_from_other_copy(&self.status)
        }
    }

    impl LevelDBIteratorKey for ErrorIterator {
        fn key(&self) -> Slice {
            panic!("ErrorIterator::key should not be called");
        }
    }

    impl LevelDBIteratorValue for ErrorIterator {
        fn value(&self) -> Slice {
            panic!("ErrorIterator::value should not be called");
        }
    }

    fn make_ok_child() -> *mut LevelDBIterator {
        let internal = MockStubIterator::new_with_entries(&[(b"a", b"va")]);
        let internal_box: Box<dyn LevelDBIteratorInterface> = Box::new(internal);
        let wrapper = LevelDBIterator::new(Some(internal_box));
        Box::into_raw(Box::new(wrapper))
    }

    fn make_error_child(status: Status) -> *mut LevelDBIterator {
        let internal = ErrorIterator {
            status: Status::new_from_other_copy(&status),
        };
        let internal_box: Box<dyn LevelDBIteratorInterface> = Box::new(internal);
        let wrapper = LevelDBIterator::new(Some(internal_box));
        Box::into_raw(Box::new(wrapper))
    }

    #[traced_test]
    fn status_reports_ok_when_all_children_ok() {
        trace!("TEST(status): status_reports_ok_when_all_children_ok");

        let c0 = make_ok_child();
        let c1 = make_ok_child();

        let mut children = [c0, c1];

        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let merging =
            MergingIterator::new(cmp, children.as_mut_ptr(), children.len() as i32);

        let st = merging.status();
        assert!(
            st.is_ok(),
            "status() must be OK when all children report OK"
        );
    }

    #[traced_test]
    fn status_reports_first_non_ok_child_status() {
        trace!("TEST(status): status_reports_first_non_ok_child_status");

        let ok_child = make_ok_child();

        let error_status =
            Status::corruption(&Slice::from("corrupt"), None);
        let err_child = make_error_child(error_status);

        let mut children = [ok_child, err_child];

        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let merging =
            MergingIterator::new(cmp, children.as_mut_ptr(), children.len() as i32);

        let st = merging.status();

        assert!(
            st.is_corruption(),
            "status() must report the first non-OK child status"
        );
    }
}
