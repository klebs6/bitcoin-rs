// ---------------- [ File: bitcoin-bench/src/bench_util_time.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/util_time.cpp]

#[bench] fn bench_time_deprecated(b: &mut Bencher)  {
    
    todo!();
        /*
            bench.run([&] {
            (c_void)GetTime();
        });
        */
}

#[bench] fn bench_time_mock(b: &mut Bencher)  {
    
    todo!();
        /*
            SetMockTime(111);
        bench.run([&] {
            (c_void)GetTime<std::chrono::seconds>();
        });
        SetMockTime(0);
        */
}

#[bench] fn bench_time_millis(b: &mut Bencher)  {
    
    todo!();
        /*
            bench.run([&] {
            (c_void)GetTime<std::chrono::milliseconds>();
        });
        */
}

#[bench] fn bench_time_millis_sys(b: &mut Bencher)  {
    
    todo!();
        /*
            bench.run([&] {
            (c_void)GetTimeMillis();
        });
        */
}
