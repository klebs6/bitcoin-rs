crate::ix!();

#[macro_export]
macro_rules! impl_u256_traits {
    () => {
        impl ::core::default::Default for u256 {
            fn default() -> Self {
                // Create a new zeroed BaseBlob<256>
                // Then wrap it in u256
                Self {
                    blob: BaseBlob::<256>::default(),
                }
            }
        }

        // (Optionally you can keep a real Debug that shows the hex, etc.)
        impl ::core::fmt::Debug for u256 {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                // For example:
                write!(f, "u256({})", self.to_string())
            }
        }

        impl ::core::clone::Clone for u256 {
            fn clone(&self) -> Self {
                let mut out = Self::default();
                // copy all 32 bytes from self:
                out.as_slice_mut().copy_from_slice(self.as_slice());
                out
            }
        }

        impl ::core::cmp::PartialEq for u256 {
            fn eq(&self, other: &Self) -> bool {
                self.as_slice() == other.as_slice()
            }
        }

        impl ::core::cmp::Eq for u256 {}

        impl ::core::cmp::PartialOrd for u256 {
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                Some(self.as_slice().cmp(other.as_slice()))
            }
        }

        impl ::core::cmp::Ord for u256 {
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                self.as_slice().cmp(other.as_slice())
            }
        }

        impl ::core::hash::Hash for u256 {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.as_slice().hash(state);
            }
        }
    };
}
