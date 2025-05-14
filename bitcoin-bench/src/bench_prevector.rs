// ---------------- [ File: bitcoin-bench/src/bench_prevector.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/prevector.cpp]

pub struct NonTrivial {
    x: i32,
}

lazy_static!{
    /*
    SERIALIZE_METHODS(nonTrivial, obj) { READWRITE(obj.x); }
    */
}

impl Default for NonTrivial {
    
    fn default() -> Self {
        todo!();
        /*
        : x(-1),

        
        */
    }
}


//expected to be trivially constructible
pub type Trivial = u8;

#[bench] fn prevector_destructor<T>(b: &mut Bencher)  {

    todo!();
        /*
            bench.batch(2).run([&] {
            prevector<28, T> t0;
            prevector<28, T> t1;
            t0.resize(28);
            t1.resize(29);
        });
        */
}

#[bench] fn prevector_clear<T>(b: &mut Bencher)  {

    todo!();
        /*
            prevector<28, T> t0;
        prevector<28, T> t1;
        bench.batch(2).run([&] {
            t0.resize(28);
            t0.clear();
            t1.resize(29);
            t1.clear();
        });
        */
}

#[bench] fn prevector_resize<T>(b: &mut Bencher)  {

    todo!();
        /*
            prevector<28, T> t0;
        prevector<28, T> t1;
        bench.batch(4).run([&] {
            t0.resize(28);
            t0.resize(0);
            t1.resize(29);
            t1.resize(0);
        });
        */
}

#[bench] fn prevector_deserialize<T>(b: &mut Bencher)  {

    todo!();
        /*
            DataStream s0(SER_NETWORK, 0);
        prevector<28, T> t0;
        t0.resize(28);
        for (auto x = 0; x < 900; ++x) {
            s0 << t0;
        }
        t0.resize(100);
        for (auto x = 0; x < 101; ++x) {
            s0 << t0;
        }
        bench.batch(1000).run([&] {
            prevector<28, T> t1;
            for (auto x = 0; x < 1000; ++x) {
                s0 >> t1;
            }
            s0.Rewind();
        });
        */
}

macro_rules! prevector_test {
    ($name:ident) => {
        /*
        
            static c_void Prevector##name##Nontrivial(benchmark::Bench& bench) 
            {                                                                
                Prevector##name<nonTrivial>(bench);                        
            }                                                                
            BENCHMARK(Prevector##name##Nontrivial);                          
            static c_void Prevector##name##Trivial(benchmark::Bench& bench)    
            {                                                                
                Prevector##name<Trivial>(bench);                           
            }                                                                
            BENCHMARK(Prevector##name##Trivial);
        */
    }
}

prevector_test!{ Clear }
prevector_test!{ Destructor }
prevector_test!{ Resize }
prevector_test!{ Deserialize }
