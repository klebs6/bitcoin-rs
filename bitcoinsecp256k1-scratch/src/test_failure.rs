// ---------------- [ File: bitcoinsecp256k1-scratch/src/test_failure.rs ]
crate::ix!();

#[cfg(DETERMINISTIC)]
macro_rules! test_failure {
    ($msg:ident) => {
        /*
                do { 
            fprintf(stderr, "%s\n", msg); 
            abort(); 
        } while(0);
        */
    }
}

#[cfg(not(DETERMINISTIC))]
macro_rules! test_failure {
    ($msg:ident) => {
        /*
                do { 
            fprintf(stderr, "%s:%d: %s\n", __FILE__, __LINE__, msg); 
            abort(); 
        } while(0)
        */
    }
}
