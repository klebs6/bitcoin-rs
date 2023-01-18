crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/chacha_poly_aead.h]

pub const CHACHA20_POLY1305_AEAD_KEY_LEN: usize = 32;
pub const CHACHA20_POLY1305_AEAD_AAD_LEN: usize = 3;  // 3 bytes length
pub const CHACHA20_ROUND_OUTPUT:          usize = 64; // 64 bytes per round
pub const AAD_PACKAGES_PER_ROUND:         usize = 21; // 64 / 3 round down

/**
  | A AEAD class for ChaCha20-Poly1305@bitcoin.
  | 
  | ChaCha20 is a stream cipher designed
  | by Daniel Bernstein and described in
  | <ref>[https://cr.yp.to/chacha/chacha-20080128.pdf
  | ChaCha20]</ref>.
  | 
  | It operates by permuting 128 fixed bits,
  | 128 or 256 bits of key, a 64 bit nonce and
  | a 64 bit counter into 64 bytes of output.
  | 
  | This output is used as a keystream, with
  | any unused bytes simply discarded.
  | 
  | Poly1305 <ref>[https://cr.yp.to/mac/poly1305-20050329.pdf
  | Poly1305]</ref>, also by Daniel Bernstein,
  | is a one-time Carter-Wegman MAC that
  | computes a 128 bit integrity tag given
  | a message and a single-use 256 bit secret
  | key.
  | 
  | The chacha20-poly1305@bitcoin combines
  | these two primitives into an authenticated
  | encryption mode.
  | 
  | The construction used is based on that
  | proposed for TLS by Adam Langley in 
  | <ref>[http://tools.ietf.org/html/draft-agl-tls-chacha20poly1305-03
  | "ChaCha20 and Poly1305 based Cipher
  | Suites for TLS", Adam Langley]</ref>,
  | but differs in the layout of data passed
  | to the MAC and in the addition of encryption
  | of the packet lengths. ==== Detailed
  | Construction ====
  | 
  | The chacha20-poly1305@bitcoin cipher
  | requires two 256 bits of key material
  | as output from the key exchange.
  | 
  | Each key (K_1 and K_2) are used by two
  | separate instances of chacha20.
  | 
  | The instance keyed by K_1 is a stream
  | cipher that is used only to encrypt the
  | 3 byte packet length field and has its
  | own sequence number.
  | 
  | The second instance, keyed by K_2, is
  | used in conjunction with poly1305 to
  | build an AEAD (Authenticated Encryption
  | with Associated Data) that is used to
  | encrypt and authenticate the entire
  | packet.
  | 
  | Two separate cipher instances are used
  | here so as to keep the packet lengths
  | confidential but not create an oracle
  | for the packet payload cipher by decrypting
  | and using the packet length prior to
  | checking the MAC.
  | 
  | By using an independently-keyed cipher
  | instance to encrypt the length, an active
  | attacker seeking to exploit the packet
  | input handling as a decryption oracle
  | can learn nothing about the payload
  | contents or its MAC (assuming key derivation,
  | 
  | ChaCha20 and Poly1305 are secure).
  | 
  | The AEAD is constructed as follows:
  | for each packet, generate a Poly1305
  | key by taking the first 256 bits of ChaCha20
  | stream output generated using K_2,
  | an IV consisting of the packet sequence
  | number encoded as an LE uint64 and a ChaCha20
  | block counter of zero.
  | 
  | The K_2 ChaCha20 block counter is then
  | set to the little-endian encoding of
  | 1 (i.e. {1, 0, 0, 0, 0, 0, 0, 0}) and this
  | instance is used for encryption of the
  | packet payload. ==== Packet Handling
  | ====
  | 
  | When receiving a packet, the length
  | must be decrypted first. When 3 bytes
  | of ciphertext length have been received,
  | they may be decrypted.
  | 
  | A ChaCha20 round always calculates
  | 64bytes which is sufficient to crypt
  | 21 times a 3 bytes length field (21*3
  | = 63). The length field sequence number
  | can thus be used 21 times (keystream
  | caching).
  | 
  | The length field must be enc-/decrypted
  | with the ChaCha20 keystream keyed with
  | 
  | K_1 defined by block counter 0, the length
  | field sequence number in little endian
  | and a keystream position from 0 to 60.
  | 
  | Once the entire packet has been received,
  | the MAC MUST be checked before decryption.
  | 
  | A per-packet Poly1305 key is generated
  | as described above and the
  | MAC tag calculated using Poly1305 with
  | this key over the ciphertext of the packet
  | length and the payload together.
  | 
  | The calculated MAC is then compared
  | in constant time with the one appended
  | to the packet and the packet decrypted
  | using ChaCha20 as described above (with
  | K_2, the packet sequence number as nonce
  | and a starting block counter of 1).
  | 
  | Detection of an invalid MAC MUST lead
  | to immediate connection termination.
  | 
  | To send a packet, first encode the 3 byte
  | length and encrypt it using K_1 as described
  | above.
  | 
  | Encrypt the packet payload (using K_2)
  | and append it to the encrypted length.
  | Finally, calculate a MAC tag and append
  | it.
  | 
  | The initiating peer MUST use <code>K_1_A,
  | K_2_A</code> to encrypt messages on
  | the send channel, <code>K_1_B, K_2_B</code>
  | MUST be used to decrypt messages on the
  | receive channel.
  | 
  | The responding peer MUST use <code>K_1_A,
  | K_2_A</code> to decrypt messages on
  | the receive channel, <code>K_1_B,
  | K_2_B</code> MUST be used to encrypt
  | messages on the send channel.
  | 
  | Optimized implementations of ChaCha20-Poly1305@bitcoin
  | are relatively fast in general, therefore
  | it is very likely that encrypted messages
  | require not more
  | 
  | CPU cycles per bytes then the current
  | unencrypted p2p message format (ChaCha20/Poly1305
  | versus double SHA256).
  | 
  | The initial packet sequence numbers
  | are 0.
  | 
  | K_2 ChaCha20 cipher instance (payload)
  | must never reuse a {key, nonce} for encryption
  | nor may it be used to encrypt more than
  | 2^70 bytes under the same {key, nonce}.
  | 
  | K_1 ChaCha20 cipher instance (length
  | field/AAD) must never reuse a {key,
  | nonce, position-in-keystream} for
  | encryption nor may it be used to encrypt
  | more than 2^70 bytes under the same {key,
  | nonce}.
  | 
  | We use message sequence numbers for
  | both communication directions.
  |
  */
pub struct ChaCha20Poly1305AEAD {

    /**
      | AAD cipher instance (encrypted length)
      | and poly1305 key-derivation cipher
      | instance
      |
      */
    chacha_header:        ChaCha20,

    /**
      | payload
      |
      */
    chacha_main:          ChaCha20,

    /**
      | aad keystream cache
      |
      */
    aad_keystream_buffer: [u8; CHACHA20_ROUND_OUTPUT],

    /**
      | aad keystream cache hint
      |
      */
    cached_aad_seqnr:     u64,
}

impl ChaCha20Poly1305AEAD {

    pub fn new(
        k_1:     *const u8,
        k_1_len: usize,
        k_2:     *const u8,
        k_2_len: usize) -> Self {
    
        todo!();
        /*


            assert(K_1_len == CHACHA20_POLY1305_AEAD_KEY_LEN);
        assert(K_2_len == CHACHA20_POLY1305_AEAD_KEY_LEN);

        m_chacha_header.SetKey(K_1, CHACHA20_POLY1305_AEAD_KEY_LEN);
        m_chacha_main.SetKey(K_2, CHACHA20_POLY1305_AEAD_KEY_LEN);

        // set the cached sequence number to uint64 max which hints for an unset cache.
        // we can't hit uint64 max since the rekey rule (which resets the sequence number) is 1GB
        m_cached_aad_seqnr = std::numeric_limits<uint64_t>::max();
        */
    }

    /**
      | Encrypts/decrypts a packet
      | 
      | -seqnr_payload, the message sequence
      | number
      | 
      | -seqnr_aad, the messages AAD sequence
      | number which allows reuse of the AAD
      | keystream
      | 
      | -aad_pos, position to use in the AAD
      | keystream to encrypt the AAD
      | 
      | -dest, output buffer, must be of a size
      | equal or larger then CHACHA20_POLY1305_AEAD_AAD_LEN
      | + payload (+ POLY1305_TAG_LEN in encryption)
      | bytes
      | 
      | -destlen, length of the destination
      | buffer
      | 
      | -src, the AAD+payload to encrypt or
      | the AAD+payload+MAC to decrypt
      | 
      | -src_len, the length of the source buffer
      | 
      | -is_encrypt, set to true if we encrypt
      | (creates and appends the MAC instead
      | of verifying it)
      |
      */
    pub fn crypt(&mut self, 
        seqnr_payload: u64,
        seqnr_aad:     u64,
        aad_pos:       i32,
        dest:          *mut u8,

        /*
          | length of the output buffer for sanity
          | checks
          |
          */
        dest_len:      usize,
        src:           *const u8,
        src_len:       usize,
        is_encrypt:    bool) -> bool {
        
        todo!();
        /*
            // check buffer boundaries
        if (
            // if we encrypt, make sure the source contains at least the expected AAD and the destination has at least space for the source + MAC
            (is_encrypt && (src_len < CHACHA20_POLY1305_AEAD_AAD_LEN || dest_len < src_len + POLY1305_TAGLEN)) ||
            // if we decrypt, make sure the source contains at least the expected AAD+MAC and the destination has at least space for the source - MAC
            (!is_encrypt && (src_len < CHACHA20_POLY1305_AEAD_AAD_LEN + POLY1305_TAGLEN || dest_len < src_len - POLY1305_TAGLEN))) {
            return false;
        }

        unsigned char expected_tag[POLY1305_TAGLEN], poly_key[POLY1305_KEYLEN];
        memset(poly_key, 0, sizeof(poly_key));
        m_chacha_main.SetIV(seqnr_payload);

        // block counter 0 for the poly1305 key
        // use lower 32bytes for the poly1305 key
        // (throws away 32 unused bytes (upper 32) from this ChaCha20 round)
        m_chacha_main.Seek(0);
        m_chacha_main.Crypt(poly_key, poly_key, sizeof(poly_key));

        // if decrypting, verify the tag prior to decryption
        if (!is_encrypt) {
            const unsigned char* tag = src + src_len - POLY1305_TAGLEN;
            poly1305_auth(expected_tag, src, src_len - POLY1305_TAGLEN, poly_key);

            // constant time compare the calculated MAC with the provided MAC
            if (timingsafe_bcmp(expected_tag, tag, POLY1305_TAGLEN) != 0) {
                memory_cleanse(expected_tag, sizeof(expected_tag));
                memory_cleanse(poly_key, sizeof(poly_key));
                return false;
            }
            memory_cleanse(expected_tag, sizeof(expected_tag));
            // MAC has been successfully verified, make sure we don't covert it in decryption
            src_len -= POLY1305_TAGLEN;
        }

        // calculate and cache the next 64byte keystream block if requested sequence number is not yet the cache
        if (m_cached_aad_seqnr != seqnr_aad) {
            m_cached_aad_seqnr = seqnr_aad;
            m_chacha_header.SetIV(seqnr_aad);
            m_chacha_header.Seek(0);
            m_chacha_header.Keystream(m_aad_keystream_buffer, CHACHA20_ROUND_OUTPUT);
        }
        // crypt the AAD (3 bytes message length) with given position in AAD cipher instance keystream
        dest[0] = src[0] ^ m_aad_keystream_buffer[aad_pos];
        dest[1] = src[1] ^ m_aad_keystream_buffer[aad_pos + 1];
        dest[2] = src[2] ^ m_aad_keystream_buffer[aad_pos + 2];

        // Set the playload ChaCha instance block counter to 1 and crypt the payload
        m_chacha_main.Seek(1);
        m_chacha_main.Crypt(src + CHACHA20_POLY1305_AEAD_AAD_LEN, dest + CHACHA20_POLY1305_AEAD_AAD_LEN, src_len - CHACHA20_POLY1305_AEAD_AAD_LEN);

        // If encrypting, calculate and append tag
        if (is_encrypt) {
            // the poly1305 tag expands over the AAD (3 bytes length) & encrypted payload
            poly1305_auth(dest + src_len, dest, src_len, poly_key);
        }

        // cleanse no longer required MAC and polykey
        memory_cleanse(poly_key, sizeof(poly_key));
        return true;
        */
    }
    
    /**
      | decrypts the 3 bytes AAD data and decodes
      | it into a uint32_t field
      |
      */
    pub fn get_length(&mut self, 
        len24_out:  *mut u32,
        seqnr_aad:  u64,
        aad_pos:    i32,
        ciphertext: *const u8) -> bool {
        
        todo!();
        /*
            // enforce valid aad position to avoid accessing outside of the 64byte keystream cache
        // (there is space for 21 times 3 bytes)
        assert(aad_pos >= 0 && aad_pos < CHACHA20_ROUND_OUTPUT - CHACHA20_POLY1305_AEAD_AAD_LEN);
        if (m_cached_aad_seqnr != seqnr_aad) {
            // we need to calculate the 64 keystream bytes since we reached a new aad sequence number
            m_cached_aad_seqnr = seqnr_aad;
            m_chacha_header.SetIV(seqnr_aad);                                         // use LE for the nonce
            m_chacha_header.Seek(0);                                                  // block counter 0
            m_chacha_header.Keystream(m_aad_keystream_buffer, CHACHA20_ROUND_OUTPUT); // write keystream to the cache
        }

        // decrypt the ciphertext length by XORing the right position of the 64byte keystream cache with the ciphertext
        *len24_out = (ciphertext[0] ^ m_aad_keystream_buffer[aad_pos + 0]) |
                     (ciphertext[1] ^ m_aad_keystream_buffer[aad_pos + 1]) << 8 |
                     (ciphertext[2] ^ m_aad_keystream_buffer[aad_pos + 2]) << 16;

        return true;
        */
    }
}

