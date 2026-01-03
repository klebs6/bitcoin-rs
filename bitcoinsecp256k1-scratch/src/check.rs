// ---------------- [ File: bitcoinsecp256k1-scratch/src/check.rs ]
crate::ix!();

#[cfg(DETERMINISTIC)]
macro_rules! check {
    ($cond:ident) => {
        /*
                do { 
            if (EXPECT(!(cond), 0)) { 
                TEST_FAILURE("test condition failed"); 
            } 
        } while(0)
        */
    }
}

#[cfg(not(DETERMINISTIC))]
macro_rules! check {
    ($cond:ident) => {
        /*
                do { 
            if (EXPECT(!(cond), 0)) { 
                TEST_FAILURE("test condition failed: " #cond); 
            } 
        } while(0)
        */
    }
}
