crate::ix!();

pub trait RecursiveDynamicUsage {
    fn recursive_dynamic_usage(&self) -> usize;
}

pub fn recursive_dynamic_usage<T: RecursiveDynamicUsage>(x: &T) -> usize {
    x.recursive_dynamic_usage()
}

impl<X> RecursiveDynamicUsage for Arc<X> {

    fn recursive_dynamic_usage(&self) -> usize {

        todo!();
            /*
                return p ? memusage::DynamicUsage(p) + RecursiveDynamicUsage(*p) : 0;
            */
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

