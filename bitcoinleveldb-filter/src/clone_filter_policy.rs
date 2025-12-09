// ---------------- [ File: bitcoinleveldb-filter/src/clone_filter_policy.rs ]
crate::ix!();

pub trait CloneFilterPolicy {
    fn clone_boxed(&self) -> Box<dyn FilterPolicy>;
}

impl<T> CloneFilterPolicy for T
where
    T: FilterPolicy + Clone + 'static,
{
    fn clone_boxed(&self) -> Box<dyn FilterPolicy> {
        Box::new(self.clone())
    }
}
