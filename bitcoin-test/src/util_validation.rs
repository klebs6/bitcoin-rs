crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/util/validation.h]
//-------------------------------------------[.cpp/bitcoin/src/test/util/validation.cpp]

pub struct TestChainState {
    base: ChainState,
}

impl TestChainState {

    /**
      | Reset the ibd cache to its initial state
      |
      */
    pub fn reset_ibd(&mut self)  {
        
        todo!();
        /*
            m_cached_finished_ibd = false;
        assert(IsInitialBlockDownload());
        */
    }
    
    /**
      | Toggle IsInitialBlockDownload from
      | true to false
      |
      */
    pub fn jump_out_of_ibd(&mut self)  {
        
        todo!();
        /*
            Assert(IsInitialBlockDownload());
        m_cached_finished_ibd = true;
        Assert(!IsInitialBlockDownload());
        */
    }
}
