// ---------------- [ File: bitcoinsecp256k1-bench/src/bench_verify.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/bench_verify.c]

pub struct BenchVerifyDaya {
    ctx:       *mut Secp256k1Context,
    msg:       [u8; 32],
    key:       [u8; 32],
    sig:       [u8; 72],
    siglen:    usize,
    pubkey:    [u8; 33],
    pubkeylen: usize,

    #[cfg(ENABLE_OPENSSL_TESTS)]
    ec_group:  *mut EC_GROUP,
}

pub fn bench_verify(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            int i;
        bench_verify_data* data = (bench_verify_data*)arg;

        for (i = 0; i < iters; i++) {
            pubkey pubkey;
            ecdsa_signature sig;
            data->sig[data->siglen - 1] ^= (i & 0xFF);
            data->sig[data->siglen - 2] ^= ((i >> 8) & 0xFF);
            data->sig[data->siglen - 3] ^= ((i >> 16) & 0xFF);
            CHECK(ec_pubkey_parse(data->ctx, &pubkey, data->pubkey, data->pubkeylen) == 1);
            CHECK(ecdsa_signature_parse_der(data->ctx, &sig, data->sig, data->siglen) == 1);
            CHECK(ecdsa_verify(data->ctx, &sig, data->msg, &pubkey) == (i == 0));
            data->sig[data->siglen - 1] ^= (i & 0xFF);
            data->sig[data->siglen - 2] ^= ((i >> 8) & 0xFF);
            data->sig[data->siglen - 3] ^= ((i >> 16) & 0xFF);
        }
        */
}

#[cfg(ENABLE_OPENSSL_TESTS)]
pub fn bench_verify_openssl(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
       int i;
        bench_verify_data* data = (bench_verify_data*)arg;

        for (i = 0; i < iters; i++) {
            data->sig[data->siglen - 1] ^= (i & 0xFF);
            data->sig[data->siglen - 2] ^= ((i >> 8) & 0xFF);
            data->sig[data->siglen - 3] ^= ((i >> 16) & 0xFF);
            {
                EC_KEY *pkey = EC_KEY_new();
                const unsigned char *pubkey = &data->pubkey[0];
                int result;

                CHECK(pkey != NULL);
                result = EC_KEY_set_group(pkey, data->ec_group);
                CHECK(result);
                result = (o2i_ECPublicKey(&pkey, &pubkey, data->pubkeylen)) != NULL;
                CHECK(result);
                result = ECDSA_verify(0, &data->msg[0], sizeof(data->msg), &data->sig[0], data->siglen, pkey) == (i == 0);
                CHECK(result);
                EC_KEY_free(pkey);
            }
            data->sig[data->siglen - 1] ^= (i & 0xFF);
            data->sig[data->siglen - 2] ^= ((i >> 8) & 0xFF);
            data->sig[data->siglen - 3] ^= ((i >> 16) & 0xFF);
        }
        */
}

pub fn secp256k1_bench_verify_main() -> i32 {
    
    todo!();
        /*
            int i;
        pubkey pubkey;
        ecdsa_signature sig;
        bench_verify_data data;

        int iters = get_iters(20000);

        data.ctx = context_create(CONTEXT_SIGN | CONTEXT_VERIFY);

        for (i = 0; i < 32; i++) {
            data.msg[i] = 1 + i;
        }
        for (i = 0; i < 32; i++) {
            data.key[i] = 33 + i;
        }
        data.siglen = 72;
        CHECK(ecdsa_sign(data.ctx, &sig, data.msg, data.key, NULL, NULL));
        CHECK(ecdsa_signature_serialize_der(data.ctx, data.sig, &data.siglen, &sig));
        CHECK(ec_pubkey_create(data.ctx, &pubkey, data.key));
        data.pubkeylen = 33;
        CHECK(ec_pubkey_serialize(data.ctx, data.pubkey, &data.pubkeylen, &pubkey, EC_COMPRESSED) == 1);

        run_benchmark("ecdsa_verify", bench_verify, NULL, NULL, &data, 10, iters);
    #ifdef ENABLE_OPENSSL_TESTS
        data.ec_group = EC_GROUP_new_by_curve_name(NID_secp256k1);
        run_benchmark("ecdsa_verify_openssl", bench_verify_openssl, NULL, NULL, &data, 10, iters);
        EC_GROUP_free(data.ec_group);
    #endif

        context_destroy(data.ctx);
        return 0;
        */
}
