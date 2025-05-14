// ---------------- [ File: bitcoin-tor/src/safecookie_response.rs ]
crate::ix!();

/** 
 | Compute Tor SAFECOOKIE response.
 |
 |    ServerHash is computed as:
 |
 |      HMAC-SHA256("Tor safe cookie
 |                  authentication
 |                  server-to-controller hash",
 |          CookieString | ClientNonce | ServerNonce)
 |
 |    (with the HMAC key as its first argument)
 |
 |    After a controller sends a successful
 |    AUTHCHALLENGE command, the next command sent
 |    on the connection must be an AUTHENTICATE
 |    command, and the only authentication string
 |    which that AUTHENTICATE command will accept
 |    is:
 |
 |      HMAC-SHA256("Tor safe cookie
 |                  authentication
 |                  controller-to-server hash",
 |          CookieString | ClientNonce | ServerNonce)
 |
 */
pub fn compute_response(
        key:          &String,
        cookie:       &Vec<u8>,
        client_nonce: &Vec<u8>,
        server_nonce: &Vec<u8>) -> Vec<u8> {
    
    todo!();
        /*
            CHMAC_SHA256 computeHash((const uint8_t*)key.data(), key.size());
        std::vector<uint8_t> computedHash(CHMAC_SHA256::OUTPUT_SIZE, 0);
        computeHash.Write(cookie.data(), cookie.size());
        computeHash.Write(clientNonce.data(), clientNonce.size());
        computeHash.Write(serverNonce.data(), serverNonce.size());
        computeHash.Finalize(computedHash.data());
        return computedHash;
        */
}
