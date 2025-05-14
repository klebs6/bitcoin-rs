// ---------------- [ File: bitcoinsecp256k1-bench/src/bench_schnorrsig.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/bench_schnorrsig.c]

pub const MSGLEN: usize = 32;

pub struct BenchSchnorrSigData {
    ctx:      *mut Secp256k1Context,
    n:        i32,
    keypairs: *const *const KeyPair,
    pk:       *const *const u8,
    sigs:     *const *const u8,
    msgs:     *const *const u8,
}

pub fn bench_schnorrsig_sign(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        bench_schnorrsig_data *data = (bench_schnorrsig_data *)arg;
        int i;
        unsigned char msg[MSGLEN] = {0};
        unsigned char sig[64];

        for (i = 0; i < iters; i++) {
            msg[0] = i;
            msg[1] = i >> 8;
            CHECK(schnorrsig_sign_custom(data->ctx, sig, msg, MSGLEN, data->keypairs[i], NULL));
        }
        */
}

pub fn bench_schnorrsig_verify(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        bench_schnorrsig_data *data = (bench_schnorrsig_data *)arg;
        int i;

        for (i = 0; i < iters; i++) {
            xonly_pubkey pk;
            CHECK(xonly_pubkey_parse(data->ctx, &pk, data->pk[i]) == 1);
            CHECK(schnorrsig_verify(data->ctx, data->sigs[i], data->msgs[i], MSGLEN, &pk));
        }
        */
}

pub fn secp256k1_bench_schnorrsig_main() -> i32 {
    
    todo!();
        /*
        int i;
        bench_schnorrsig_data data;
        int iters = get_iters(10000);

        data.ctx = context_create(CONTEXT_VERIFY | CONTEXT_SIGN);
        data.keypairs = (const keypair **)malloc(iters * sizeof(keypair *));
        data.pk = (const unsigned char **)malloc(iters * sizeof(unsigned char *));
        data.msgs = (const unsigned char **)malloc(iters * sizeof(unsigned char *));
        data.sigs = (const unsigned char **)malloc(iters * sizeof(unsigned char *));

        CHECK(MSGLEN >= 4);
        for (i = 0; i < iters; i++) {
            unsigned char sk[32];
            unsigned char *msg = (unsigned char *)malloc(MSGLEN);
            unsigned char *sig = (unsigned char *)malloc(64);
            keypair *keypair = (keypair *)malloc(sizeof(*keypair));
            unsigned char *pk_char = (unsigned char *)malloc(32);
            xonly_pubkey pk;
            msg[0] = sk[0] = i;
            msg[1] = sk[1] = i >> 8;
            msg[2] = sk[2] = i >> 16;
            msg[3] = sk[3] = i >> 24;
            memset(&msg[4], 'm', MSGLEN - 4);
            memset(&sk[4], 's', 28);

            data.keypairs[i] = keypair;
            data.pk[i] = pk_char;
            data.msgs[i] = msg;
            data.sigs[i] = sig;

            CHECK(keypair_create(data.ctx, keypair, sk));
            CHECK(schnorrsig_sign_custom(data.ctx, sig, msg, MSGLEN, keypair, NULL));
            CHECK(keypair_xonly_pub(data.ctx, &pk, NULL, keypair));
            CHECK(xonly_pubkey_serialize(data.ctx, pk_char, &pk) == 1);
        }

        run_benchmark("schnorrsig_sign", bench_schnorrsig_sign, NULL, NULL, (c_void *) &data, 10, iters);
        run_benchmark("schnorrsig_verify", bench_schnorrsig_verify, NULL, NULL, (c_void *) &data, 10, iters);

        for (i = 0; i < iters; i++) {
            free((c_void *)data.keypairs[i]);
            free((c_void *)data.pk[i]);
            free((c_void *)data.msgs[i]);
            free((c_void *)data.sigs[i]);
        }
        free(data.keypairs);
        free(data.pk);
        free(data.msgs);
        free(data.sigs);

        context_destroy(data.ctx);
        return 0;
        */
}
