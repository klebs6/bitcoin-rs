// ---------------- [ File: bitcoin-golombrice/tests/gcs.rs ]
crate::ix!();

#[test] fn gcsfilter_test() {
    todo!();
    /*
    
        GCSFilter::ElementSet included_elements, excluded_elements;
        for (int i = 0; i < 100; ++i) {
            GCSFilter::Element element1(32);
            element1[0] = i;
            included_elements.insert(std::move(element1));

            GCSFilter::Element element2(32);
            element2[1] = i;
            excluded_elements.insert(std::move(element2));
        }

        GCSFilter filter({0, 0, 10, 1 << 10}, included_elements);
        for (const auto& element : included_elements) {
            BOOST_CHECK(filter.Match(element));

            auto insertion = excluded_elements.insert(element);
            BOOST_CHECK(filter.MatchAny(excluded_elements));
            excluded_elements.erase(insertion.first);
        }

    */
}

#[test] fn gcsfilter_default_constructor() {
    todo!();
    /*
    
        GCSFilter filter;
        BOOST_CHECK_EQUAL(filter.GetN(), 0U);
        BOOST_CHECK_EQUAL(filter.GetEncoded().size(), 1U);

        const GCSFilter::Params& params = filter.GetParams();
        BOOST_CHECK_EQUAL(params.m_siphash_k0, 0U);
        BOOST_CHECK_EQUAL(params.m_siphash_k1, 0U);
        BOOST_CHECK_EQUAL(params.m_P, 0);
        BOOST_CHECK_EQUAL(params.m_M, 1U);

    */
}
