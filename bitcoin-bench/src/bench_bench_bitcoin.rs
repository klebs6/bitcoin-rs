// ---------------- [ File: bitcoin-bench/src/bench_bench_bitcoin.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/bench_bitcoin.cpp]

pub const DEFAULT_BENCH_FILTER: &'static str = ".*";
pub const DEFAULT_MIN_TIME_MS:  i64 = 10;

pub fn setup_bench_args(argsman: &mut ArgsManager)  {
    
    todo!();
        /*
        SetupHelpOptions(argsman);

        argsman.AddArg("-asymptote=<n1,n2,n3,...>", "Test asymptotic growth of the runtime of an algorithm, if supported by the benchmark", ArgsManager::ALLOW_ANY, OptionsCategory::OPTIONS);
        argsman.AddArg("-filter=<regex>", strprintf("Regular expression filter to select benchmark by name (default: %s)", DEFAULT_BENCH_FILTER), ArgsManager::ALLOW_ANY, OptionsCategory::OPTIONS);
        argsman.AddArg("-list", "List benchmarks without executing them", ArgsManager::ALLOW_BOOL, OptionsCategory::OPTIONS);
        argsman.AddArg("-min_time=<milliseconds>", strprintf("Minimum runtime per benchmark, in milliseconds (default: %d)", DEFAULT_MIN_TIME_MS), ArgsManager::ALLOW_INT, OptionsCategory::OPTIONS);
        argsman.AddArg("-output_csv=<output.csv>", "Generate CSV file with the most important benchmark results", ArgsManager::ALLOW_ANY, OptionsCategory::OPTIONS);
        argsman.AddArg("-output_json=<output.json>", "Generate JSON file with all benchmark results", ArgsManager::ALLOW_ANY, OptionsCategory::OPTIONS);
        */
}

/**
  | parses a comma separated list like "10,20,30,50"
  |
  */
pub fn parse_asymptote(str_: &String) -> Vec<f64> {
    
    todo!();
        /*
        std::stringstream ss(str);
        std::vector<double> numbers;
        double d;
        char c;
        while (ss >> d) {
            numbers.push_back(d);
            ss >> c;
        }
        return numbers;
        */
}

pub fn bench_bitcoin_main(
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            ArgsManager argsman;
        SetupBenchArgs(argsman);
        SHA256AutoDetect();
        std::string error;
        if (!argsman.ParseParameters(argc, argv, error)) {
            tfm::format(std::cerr, "Error parsing command line arguments: %s\n", error);
            return EXIT_FAILURE;
        }

        if (HelpRequested(argsman)) {
            std::cout << "Usage:  bench_bitcoin [options]\n"
                         "\n"
                      << argsman.GetHelpMessage()
                      << "Description:\n"
                         "\n"
                         "  bench_bitcoin executes microbenchmarks. The quality of the benchmark results\n"
                         "  highly depend on the stability of the machine. It can sometimes be difficult\n"
                         "  to get stable, repeatable results, so here are a few tips:\n"
                         "\n"
                         "  * Use pyperf [1] to disable frequency scaling, turbo boost etc. For best\n"
                         "    results, use CPU pinning and CPU isolation (see [2]).\n"
                         "\n"
                         "  * Each call of run() should do exactly the same work. E.g. inserting into\n"
                         "    a std::vector doesn't do that as it will reallocate on certain calls. Make\n"
                         "    sure each run has exactly the same preconditions.\n"
                         "\n"
                         "  * If results are still not reliable, increase runtime with e.g.\n"
                         "    -min_time=5000 to let a benchmark run for at least 5 seconds.\n"
                         "\n"
                         "  * bench_bitcoin uses nanobench [3] for which there is extensive\n"
                         "    documentation available online.\n"
                         "\n"
                         "Environment Variables:\n"
                         "\n"
                         "  To attach a profiler you can run a benchmark in endless mode. This can be\n"
                         "  done with the environment variable NANOBENCH_ENDLESS. E.g. like so:\n"
                         "\n"
                         "    NANOBENCH_ENDLESS=MuHash ./bench_bitcoin -filter=MuHash\n"
                         "\n"
                         "  In rare cases it can be useful to suppress stability warnings. This can be\n"
                         "  done with the environment variable NANOBENCH_SUPPRESS_WARNINGS, e.g:\n"
                         "\n"
                         "    NANOBENCH_SUPPRESS_WARNINGS=1 ./bench_bitcoin\n"
                         "\n"
                         "Notes:\n"
                         "\n"
                         "  1. pyperf\n"
                         "     https://github.com/psf/pyperf\n"
                         "\n"
                         "  2. CPU pinning & isolation\n"
                         "     https://pyperf.readthedocs.io/en/latest/system.html\n"
                         "\n"
                         "  3. nanobench\n"
                         "     https://github.com/martinus/nanobench\n"
                         "\n";

            return EXIT_SUCCESS;
        }

        typename benchmark::Args args;
        args.asymptote = parseAsymptote(argsman.GetArg("-asymptote", ""));
        args.is_list_only = argsman.GetBoolArg("-list", false);
        args.min_time = std::chrono::milliseconds(argsman.GetIntArg("-min_time", DEFAULT_MIN_TIME_MS));
        args.output_csv = argsman.GetArg("-output_csv", "");
        args.output_json = argsman.GetArg("-output_json", "");
        args.regex_filter = argsman.GetArg("-filter", DEFAULT_BENCH_FILTER);

        typename benchmark::BenchRunner::RunAll(args);

        return EXIT_SUCCESS;
        */
}
