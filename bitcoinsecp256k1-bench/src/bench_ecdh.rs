crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/bench_ecdh.c]

pub struct BenchEcdhData {
    ctx:    *mut Secp256k1Context,
    point:  PubKey,
    scalar: [u8; 32],
}

pub fn bench_ecdh_setup(arg: *mut c_void)  {
    
    todo!();
        /*
            int i;
        bench_ecdh_data *data = (bench_ecdh_data*)arg;
        const unsigned char point[] = {
            0x03,
            0x54, 0x94, 0xc1, 0x5d, 0x32, 0x09, 0x97, 0x06,
            0xc2, 0x39, 0x5f, 0x94, 0x34, 0x87, 0x45, 0xfd,
            0x75, 0x7c, 0xe3, 0x0e, 0x4e, 0x8c, 0x90, 0xfb,
            0xa2, 0xba, 0xd1, 0x84, 0xf8, 0x83, 0xc6, 0x9f
        };

        for (i = 0; i < 32; i++) {
            data->scalar[i] = i + 1;
        }
        CHECK(ec_pubkey_parse(data->ctx, &data->point, point, sizeof(point)) == 1);
        */
}

pub fn bench_ecdh(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            int i;
        unsigned char res[32];
        bench_ecdh_data *data = (bench_ecdh_data*)arg;

        for (i = 0; i < iters; i++) {
            CHECK(ecdh(data->ctx, res, &data->point, data->scalar, NULL, NULL) == 1);
        }
        */
}

pub fn secp256k1_bench_ecdh_main() -> i32 {
    
    todo!();
        /*
        bench_ecdh_data data;

        int iters = get_iters(20000);

        /* create a context with no capabilities */
        data.ctx = context_create(FLAGS_TYPE_CONTEXT);

        run_benchmark("ecdh", bench_ecdh, bench_ecdh_setup, NULL, &data, 10, iters);

        context_destroy(data.ctx);
        return 0;
        */
}
