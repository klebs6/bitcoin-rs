crate::ix!();

/**
  | Interface for public key objects in
  | descriptors.
  |
  */
pub struct PubkeyProvider {

    /**
      | Index of this key expression in the
      | descriptor
      |
      | E.g. If this PubkeyProvider is key1 in
      | multi(2, key1, key2, key3), then
      | m_expr_index = 0
      */
    expr_index: u32,
}

impl From<u32> for PubkeyProvider {

    fn from(exp_index: u32) -> Self {
    
        todo!();
        /*
        : expr_index(exp_index),

        
        */
    }
}
