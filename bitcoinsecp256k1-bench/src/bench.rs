// ---------------- [ File: bitcoinsecp256k1-bench/src/bench.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/bench.h]

pub fn gettime_i64() -> i64 {
    
    todo!();
        /*
            struct timeval tv;
        gettimeofday(&tv, NULL);
        return (int64_t)tv.tv_usec + (int64_t)tv.tv_sec * 1000000LL;
        */
}

pub const FP_EXP:  usize = 6;
pub const FP_MULT: usize = 1000000;

/**
  | Format fixed point number.
  |
  */
pub fn print_number(x: i64)  {
    
    todo!();
        /*
            int64_t x_abs, y;
        int c, i, rounding;
        size_t ptr;
        char buffer[30];

        if (x == INT64_MIN) {
            /* Prevent UB. */
            printf("ERR");
            return;
        }
        x_abs = x < 0 ? -x : x;

        /* Determine how many decimals we want to show (more than FP_EXP makes no
         * sense). */
        y = x_abs;
        c = 0;
        while (y > 0LL && y < 100LL * FP_MULT && c < FP_EXP) {
            y *= 10LL;
            c++;
        }

        /* Round to 'c' decimals. */
        y = x_abs;
        rounding = 0;
        for (i = c; i < FP_EXP; ++i) {
            rounding = (y % 10) >= 5;
            y /= 10;
        }
        y += rounding;

        /* Format and print the number. */
        ptr = sizeof(buffer) - 1;
        buffer[ptr] = 0;
        if (c != 0) {
            for (i = 0; i < c; ++i) {
                buffer[--ptr] = '0' + (y % 10);
                y /= 10;
            }
            buffer[--ptr] = '.';
        }
        do {
            buffer[--ptr] = '0' + (y % 10);
            y /= 10;
        } while (y != 0);
        if (x < 0) {
            buffer[--ptr] = '-';
        }
        printf("%s", &buffer[ptr]);
        */
}

pub fn run_benchmark(
        name:      *mut u8,
        benchmark: fn(_0: *mut c_void, _1: i32) -> c_void,
        setup:     fn(_0: *mut c_void) -> c_void,
        teardown:  fn(_0: *mut c_void, _1: i32) -> c_void,
        data:      *mut c_void,
        count:     i32,
        iter:      i32)  {
    
    todo!();
        /*
            int i;
        int64_t min = INT64_MAX;
        int64_t sum = 0;
        int64_t max = 0;
        for (i = 0; i < count; i++) {
            int64_t begin, total;
            if (setup != NULL) {
                setup(data);
            }
            begin = gettime_i64();
            benchmark(data, iter);
            total = gettime_i64() - begin;
            if (teardown != NULL) {
                teardown(data, iter);
            }
            if (total < min) {
                min = total;
            }
            if (total > max) {
                max = total;
            }
            sum += total;
        }
        printf("%s: min ", name);
        print_number(min * FP_MULT / iter);
        printf("us / avg ");
        print_number(((sum * FP_MULT) / count) / iter);
        printf("us / max ");
        print_number(max * FP_MULT / iter);
        printf("us\n");
        */
}

pub fn have_flag(
        argc: i32,
        argv: *mut *mut u8,
        flag: *mut u8) -> i32 {
    
    todo!();
        /*
            char** argm = argv + argc;
        argv++;
        if (argv == argm) {
            return 1;
        }
        while (argv != NULL && argv != argm) {
            if (strcmp(*argv, flag) == 0) {
                return 1;
            }
            argv++;
        }
        return 0;
        */
}

pub fn get_iters(default_iters: i32) -> i32 {
    
    todo!();
        /*
            char* env = getenv("BENCH_ITERS");
        if (env) {
            return strtol(env, NULL, 0);
        } else {
            return default_iters;
        }
        */
}
