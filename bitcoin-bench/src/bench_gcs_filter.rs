crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/gcs_filter.cpp]

#[bench] fn construct_gcs_filter(b: &mut Bencher)  {
    
    todo!();
        /*
            GCSFilter::ElementSet elements;
        for (int i = 0; i < 10000; ++i) {
            GCSFilter::Element element(32);
            element[0] = static_cast<unsigned char>(i);
            element[1] = static_cast<unsigned char>(i >> 8);
            elements.insert(std::move(element));
        }

        uint64_t siphash_k0 = 0;
        bench.batch(elements.size()).unit("elem").run([&] {
            GCSFilter filter({siphash_k0, 0, 20, 1 << 20}, elements);

            siphash_k0++;
        });
        */
}

#[bench] fn match_gcs_filter(b: &mut Bencher)  {
    
    todo!();
        /*
            GCSFilter::ElementSet elements;
        for (int i = 0; i < 10000; ++i) {
            GCSFilter::Element element(32);
            element[0] = static_cast<unsigned char>(i);
            element[1] = static_cast<unsigned char>(i >> 8);
            elements.insert(std::move(element));
        }
        GCSFilter filter({0, 0, 20, 1 << 20}, elements);

        bench.unit("elem").run([&] {
            filter.Match(GCSFilter::Element());
        });
        */
}
