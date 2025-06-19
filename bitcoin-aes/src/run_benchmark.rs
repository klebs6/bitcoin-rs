// ---------------- [ File: bitcoin-aes/src/run_benchmark.rs ]
crate::ix!();

pub type BenchHook = fn(_0: *mut c_void);

pub const BENCH_HOOK_NOOP: BenchHook = null_hook;

pub fn null_hook(x: *mut c_void) {}

pub fn run_benchmark(
        name:      &str,
        benchmark: Option<BenchHook>,
        setup:     Option<BenchHook>,
        teardown:  Option<BenchHook>,
        data:      *mut c_void,
        count:     i32,
        iter:      i32)  {

    let benchmark = benchmark.unwrap_or(BENCH_HOOK_NOOP);

    let mut i = i32::default();

    let mut min: f64 = f64::MAX;
    let mut sum: f64 = 0.0;
    let mut max: f64 = 0.0;

    for i in 0..count {

        let mut begin: f64 = f64::default();
        let mut total: f64 = f64::default();

        if let Some(setup) = setup {
            setup(data);
        }

        begin = gettimedouble();

        benchmark(data);

        total = gettimedouble() - begin;

        if let Some(teardown) = teardown {
            teardown(data);
        }

        if total < min {
            min = total;
        }

        if total > max {
            max = total;
        }

        sum += total;
    }

    println!("{}: min ", name);
    print_number(min * 1000000000.0 / iter as f64);
    println!("ns / avg ");
    print_number((sum / count as f64) * 1000000000.0 / iter as f64);
    println!("ns / max ");
    print_number(max * 1000000000.0 / iter as f64);
    println!("ns\n");
}
