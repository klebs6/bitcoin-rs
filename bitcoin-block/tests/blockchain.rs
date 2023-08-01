crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/blockchain_tests.cpp]

/**
  | Equality between doubles is imprecise.
  | Comparison should be done with a small
  | threshold of tolerance, rather than
  | exact equality.
  |
  */
pub fn double_equals(
        a:       f64,
        b:       f64,
        epsilon: f64) -> bool {
    
    todo!();
        /*
            return std::abs(a - b) < epsilon;
        */
}

pub fn create_block_index_with_nbits(nbits: u32) -> Arc<Mutex<BlockIndex>> {
    
    todo!();
        /*
            CBlockIndex* block_index = new CBlockIndex();
        block_index->nHeight = 46367;
        block_index->nTime = 1269211443;
        block_index->nBits = nbits;
        return block_index;
        */
}

pub fn reject_difficulty_mismatch(
        difficulty:          f64,
        expected_difficulty: f64)  {
    
    todo!();
        /*
            BOOST_CHECK_MESSAGE(
            DoubleEquals(difficulty, expected_difficulty, 0.00001),
            "Difficulty was " + ToString(difficulty)
                + " but was expected to be " + ToString(expected_difficulty));
        */
}

/**
  | Given a BlockIndex with the provided
  | nbits, verify that the expected difficulty
  | results.
  |
  */
pub fn test_difficulty(
        nbits:               u32,
        expected_difficulty: f64)  {
    
    todo!();
        /*
            CBlockIndex* block_index = CreateBlockIndexWithNbits(nbits);
        double difficulty = GetDifficulty(block_index);
        delete block_index;

        RejectDifficultyMismatch(difficulty, expected_difficulty);
        */
}

#[cfg(test)]
#[BasicTestingSetup]
pub mod blockchain_tests {

    #[test] fn get_difficulty_for_very_low_target() {
        todo!();
        /*
        
            TestDifficulty(0x1f111111, 0.000001);

        */
    }

    #[test] fn get_difficulty_for_low_target() {
        todo!();
        /*
        
            TestDifficulty(0x1ef88f6f, 0.000016);

        */
    }

    #[test] fn get_difficulty_for_mid_target() {
        todo!();
        /*
        
            TestDifficulty(0x1df88f6f, 0.004023);

        */
    }

    #[test] fn get_difficulty_for_high_target() {
        todo!();
        /*
        
            TestDifficulty(0x1cf88f6f, 1.029916);

        */
    }

    #[test] fn get_difficulty_for_very_high_target() {
        todo!();
        /*
        
            TestDifficulty(0x12345678, 5913134931067755359633408.0);

        */
    }
}
