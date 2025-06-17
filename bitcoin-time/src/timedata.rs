// ---------------- [ File: bitcoin-time/src/timedata.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/timedata.h]

pub const DEFAULT_MAX_TIME_ADJUSTMENT: i64 = 70 * 60;

/// Median filter over a stream of values.
///
/// Returns the median of the last N numbers
///
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct MedianFilter<T> {
    values: Vec<T>,
    sorted: Vec<T>,
    n_size: u32,
}

impl<T> MedianFilter<T>
where
    T: Copy
        + PartialOrd
        + Add<Output = T>
        + Div<Output = T>
        + From<u8>,
{
    pub fn new(size: u32, initial_value: T) -> Self {
        let mut values = Vec::with_capacity(size as usize);
        values.push(initial_value);
        let sorted = values.clone();
        Self {
            values,
            sorted,
            n_size: size,
        }
    }

    pub fn input(&mut self, value: T) {
        if self.values.len() == self.n_size as usize {
            self.values.remove(0);
        }
        self.values.push(value);

        self.sorted = self.values.clone();
        self.sorted
            .sort_by(|a, b| a.partial_cmp(b).expect("NaN not supported"));
    }

    pub fn median(&self) -> T {
        let len = self.sorted.len();
        assert!(len > 0, "median on empty MedianFilter");
        if len & 1 == 1 {
            // odd
            self.sorted[len / 2]
        } else {
            // even
            (self.sorted[len / 2 - 1] + self.sorted[len / 2]) / T::from(2u8)
        }
    }

    pub fn size(&self) -> i32 {
        self.values.len() as i32
    }
}

//-------------------------------------------[.cpp/bitcoin/src/timedata.cpp]

/// "Never go to sea with two chronometers;
/// take one or three."
/// 
/// Our three time sources are:
/// 
/// - System clock
/// 
/// - Median of other nodes clocks
/// 
/// - The user (asking the user to fix the
/// system clock if the first two disagree)
/// 
/// Return the current P2P time offset (seconds).
pub fn get_time_offset() -> i64 {
    let off = *TIME_OFFSET.lock();
    trace!(offset_seconds = off, "get_time_offset");
    off
}

#[inline] pub fn get_adjusted_datetime() -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(get_adjusted_time()).unwrap()
}

/// Return Unix‑epoch seconds adjusted by the P2P time offset.
pub fn get_adjusted_time() -> i64 {
    let t = get_time::get_time_seconds_since_epoch() + get_time_offset();
    trace!(adjusted_epoch = t, "get_adjusted_time");
    t
}

pub const BITCOIN_TIMEDATA_MAX_SAMPLES: usize = 200;

lazy_static! {
    static ref TIME_OFFSET: Mutex<i64> = Mutex::new(0);
}
