crate::ix!();

/**
  | First two bits of nFlags control how
  | much IsRelevantAndUpdate actually
  | updates
  | 
  | The remaining bits are reserved
  |
  */
#[repr(u8)]
pub enum BloomFlags
{
    BLOOM_UPDATE_NONE          = 0,
    BLOOM_UPDATE_ALL           = 1,

    /**
      | Only adds outpoints to the filter if
      | the output is a pay-to-pubkey/pay-to-multisig
      | script
      |
      */
    BLOOM_UPDATE_P2PUBKEY_ONLY = 2,
    BLOOM_UPDATE_MASK          = 3,
}
