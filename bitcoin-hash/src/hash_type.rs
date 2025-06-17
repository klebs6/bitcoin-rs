// ---------------- [ File: bitcoin-hash/src/hash_type.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/hash_type.h]

/// Thin wrapper around a fixed‑width hash type that
/// mirrors Bitcoin Core’s `BaseHash` helpers while
/// integrating with Rust’s trait ecosystem.
///
/// All methods are fully in‑line with the original
/// C++ semantics (pointer helpers are retained for
/// 1‑to‑1 porting convenience, even though they are
/// rarely needed in safe Rust).
#[derive(Debug,Clone, Getters)]
pub struct BaseHash<HashType>
where
    HashType: Clone
        + Default
        + Debug
        + AsRef<[u8]>
        + AsMut<[u8]>
        + Ord
        + core::fmt::Display,
{
    #[get] // public getter ‑ keeps the field private
    hash: HashType,
}

impl<HashType> Default for BaseHash<HashType>
where
    HashType: Clone
        + Default
        + Debug
        + AsRef<[u8]>
        + AsMut<[u8]>
        + Ord
        + core::fmt::Display,
{
    #[instrument(level = "trace")]
    fn default() -> Self {
        trace!("BaseHash<…>::default()");
        Self {
            hash: HashType::default(),
        }
    }
}

impl<HashType> BaseHash<HashType>
where
    HashType: Clone
        + Default
        + Debug
        + AsRef<[u8]>
        + AsMut<[u8]>
        + Ord
        + core::fmt::Display,
{
    /// Create a new `BaseHash` by cloning `input`.
    #[instrument(level = "debug")]
    pub fn new(input: &HashType) -> Self {
        Self {
            hash: input.clone(),
        }
    }

    /// Return the number of bytes in the underlying hash.
    #[inline]
    #[instrument(level = "trace", skip(self))]
    pub fn size(&self) -> usize {
        self.hash.as_ref().len()
    }

    #[inline]
    #[instrument(level = "trace", skip(self))]
    pub fn begin(&self) -> *const u8 {
        self.hash.as_ref().as_ptr()
    }

    #[inline]
    #[instrument(level = "trace", skip(self))]
    pub fn begin_mut(&mut self) -> *mut u8 {
        self.hash.as_mut().as_mut_ptr()
    }

    #[inline]
    #[instrument(level = "trace", skip(self))]
    pub fn end(&self) -> *const u8 {
        // SAFETY: adding the exact length is safe
        unsafe { self.begin().add(self.size()) }
    }

    #[inline]
    #[instrument(level = "trace", skip(self))]
    pub fn end_mut(&mut self) -> *mut u8 {
        // SAFETY: adding the exact length is safe
        unsafe { self.begin_mut().add(self.size()) }
    }

    #[inline]
    #[instrument(level = "trace", skip(self))]
    pub fn data(&self) -> *const u8 {
        self.begin()
    }

    #[inline]
    #[instrument(level = "trace", skip(self))]
    pub fn data_mut(&mut self) -> *mut u8 {
        self.begin_mut()
    }

    #[inline]
    #[instrument(level = "trace", skip(self))]
    pub fn to_string(&self) -> String {
        self.hash.to_string()
    }
}

impl<HashType> Into<Vec<u8>> for BaseHash<HashType>
where
    HashType: Clone
        + Default
        + Debug
        + AsRef<[u8]>
        + AsMut<[u8]>
        + Ord
        + core::fmt::Display,
{
    #[instrument(level = "trace", skip(self))]
    fn into(self) -> Vec<u8> {
        self.hash.as_ref().to_vec()
    }
}

impl<HashType> PartialEq for BaseHash<HashType>
where
    HashType: Clone
        + Default
        + Debug
        + AsRef<[u8]>
        + AsMut<[u8]>
        + Ord
        + core::fmt::Display,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl<HashType> Eq for BaseHash<HashType>
where
    HashType: Clone
        + Default
        + Debug
        + AsRef<[u8]>
        + AsMut<[u8]>
        + Ord
        + core::fmt::Display,
{
}

impl<HashType> PartialOrd for BaseHash<HashType>
where
    HashType: Clone
        + Default
        + Debug
        + AsRef<[u8]>
        + AsMut<[u8]>
        + Ord
        + core::fmt::Display,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<HashType> Ord for BaseHash<HashType>
where
    HashType: Clone
        + Default
        + Debug
        + AsRef<[u8]>
        + AsMut<[u8]>
        + Ord
        + core::fmt::Display,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.hash.cmp(&other.hash)
    }
}

/// Deref so downstream code can transparently
/// treat a `BaseHash` as the inner hash type.
impl<HashType> Deref for BaseHash<HashType>
where
    HashType: Clone
        + Default
        + Debug
        + AsRef<[u8]>
        + AsMut<[u8]>
        + Ord
        + core::fmt::Display,
{
    type Target = HashType;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.hash
    }
}

impl<HashType> DerefMut for BaseHash<HashType>
where
    HashType: Clone
        + Default
        + Debug
        + AsRef<[u8]>
        + AsMut<[u8]>
        + Ord
        + core::fmt::Display,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.hash
    }
}
