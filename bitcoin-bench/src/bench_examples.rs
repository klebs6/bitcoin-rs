// ---------------- [ File: bitcoin-bench/src/bench_examples.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/examples.cpp]

lazy_static!{
    /*
    volatile double sum = 0.0; // volatile, global so not optimized away
    */
}

#[bench] fn trig(b: &mut Bencher)  {
    
    todo!();
        /*
            double d = 0.01;
        bench.run([&] {
            sum += sin(d);
            d += 0.000001;
        });
        */
}
