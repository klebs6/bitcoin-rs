// ---------------- [ File: bitcoinsecp256k1-bench/src/bench_sign.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/bench_sign.c]

pub struct BenchSignData {
    ctx: *mut Secp256k1Context,
    msg: [u8; 32],
    key: [u8; 32],
}

pub fn bench_sign_setup(arg: *mut c_void)  {
    
    todo!();
        /*
            int i;
        bench_sign_data *data = (bench_sign_data*)arg;

        for (i = 0; i < 32; i++) {
            data->msg[i] = i + 1;
        }
        for (i = 0; i < 32; i++) {
            data->key[i] = i + 65;
        }
        */
}

pub fn bench_sign_run(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            int i;
        bench_sign_data *data = (bench_sign_data*)arg;

        unsigned char sig[74];
        for (i = 0; i < iters; i++) {
            size_t siglen = 74;
            int j;
            ecdsa_signature signature;
            CHECK(ecdsa_sign(data->ctx, &signature, data->msg, data->key, NULL, NULL));
            CHECK(ecdsa_signature_serialize_der(data->ctx, sig, &siglen, &signature));
            for (j = 0; j < 32; j++) {
                data->msg[j] = sig[j];
                data->key[j] = sig[j + 32];
            }
        }
        */
}

pub fn secp256k1_bench_sign_main() -> i32 {
    
    todo!();
        /*
            bench_sign_data data;

        int iters = get_iters(20000);

        data.ctx = context_create(CONTEXT_SIGN);

        run_benchmark("ecdsa_sign", bench_sign_run, bench_sign_setup, NULL, &data, 10, iters);

        context_destroy(data.ctx);
        return 0;
        */
}
