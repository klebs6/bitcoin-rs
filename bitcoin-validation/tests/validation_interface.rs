// ---------------- [ File: bitcoin-validation/tests/validation_interface.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/validationinterface_tests.cpp]

pub struct TestSubscriberNoop {
    base: ValidationInterface,
}

impl TestSubscriberNoop {
    
    pub fn block_checked(&mut self, 
        _0: &Block,
        _1: &BlockValidationState)  { }
}

#[test] fn unregister_validation_interface_race() {
    todo!();
    /*
    
        std::atomic<bool> generate{true};

        // Start thread to generate notifications
        std::thread gen{[&] {
            const CBlock block_dummy;
            BlockValidationState state_dummy;
            while (generate) {
                GetMainSignals().BlockChecked(block_dummy, state_dummy);
            }
        }};

        // Start thread to consume notifications
        std::thread sub{[&] {
            // keep going for about 1 sec, which is 250k iterations
            for (int i = 0; i < 250000; i++) {
                auto sub = std::make_shared<TestSubscriberNoop>();
                RegisterSharedValidationInterface(sub);
                UnregisterSharedValidationInterface(sub);
            }
            // tell the other thread we are done
            generate = false;
        }};

        gen.join();
        sub.join();
        BOOST_CHECK(!generate);

    */
}

pub struct TestInterface {
    base:       ValidationInterface,
    on_call:    fn() -> (),
    on_destroy: fn() -> (),
}

impl Drop for TestInterface {
    fn drop(&mut self) {
        todo!();
        /*
            if (m_on_destroy) m_on_destroy();
        */
    }
}

impl TestInterface {

    pub fn new(
        on_call:    fn() -> (),
        on_destroy: fn() -> ()) -> Self {
    
        todo!();
        /*


            : m_on_call(std::move(on_call)), m_on_destroy(std::move(on_destroy))
        */
    }
    
    pub fn block_checked(&mut self, 
        block: &Block,
        state: &BlockValidationState)  {
        
        todo!();
        /*
            if (m_on_call) m_on_call();
        */
    }
    
    pub fn call()  {
        
        todo!();
        /*
            CBlock block;
            BlockValidationState state;
            GetMainSignals().BlockChecked(block, state);
        */
    }
}

/**
  | Regression test to ensure
  | UnregisterAllValidationInterfaces calls don't
  | destroy a validation interface while it is
  | being called. Bug:
  | https://github.com/bitcoin/bitcoin/pull/18551
  */
#[test] fn unregister_all_during_call() {
    todo!();
    /*
    
        bool destroyed = false;
        RegisterSharedValidationInterface(std::make_shared<TestInterface>(
            [&] {
                // First call should decrements reference count 2 -> 1
                UnregisterAllValidationInterfaces();
                BOOST_CHECK(!destroyed);
                // Second call should not decrement reference count 1 -> 0
                UnregisterAllValidationInterfaces();
                BOOST_CHECK(!destroyed);
            },
            [&] { destroyed = true; }));
        TestInterface::Call();
        BOOST_CHECK(destroyed);

    */
}
