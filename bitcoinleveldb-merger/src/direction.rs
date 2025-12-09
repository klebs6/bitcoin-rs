// ---------------- [ File: bitcoinleveldb-merger/src/direction.rs ]
crate::ix!();

/**
  | Which direction is the iterator moving?
  |
  */
#[derive(Debug,PartialEq,Eq,Copy,Clone)]
pub enum MergingIteratorDirection { 
    Forward, 
    Reverse 
}

#[cfg(test)]
mod merging_iterator_direction_tests {
    use super::*;

    #[traced_test]
    fn merging_iterator_direction_equality_and_inequality() {
        trace!("TEST(direction): merging_iterator_direction_equality_and_inequality");

        let f = MergingIteratorDirection::Forward;
        let r = MergingIteratorDirection::Reverse;

        assert_ne!(f, r, "Forward and Reverse must compare as different");
        assert_eq!(
            f,
            MergingIteratorDirection::Forward,
            "Forward must be equal to itself"
        );
        assert_eq!(
            r,
            MergingIteratorDirection::Reverse,
            "Reverse must be equal to itself"
        );
    }

    #[traced_test]
    fn merging_iterator_direction_debug_representation_is_stable() {
        trace!("TEST(direction): merging_iterator_direction_debug_representation_is_stable");

        let f_fmt = format!("{:?}", MergingIteratorDirection::Forward);
        let r_fmt = format!("{:?}", MergingIteratorDirection::Reverse);

        assert_eq!(f_fmt, "Forward");
        assert_eq!(r_fmt, "Reverse");
    }
}
