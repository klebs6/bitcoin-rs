// ---------------- [ File: bitcoin-mem/src/recursive_dynamic_usage.rs ]
crate::ix!();

pub trait RecursiveDynamicUsage {
    fn recursive_dynamic_usage(&self) -> usize;
}

pub fn recursive_dynamic_usage<T: RecursiveDynamicUsage>(x: &T) -> usize {
    x.recursive_dynamic_usage()
}

impl<X: RecursiveDynamicUsage + DynamicUsage> RecursiveDynamicUsage for Arc<X> {
    fn recursive_dynamic_usage(&self) -> usize {
        let own = DynamicUsage::dynamic_usage(self);
        let inner = recursive_dynamic_usage(&**self);
        trace!(
            "RecursiveDynamicUsage<Arc<{}>> own={} inner={} total={}",
            core::any::type_name::<X>(),
            own,
            inner,
            own + inner
        );
        own + inner
    }
}

impl<X> RecursiveDynamicUsage for Amo<X> {

    fn recursive_dynamic_usage(&self) -> usize {

        todo!();
            /*
                return p ? memusage::DynamicUsage(p) + RecursiveDynamicUsage(*p) : 0;
            */
    }
}
