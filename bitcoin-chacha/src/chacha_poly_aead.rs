// ---------------- [ File: bitcoin-chacha/src/chacha_poly_aead.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/chacha_poly_aead.h]

/// A AEAD class for ChaCha20-Poly1305@bitcoin.
/// 
/// ChaCha20 is a stream cipher designed by Daniel Bernstein and described in
/// <ref>[https://cr.yp.to/chacha/chacha-20080128.pdf ChaCha20]</ref>.
/// 
/// It operates by permuting 128 fixed bits,
/// 128 or 256 bits of key, a 64 bit nonce and a 64 bit counter into 64 bytes of
/// output.
/// 
/// This output is used as a keystream, with any unused bytes simply discarded.
/// 
/// Poly1305 <ref>[https://cr.yp.to/mac/poly1305-20050329.pdf Poly1305]</ref>,
/// also by Daniel Bernstein, is a one-time Carter-Wegman MAC that computes
/// a 128 bit integrity tag given a message and a single-use 256 bit secret key.
/// 
/// The chacha20-poly1305@bitcoin combines these two primitives into an
/// authenticated encryption mode.
/// 
/// The construction used is based on that proposed for TLS by Adam Langley in
/// <ref>[http://tools.ietf.org/html/draft-agl-tls-chacha20poly1305-03 "ChaCha20
/// and Poly1305 based Cipher Suites for TLS", Adam Langley]</ref>, but differs
/// in the layout of data passed to the MAC and in the addition of encryption of
/// the packet lengths. 
///
/// ==== Detailed Construction ====
/// 
/// The chacha20-poly1305@bitcoin cipher requires two 256 bits of key material
/// as output from the key exchange.
/// 
/// Each key (K_1 and K_2) are used by two separate instances of chacha20.
/// 
/// The instance keyed by K_1 is a stream cipher that is used only to encrypt
/// the
/// 3 byte packet length field and has its own sequence number.
/// 
/// The second instance, keyed by K_2, is used in conjunction with poly1305 to
/// build an AEAD (Authenticated Encryption with Associated Data) that is used
/// to encrypt and authenticate the entire packet.
/// 
/// Two separate cipher instances are used here so as to keep the packet lengths
/// confidential but not create an oracle for the packet payload cipher by
/// decrypting and using the packet length prior to checking the MAC.
/// 
/// By using an independently-keyed cipher instance to encrypt the length, an
/// active attacker seeking to exploit the packet input handling as a decryption
/// oracle can learn nothing about the payload contents or its MAC (assuming key
/// derivation,
/// 
/// ChaCha20 and Poly1305 are secure).
/// 
/// The AEAD is constructed as follows: for each packet, generate a Poly1305 key
/// by taking the first 256 bits of ChaCha20 stream output generated using K_2,
/// an IV consisting of the packet sequence number encoded as an LE uint64 and
/// a ChaCha20 block counter of zero.
/// 
/// The K_2 ChaCha20 block counter is then set to the little-endian encoding of
/// 1 (i.e. {1, 0, 0, 0, 0, 0, 0, 0}) and this instance is used for encryption
/// of the packet payload. 
///
/// ==== Packet Handling ====
/// 
/// When receiving a packet, the length must be decrypted first. When 3 bytes of
/// ciphertext length have been received, they may be decrypted.
/// 
/// A ChaCha20 round always calculates 64bytes which is sufficient to crypt
/// 21 times a 3 bytes length field (21*3 = 63). The length field sequence
/// number can thus be used 21 times (keystream caching).
/// 
/// The length field must be enc-/decrypted with the ChaCha20 keystream keyed
/// with
/// 
/// K_1 defined by block counter 0, the length field sequence number in little
/// endian and a keystream position from 0 to 60.
/// 
/// Once the entire packet has been received, the MAC MUST be checked before
/// decryption.
/// 
/// A per-packet Poly1305 key is generated as described above and the MAC tag
/// calculated using Poly1305 with this key over the ciphertext of the packet
/// length and the payload together.
/// 
/// The calculated MAC is then compared in constant time with the one appended
/// to the packet and the packet decrypted using ChaCha20 as described above
/// (with K_2, the packet sequence number as nonce and a starting block counter
/// of 1).
/// 
/// Detection of an invalid MAC MUST lead to immediate connection termination.
/// 
/// To send a packet, first encode the 3 byte length and encrypt it using K_1 as
/// described above.
/// 
/// Encrypt the packet payload (using K_2) and append it to the encrypted
/// length. Finally, calculate a MAC tag and append it.
/// 
/// The initiating peer MUST use <code>K_1_A, K_2_A</code> to encrypt messages
/// on the send channel, <code>K_1_B, K_2_B</code> MUST be used to decrypt
/// messages on the receive channel.
/// 
/// The responding peer MUST use <code>K_1_A, K_2_A</code> to decrypt messages
/// on the receive channel, <code>K_1_B, K_2_B</code> MUST be used to encrypt
/// messages on the send channel.
/// 
/// Optimized implementations of ChaCha20-Poly1305@bitcoin are relatively fast
/// in general, therefore it is very likely that encrypted messages require not
/// more
/// 
/// CPU cycles per bytes then the current unencrypted p2p message format
/// (ChaCha20/Poly1305 versus double SHA256).
/// 
/// The initial packet sequence numbers are 0.
/// 
/// K_2 ChaCha20 cipher instance (payload) must never reuse a {key, nonce} for
/// encryption nor may it be used to encrypt more than 2^70 bytes under the same
/// {key, nonce}.
/// 
/// K_1 ChaCha20 cipher instance (length field/AAD) must never reuse a {key,
/// nonce, position-in-keystream} for encryption nor may it be used to encrypt
/// more than 2^70 bytes under the same {key, nonce}.
/// 
/// We use message sequence numbers for both communication directions.
/// 
#[derive(MutGetters,Debug,Getters)]
#[getset(get="pub",get_mut="pub")]
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
        k_1: *const u8,
        k_1_len: usize,
        k_2: *const u8,
        k_2_len: usize,
    ) -> Self {
        trace!("ChaCha20Poly1305AEAD::new");
        assert_eq!(
            k_1_len, CHACHA20_POLY1305_AEAD_KEY_LEN,
            "K₁ must be 256 bits"
        );
        assert_eq!(
            k_2_len, CHACHA20_POLY1305_AEAD_KEY_LEN,
            "K₂ must be 256 bits"
        );

        let mut chacha_header = ChaCha20::default();
        chacha_header.set_key(k_1, k_1_len);

        let mut chacha_main = ChaCha20::default();
        chacha_main.set_key(k_2, k_2_len);

        Self {
            chacha_header,
            chacha_main,
            aad_keystream_buffer: [0u8; CHACHA20_ROUND_OUTPUT],

            // set the cached sequence number to uint64 max which hints for an unset cache.
            // we can't hit uint64 max since the rekey rule (which resets the sequence number) is 1GB
            cached_aad_seqnr: u64::MAX,
        }
    }
}
