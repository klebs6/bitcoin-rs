// ---------------- [ File: bitcoinsecp256k1-bench/src/bench_recover.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/bench_recover.c]

pub struct BenchRecoverData {
    ctx: *mut Secp256k1Context,
    msg: [u8; 32],
    sig: [u8; 64],
}

pub fn bench_recover(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        int i;
        bench_recover_data *data = (bench_recover_data*)arg;
        pubkey pubkey;
        unsigned char pubkeyc[33];

        for (i = 0; i < iters; i++) {
            int j;
            size_t pubkeylen = 33;
            ecdsa_recoverable_signature sig;
            CHECK(ecdsa_recoverable_signature_parse_compact(data->ctx, &sig, data->sig, i % 2));
            CHECK(ecdsa_recover(data->ctx, &pubkey, &sig, data->msg));
            CHECK(ec_pubkey_serialize(data->ctx, pubkeyc, &pubkeylen, &pubkey, EC_COMPRESSED));
            for (j = 0; j < 32; j++) {
                data->sig[j + 32] = data->msg[j];    /* Move former message to S. */
                data->msg[j] = data->sig[j];         /* Move former R to message. */
                data->sig[j] = pubkeyc[j + 1];       /* Move recovered pubkey X coordinate to R (which must be a valid X coordinate). */
            }
        }
        */
}

pub fn bench_recover_setup(arg: *mut c_void)  {
    
    todo!();
        /*
        int i;
        bench_recover_data *data = (bench_recover_data*)arg;

        for (i = 0; i < 32; i++) {
            data->msg[i] = 1 + i;
        }
        for (i = 0; i < 64; i++) {
            data->sig[i] = 65 + i;
        }
        */
}

pub fn secp256k1_bench_recover_main() -> i32 {
    
    todo!();
        /*
        bench_recover_data data;

        int iters = get_iters(20000);

        data.ctx = context_create(CONTEXT_VERIFY);

        run_benchmark("ecdsa_recover", bench_recover, bench_recover_setup, NULL, &data, 10, iters);

        context_destroy(data.ctx);
        return 0;
        */
}
