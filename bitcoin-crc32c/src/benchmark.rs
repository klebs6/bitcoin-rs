crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_benchmark.cc]

pub struct CRC32CBenchmark {
    base:         BenchmarkFixture,
    block_data:   String,
    block_buffer: *const u8,
    block_size:   usize,
}

impl CRC32CBenchmark {

    pub fn set_up(&mut self, state: &BenchmarkState)  {
        
        todo!();
        /*
            block_size_ = static_cast<size_t>(state.range(0));
        block_data_ = std::string(block_size_, 'x');
        block_buffer_ = reinterpret_cast<const uint8_t*>(block_data_.data());
        */
    }
}

lazy_static!{
    /*
    BENCHMARK_DEFINE_F(CRC32CBenchmark, Public)(BenchmarkState& state) {
      uint32_t crc = 0;
      for (auto _ : state)
        crc = crc32c::Extend(crc, block_buffer_, block_size_);
      state.SetBytesProcessed(state.iterations() * block_size_);
    }
    BENCHMARK_REGISTER_F(CRC32CBenchmark, Public)
        ->RangeMultiplier(16)
        ->Range(256, 16777216);  // Block size.
    */
}

lazy_static!{
    /*
    BENCHMARK_DEFINE_F(CRC32CBenchmark, Portable)(BenchmarkState& state) {
      uint32_t crc = 0;
      for (auto _ : state)
        crc = crc32c::ExtendPortable(crc, block_buffer_, block_size_);
      state.SetBytesProcessed(state.iterations() * block_size_);
    }
    BENCHMARK_REGISTER_F(CRC32CBenchmark, Portable)
        ->RangeMultiplier(16)
        ->Range(256, 16777216);  // Block size.
    */
}

#[cfg(HAVE_ARM64_CRC32C)]
lazy_static!{
    /*
    BENCHMARK_DEFINE_F(CRC32CBenchmark, ArmCRC32C)(BenchmarkState& state) {
      if (!crc32c::CanUseArm64Crc32()) {
        state.SkipWithError("ARM CRC32C instructions not available or not enabled");
        return;
      }

      uint32_t crc = 0;
      for (auto _ : state)
        crc = crc32c::ExtendArm64(crc, block_buffer_, block_size_);
      state.SetBytesProcessed(state.iterations() * block_size_);
    }
    BENCHMARK_REGISTER_F(CRC32CBenchmark, ArmCRC32C)
        ->RangeMultiplier(16)
        ->Range(256, 16777216);  // Block size.
    */
}

#[cfg(all(HAVE_SSE42,any(_M_X64,__x86_64__)))]
lazy_static!{
    /*
    BENCHMARK_DEFINE_F(CRC32CBenchmark, Sse42)(BenchmarkState& state) {
      if (!crc32c::CanUseSse42()) {
        state.SkipWithError("SSE4.2 instructions not available or not enabled");
        return;
      }

      uint32_t crc = 0;
      for (auto _ : state)
        crc = crc32c::ExtendSse42(crc, block_buffer_, block_size_);
      state.SetBytesProcessed(state.iterations() * block_size_);
    }
    BENCHMARK_REGISTER_F(CRC32CBenchmark, Sse42)
        ->RangeMultiplier(16)
        ->Range(256, 16777216);  // Block size.
    */
}

pub fn crc32c_crc32c_benchmark_main(
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            #if CRC32C_TESTS_BUILT_WITH_GLOG
      google::InitGoogleLogging(argv[0]);
      google::InstallFailureSignalHandler();
    #endif  // CRC32C_TESTS_BUILT_WITH_GLOG

      benchmark::Initialize(&argc, argv);
      benchmark::RunSpecifiedBenchmarks();
      return 0;
        */
}
