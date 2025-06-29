// ---------------- [ File: bitcoin-network/src/set_special.rs ]
crate::ix!();

impl NetAddr {

    /**
      | Parse a Tor or I2P address and set this
      | object to it.
      | 
      | -----------
      | @param[in] addr
      | 
      | Address to parse, for example pg6mmjiyjmcrsslvykfwnntlaru7p5svn6y2ymmju6nubxndf4pscryd.onion
      | or ukeu3k5oycgaauneqgtnvselmt4yemvoilkln7jpvamvfx7dnkdq.b32.i2p.
      | 
      | -----------
      | @return
      | 
      | Whether the operation was successful.
      | @see CNetAddr::IsTor(), CNetAddr::IsI2P()
      |
      */
    pub fn set_special(&mut self, addr: &String) -> bool {
        
        todo!();
        /*
            if (!ValidAsCString(addr)) {
            return false;
        }

        if (SetTor(addr)) {
            return true;
        }

        if (SetI2P(addr)) {
            return true;
        }

        return false;
        */
    }
}
