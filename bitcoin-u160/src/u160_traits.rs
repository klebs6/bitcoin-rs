crate::ix!();

#[macro_export]
macro_rules! impl_u160_traits {
    () => {
        impl ::core::default::Default for u160 {
            fn default() -> Self {
                Self {
                    blob: BaseBlob::<Bits160>::default(),
                }
            }
        }

        impl ::core::fmt::Debug for u160 {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "u160({})", self.to_string())
            }
        }

        impl ::core::clone::Clone for u160 {
            fn clone(&self) -> Self {
                let mut out = Self::default();
                out.as_slice_mut().copy_from_slice(self.as_slice());
                out
            }
        }

        impl ::core::cmp::PartialEq for u160 {
            fn eq(&self, other: &Self) -> bool {
                self.as_slice() == other.as_slice()
            }
        }

        impl ::core::cmp::Eq for u160 {}

        impl ::core::cmp::PartialOrd for u160 {
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                Some(self.as_slice().cmp(other.as_slice()))
            }
        }

        impl ::core::cmp::Ord for u160 {
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                self.as_slice().cmp(other.as_slice())
            }
        }

        impl ::core::hash::Hash for u160 {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.as_slice().hash(state);
            }
        }
    };
}
