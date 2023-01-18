crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/key.h]

/**
  | PrivKey is a serialized private key,
  | with all parameters included (SIZE
  | bytes)
  |
  */
pub type PrivKey = Vec<u8,SecureAllocator>;

/**
  | secp256k1:
  |
  */
pub const KEY_SIZE:            u32 = 279;
pub const KEY_COMPRESSED_SIZE: u32 = 214;

/**
  | see www.keylength.com script supports
  | up to 75 for single byte push
  |
  */
const_assert!{
    KEY_SIZE >= KEY_COMPRESSED_SIZE
} //"COMPRESSED_SIZE is larger than SIZE"

//-------------------------------------------[.cpp/bitcoin/src/key.cpp]

lazy_static!{
    /*
    static secp256k1_context* secp256k1_context_sign = nullptr;
    */
}

/*
  | These functions are taken from the libsecp256k1
  | distribution and are very ugly.
  |
  */


/**
  | Check that required EC support is available
  | at runtime.
  |
  */
pub fn ecc_init_sanity_check() -> bool {
    
    todo!();
        /*
            CKey key;
        key.MakeNewKey(true);
        CPubKey pubkey = key.GetPubKey();
        return key.VerifyPubKey(pubkey);
        */
}

/**
  | Initialize the elliptic curve support.
  | May not be called twice without calling
  | ECC_Stop first.
  |
  */
pub fn ecc_start()  {
    
    todo!();
        /*
            assert(secp256k1_context_sign == nullptr);

        secp256k1_context *ctx = secp256k1_context_create(SECP256K1_CONTEXT_SIGN);
        assert(ctx != nullptr);

        {
            // Pass in a random blinding seed to the secp256k1 context.
            std::vector<unsigned char, secure_allocator<unsigned char>> vseed(32);
            GetRandBytes(vseed.data(), 32);
            bool ret = secp256k1_context_randomize(ctx, vseed.data());
            assert(ret);
        }

        secp256k1_context_sign = ctx;
        */
}

/**
  | Deinitialize the elliptic curve support.
  | No-op if ECC_Start wasn't called first.
  |
  */
pub fn ecc_stop()  {
    
    todo!();
        /*
            secp256k1_context *ctx = secp256k1_context_sign;
        secp256k1_context_sign = nullptr;

        if (ctx) {
            secp256k1_context_destroy(ctx);
        }
        */
}
