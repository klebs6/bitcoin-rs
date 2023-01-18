crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/no_destructor_test.cc]

struct DoNotDestruct {

    /**
      | Used to check constructor argument
      | forwarding.
      |
      */
    a: u32,
    b: u64,
}

impl Drop for DoNotDestruct {
    fn drop(&mut self) {
        todo!();
        /*
            std::abort();
        */
    }
}

impl DoNotDestruct {

    pub fn new(a: u32, b: u64) -> Self {
    
        todo!();
        /*
        : a(a),
        : b(b),

        
        */
    }
}

const GOLDENA: u32 = 0xdeadbeef;
const GOLDENB: u64 = 0xaabbccddeeffaabb;

struct NoDestructorTest {}

#[test] fn no_destructor_test_stack_instance() {
    todo!();
    /*
    
      NoDestructor<DoNotDestruct> instance(kGoldenA, kGoldenB);
      ASSERT_EQ(kGoldenA, instance.get()->a);
      ASSERT_EQ(kGoldenB, instance.get()->b);

    */
}

#[test] fn no_destructor_test_static_instance() {
    todo!();
    /*
    
      static NoDestructor<DoNotDestruct> instance(kGoldenA, kGoldenB);
      ASSERT_EQ(kGoldenA, instance.get()->a);
      ASSERT_EQ(kGoldenB, instance.get()->b);

    */
}

fn testno_destructor_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}
