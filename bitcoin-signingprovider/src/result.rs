// ---------------- [ File: bitcoin-signingprovider/src/result.rs ]
crate::ix!();

pub enum SigningResult {
    OK, /// No error
    PRIVATE_KEY_NOT_AVAILABLE,
    SIGNING_FAILED,
}

pub fn signing_result_string(res: SigningResult) -> String {
    
    todo!();
        /*
            switch (res) {
            case SigningResult::OK:
                return "No error";
            case SigningResult::PRIVATE_KEY_NOT_AVAILABLE:
                return "Private key not available";
            case SigningResult::SIGNING_FAILED:
                return "Sign failed";
            // no default case, so the compiler can warn about missing cases
        }
        assert(false);
        */
}
