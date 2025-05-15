// ---------------- [ File: bitcoin-time/src/timedata.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/timedata.h]

pub const DEFAULT_MAX_TIME_ADJUSTMENT: i64 = 70 * 60;

/**
  | Median filter over a stream of values.
  | 
  | Returns the median of the last N numbers
  |
  */
pub struct MedianFilter<T> {
    values: Vec<T>,
    sorted: Vec<T>,
    n_size: u32,
}

impl<T> MedianFilter<T> {
    
    pub fn new(
        size:          u32,
        initial_value: T) -> Self {
    
        todo!();
        /*
        : n_size(_size),

            vValues.reserve(_size);
            vValues.push_back(initial_value);
            vSorted = vValues;
        */
    }
    
    pub fn input(&mut self, value: T)  {
        
        todo!();
        /*
            if (vValues.size() == nSize) {
                vValues.erase(vValues.begin());
            }
            vValues.push_back(value);

            vSorted.resize(vValues.size());
            std::copy(vValues.begin(), vValues.end(), vSorted.begin());
            std::sort(vSorted.begin(), vSorted.end());
        */
    }
    
    pub fn median(&self) -> T {
        
        todo!();
        /*
            int vSortedSize = vSorted.size();
            assert(vSortedSize > 0);
            if (vSortedSize & 1) // Odd number of elements
            {
                return vSorted[vSortedSize / 2];
            } else // Even number of elements
            {
                return (vSorted[vSortedSize / 2 - 1] + vSorted[vSortedSize / 2]) / 2;
            }
        */
    }
    
    pub fn size(&self) -> i32 {
        
        todo!();
        /*
            return vValues.size();
        */
    }
    
    pub fn sorted(&self) -> Vec<T> {
        
        todo!();
        /*
            return vSorted;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/timedata.cpp]

lazy_static!{
    /*
    static Mutex g_timeoffset_mutex;
    static int64_t nTimeOffset GUARDED_BY(g_timeoffset_mutex) = 0;
    */
}

/*
  | Functions to keep track of adjusted
  | P2P time
  |
  */

/**
  | "Never go to sea with two chronometers;
  | take one or three."
  | 
  | Our three time sources are:
  | 
  | - System clock
  | 
  | - Median of other nodes clocks
  | 
  | - The user (asking the user to fix the
  | system clock if the first two disagree)
  |
  */
pub fn get_time_offset() -> i64 {
    
    todo!();
        /*
            LOCK(g_timeoffset_mutex);
        return nTimeOffset;
        */
}

#[inline] pub fn get_adjusted_datetime() -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(get_adjusted_time()).unwrap()
}

pub fn get_adjusted_time() -> i64 {
    
    todo!();
        /*
            return GetTime() + GetTimeOffset();
        */
}

pub const BITCOIN_TIMEDATA_MAX_SAMPLES: usize = 200;
