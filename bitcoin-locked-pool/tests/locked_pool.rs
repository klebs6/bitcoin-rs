// ---------------- [ File: bitcoin-locked-pool/tests/locked_pool.rs ]
use bitcoin_support::*;


#[test] fn lockedpool_tests_mock() {
    todo!();
    /*
    
        // Test over three virtual arenas, of which one will succeed being locked
        std::unique_ptr<LockedPageAllocator> x = std::make_unique<TestLockedPageAllocator>(3, 1);
        LockedPool pool(std::move(x));
        BOOST_CHECK(pool.stats().total == 0);
        BOOST_CHECK(pool.stats().locked == 0);

        // Ensure unreasonable requests are refused without allocating anything
        c_void *invalid_toosmall = pool.alloc(0);
        BOOST_CHECK(invalid_toosmall == nullptr);
        BOOST_CHECK(pool.stats().used == 0);
        BOOST_CHECK(pool.stats().free == 0);
        c_void *invalid_toobig = pool.alloc(LockedPool::ARENA_SIZE+1);
        BOOST_CHECK(invalid_toobig == nullptr);
        BOOST_CHECK(pool.stats().used == 0);
        BOOST_CHECK(pool.stats().free == 0);

        c_void *a0 = pool.alloc(LockedPool::ARENA_SIZE / 2);
        BOOST_CHECK(a0);
        BOOST_CHECK(pool.stats().locked == LockedPool::ARENA_SIZE);
        c_void *a1 = pool.alloc(LockedPool::ARENA_SIZE / 2);
        BOOST_CHECK(a1);
        c_void *a2 = pool.alloc(LockedPool::ARENA_SIZE / 2);
        BOOST_CHECK(a2);
        c_void *a3 = pool.alloc(LockedPool::ARENA_SIZE / 2);
        BOOST_CHECK(a3);
        c_void *a4 = pool.alloc(LockedPool::ARENA_SIZE / 2);
        BOOST_CHECK(a4);
        c_void *a5 = pool.alloc(LockedPool::ARENA_SIZE / 2);
        BOOST_CHECK(a5);
        // We've passed a count of three arenas, so this allocation should fail
        c_void *a6 = pool.alloc(16);
        BOOST_CHECK(!a6);

        pool.free(a0);
        pool.free(a2);
        pool.free(a4);
        pool.free(a1);
        pool.free(a3);
        pool.free(a5);
        BOOST_CHECK(pool.stats().total == 3*LockedPool::ARENA_SIZE);
        BOOST_CHECK(pool.stats().locked == LockedPool::ARENA_SIZE);
        BOOST_CHECK(pool.stats().used == 0);

    */
}

/**
  | These tests used the live LockedPoolManager
  | object, this is also used by other tests so the
  | conditions are somewhat less controllable and
  | thus the tests are somewhat more error-prone.
  */
#[test] fn lockedpool_tests_live() {
    todo!();
    /*
    
        LockedPoolManager &pool = LockedPoolManager::Instance();
        LockedPool::Stats initial = pool.stats();

        c_void *a0 = pool.alloc(16);
        BOOST_CHECK(a0);
        // Test reading and writing the allocated memory
        *((uint32_t*)a0) = 0x1234;
        BOOST_CHECK(*((uint32_t*)a0) == 0x1234);

        pool.free(a0);
        try { // Test exception on double-free
            pool.free(a0);
            BOOST_CHECK(0);
        } catch(std::runtime_error &)
        {
        }
        // If more than one new arena was allocated for the above tests, something is wrong
        BOOST_CHECK(pool.stats().total <= (initial.total + LockedPool::ARENA_SIZE));
        // Usage must be back to where it started
        BOOST_CHECK(pool.stats().used == initial.used);

    */
}
