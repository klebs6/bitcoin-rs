crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/raii_event_tests.cpp]

#[cfg(test)]
#[fixture(BasicTestingSetup)]
pub mod raii_event_tests {

    #[cfg(EVENT_SET_MEM_FUNCTIONS_IMPLEMENTED)]
    lazy_static!{
        /*
        static std::map<c_void*, short> tags;
        static std::map<c_void*, uint16_t> orders;
        static uint16_t tagSequence = 0;
        */
    }

    #[cfg(EVENT_SET_MEM_FUNCTIONS_IMPLEMENTED)]
    pub fn tag_malloc(sz: usize)  {
        
        todo!();
            /*
                c_void* mem = malloc(sz);
            if (!mem) return mem;
            tags[mem]++;
            orders[mem] = tagSequence++;
            return mem;
            */
    }

    #[cfg(EVENT_SET_MEM_FUNCTIONS_IMPLEMENTED)]
    pub fn tag_free(mem: *mut c_void)  {
        
        todo!();
            /*
                tags[mem]--;
            orders[mem] = tagSequence++;
            free(mem);
            */
    }

    #[cfg(EVENT_SET_MEM_FUNCTIONS_IMPLEMENTED)]
    #[test] fn raii_event_creation() {
        todo!();
        /*
        
            event_set_mem_functions(tag_malloc, realloc, tag_free);

            c_void* base_ptr = nullptr;
            {
                auto base = obtain_event_base();
                base_ptr = (c_void*)base.get();
                BOOST_CHECK(tags[base_ptr] == 1);
            }
            BOOST_CHECK(tags[base_ptr] == 0);

            c_void* event_ptr = nullptr;
            {
                auto base = obtain_event_base();
                auto event = obtain_event(base.get(), -1, 0, nullptr, nullptr);

                base_ptr = (c_void*)base.get();
                event_ptr = (c_void*)event.get();

                BOOST_CHECK(tags[base_ptr] == 1);
                BOOST_CHECK(tags[event_ptr] == 1);
            }
            BOOST_CHECK(tags[base_ptr] == 0);
            BOOST_CHECK(tags[event_ptr] == 0);

            event_set_mem_functions(malloc, realloc, free);

        */
    }

    #[cfg(EVENT_SET_MEM_FUNCTIONS_IMPLEMENTED)]
    #[test] fn raii_event_order() {
        todo!();
        /*
        
            event_set_mem_functions(tag_malloc, realloc, tag_free);

            c_void* base_ptr = nullptr;
            c_void* event_ptr = nullptr;
            {
                auto base = obtain_event_base();
                auto event = obtain_event(base.get(), -1, 0, nullptr, nullptr);

                base_ptr = (c_void*)base.get();
                event_ptr = (c_void*)event.get();

                // base should have allocated before event
                BOOST_CHECK(orders[base_ptr] < orders[event_ptr]);
            }
            // base should be freed after event
            BOOST_CHECK(orders[base_ptr] > orders[event_ptr]);

            event_set_mem_functions(malloc, realloc, free);

        */
    }

    #[cfg(not(EVENT_SET_MEM_FUNCTIONS_IMPLEMENTED))]
    #[test] fn raii_event_tests_skipped() {
        todo!();
        /*
        
            // It would probably be ideal to report skipped, but boost::test doesn't seem to make that practical (at least not in versions available with common distros)
            BOOST_TEST_MESSAGE("Skipping raii_event_tess: libevent doesn't support event_set_mem_functions");

        */
    }
}
