// ---------------- [ File: bitcoin-fuzz/src/fuzz_checkqueue.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/checkqueue.cpp]

#[derive(Default)]
pub struct DumbCheck {
    result: bool, // default = false
}

impl DumbCheck {
    
    pub fn new(result: bool) -> Self {
    
        todo!();
        /*
        : result(_result),

        
        */
    }
    
    pub fn invoke(&self) -> bool {
        
        todo!();
        /*
            return result;
        */
    }
    
    pub fn swap(&mut self, x: &mut DumbCheck)  {
        
        todo!();
        /*
        
        */
    }
}

#[fuzz_test] fn checkqueue() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());

        const unsigned int batch_size = fuzzed_data_provider.ConsumeIntegralInRange<unsigned int>(0, 1024);
        CCheckQueue<DumbCheck> check_queue_1{batch_size};
        CCheckQueue<DumbCheck> check_queue_2{batch_size};
        std::vector<DumbCheck> checks_1;
        std::vector<DumbCheck> checks_2;
        const int size = fuzzed_data_provider.ConsumeIntegralInRange<int>(0, 1024);
        for (int i = 0; i < size; ++i) {
            const bool result = fuzzed_data_provider.ConsumeBool();
            checks_1.emplace_back(result);
            checks_2.emplace_back(result);
        }
        if (fuzzed_data_provider.ConsumeBool()) {
            check_queue_1.Add(checks_1);
        }
        if (fuzzed_data_provider.ConsumeBool()) {
            (c_void)check_queue_1.Wait();
        }

        CCheckQueueControl<DumbCheck> check_queue_control{&check_queue_2};
        if (fuzzed_data_provider.ConsumeBool()) {
            check_queue_control.Add(checks_2);
        }
        if (fuzzed_data_provider.ConsumeBool()) {
            (c_void)check_queue_control.Wait();
        }

    */
}
