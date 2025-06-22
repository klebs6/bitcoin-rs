// ---------------- [ File: bitcoin-indirectmap/src/traits.rs ]
crate::ix!();

/// Trait used in the Bitcoin code‑base to expose
/// iterator and size types without leaking the
/// concrete map implementation.
pub trait HasIndirectMapTypes<'a>
where
    Self::Key:  'a,
    Self::Value: 'a,
{
    type Iterator:       Iterator<Item = (&'a Arc<Self::Key>, &'a Self::Value)> + 'a;
    type ConstIterator:  Iterator<Item = (&'a Arc<Self::Key>, &'a Self::Value)> + 'a;
    type SizeType;
    type Key;
    type Value;
}


impl<'a, K, V> HasIndirectMapTypes<'a> for IndirectMap<K, V>
where
    K: Ord + 'a,
    V: 'a,
{
    type Iterator = Box<dyn Iterator<Item = (&'a Arc<K>, &'a V)> + 'a>;
    type ConstIterator = Box<dyn Iterator<Item = (&'a Arc<K>, &'a V)> + 'a>;
    type SizeType = usize;
    type Key = K;
    type Value = V;
}
