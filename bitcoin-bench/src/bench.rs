// ---------------- [ File: bitcoin-bench/src/bench.rs ]
/*!
   |  Usage:
   |
   | static c_void NameOfYourBenchmarkFunction(Bench& bench)
   | {
   |     ...do any setup needed...
   |
   |     bench.run([&] {
   |          ...do stuff you want to time; refer to src/bench/nanobench.h
   |             for more information and the options that can be passed here...
   |     });
   |
   |     ...do any cleanup needed...
   | }
   |
   | BENCHMARK(NameOfYourBenchmarkFunction);
   |
   */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/bench.h]

pub type BenchFunction = fn(_0: &mut Bencher) -> ();

pub struct Args {

    is_list_only: bool,

    /// millis
    min_time:     Duration, 

    asymptote:    Vec<f64>,
    output_csv:   String,
    output_json:  String,
    regex_filter: String,
}

pub struct BenchRunner {

}

pub type BenchmarkMap = HashMap<String,BenchFunction>;

/**
  | BENCHMARK(foo) expands to: BenchRunner
  | bench_11foo("foo", foo);
  |
  */
macro_rules! benchmark {
    ($n:ident) => {
        /*
        
            BenchRunner PASTE2(bench_, PASTE2(__LINE__, n))(STRINGIZE(n), n);
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/bench/bench.cpp]

lazy_static!{
    /*
    const std::function<c_void(const std::string&)> G_TEST_LOG_FUN{};
    */
}

pub type NanoBenchResult = Broken;

pub fn generate_template_results(
        benchmark_results: &Vec<NanoBenchResult>,
        filename:          &String,
        tpl:               *const u8)  {
    
    todo!();
        /*
        if (benchmarkResults.empty() || filename.empty()) {
            // nothing to write, bail out
            return;
        }
        std::ofstream fout(filename);
        if (fout.is_open()) {
            ankerl::nanobench::render(tpl, benchmarkResults, fout);
        } else {
            std::cout << "Could write to file '" << filename << "'" << std::endl;
        }

        std::cout << "Created '" << filename << "'" << std::endl;
        */
}

impl BenchRunner {
    
    pub fn benchmarks(&mut self) -> &mut BenchmarkMap {
        
        todo!();
        /*
            static std::map<std::string, BenchFunction> benchmarks_map;
        return benchmarks_map;
        */
    }
    
    pub fn new(
        name: String,
        func: BenchFunction) -> Self {
    
        todo!();
        /*
            benchmarks().insert(std::make_pair(name, func));
        */
    }
    
    pub fn run_all(&mut self, args: &Args)  {
        
        todo!();
        /*
            std::regex reFilter(args.regex_filter);
        std::smatch baseMatch;

        std::vector<ankerl::nanobench::Result> benchmarkResults;
        for (const auto& p : benchmarks()) {
            if (!std::regex_match(p.first, baseMatch, reFilter)) {
                continue;
            }

            if (args.is_list_only) {
                std::cout << p.first << std::endl;
                continue;
            }

            Bench bench;
            bench.name(p.first);
            if (args.min_time > 0ms) {
                // convert to nanos before dividing to reduce rounding errors
                std::chrono::nanoseconds min_time_ns = args.min_time;
                bench.minEpochTime(min_time_ns / bench.epochs());
            }

            if (args.asymptote.empty()) {
                p.second(bench);
            } else {
                for (auto n : args.asymptote) {
                    bench.complexityN(n);
                    p.second(bench);
                }
                std::cout << bench.complexityBigO() << std::endl;
            }

            if (!bench.results().empty()) {
                benchmarkResults.push_back(bench.results().back());
            }
        }

        GenerateTemplateResults(benchmarkResults, args.output_csv, "# Benchmark, evals, iterations, total, min, max, median\n"
                                                                   "{{#result}}{{name}}, {{epochs}}, {{average(iterations)}}, {{sumProduct(iterations, elapsed)}}, {{minimum(elapsed)}}, {{maximum(elapsed)}}, {{median(elapsed)}}\n"
                                                                   "{{/result}}");
        GenerateTemplateResults(benchmarkResults, args.output_json, ankerl::nanobench::templates::json());
        */
    }
}
