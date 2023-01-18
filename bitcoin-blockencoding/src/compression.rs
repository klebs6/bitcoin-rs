crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/blockencodings.h]

/**
   | Transaction compression schemes for compact
   | block relay can be introduced by writing an
   | actual formatter here.
  */
pub type TransactionCompression<'a,T> = DefaultFormatter<'a,T>;
