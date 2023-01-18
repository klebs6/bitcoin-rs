crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/histogram.h]

pub struct Histogram {
    min:         f64,
    max:         f64,
    num:         f64,
    sum:         f64,
    sum_squares: f64,
    buckets:     [f64; HISTOGRAM_NUM_BUCKETS],
}

pub const HISTOGRAM_NUM_BUCKETS: usize = 154;

pub const BUCKET_LIMIT: [f64; HISTOGRAM_NUM_BUCKETS] = [
    1.0,
    2.0,
    3.0,
    4.0,
    5.0,
    6.0,
    7.0,
    8.0,
    9.0,
    10.0,
    12.0,
    14.0,
    16.0,
    18.0,
    20.0,
    25.0,
    30.0,
    35.0,
    40.0,
    45.0,
    50.0,
    60.0,
    70.0,
    80.0,
    90.0,
    100.0,
    120.0,
    140.0,
    160.0,
    180.0,
    200.0,
    250.0,
    300.0,
    350.0,
    400.0,
    450.0,
    500.0,
    600.0,
    700.0,
    800.0,
    900.0,
    1000.0,
    1200.0,
    1400.0,
    1600.0,
    1800.0,
    2000.0,
    2500.0,
    3000.0,
    3500.0,
    4000.0,
    4500.0,
    5000.0,
    6000.0,
    7000.0,
    8000.0,
    9000.0,
    10000.0,
    12000.0,
    14000.0,
    16000.0,
    18000.0,
    20000.0,
    25000.0,
    30000.0,
    35000.0,
    40000.0,
    45000.0,
    50000.0,
    60000.0,
    70000.0,
    80000.0,
    90000.0,
    100000.0,
    120000.0,
    140000.0,
    160000.0,
    180000.0,
    200000.0,
    250000.0,
    300000.0,
    350000.0,
    400000.0,
    450000.0,
    500000.0,
    600000.0,
    700000.0,
    800000.0,
    900000.0,
    1000000.0,
    1200000.0,
    1400000.0,
    1600000.0,
    1800000.0,
    2000000.0,
    2500000.0,
    3000000.0,
    3500000.0,
    4000000.0,
    4500000.0,
    5000000.0,
    6000000.0,
    7000000.0,
    8000000.0,
    9000000.0,
    10000000.0,
    12000000.0,
    14000000.0,
    16000000.0,
    18000000.0,
    20000000.0,
    25000000.0,
    30000000.0,
    35000000.0,
    40000000.0,
    45000000.0,
    50000000.0,
    60000000.0,
    70000000.0,
    80000000.0,
    90000000.0,
    100000000.0,
    120000000.0,
    140000000.0,
    160000000.0,
    180000000.0,
    200000000.0,
    250000000.0,
    300000000.0,
    350000000.0,
    400000000.0,
    450000000.0,
    500000000.0,
    600000000.0,
    700000000.0,
    800000000.0,
    900000000.0,
    1000000000.0,
    1200000000.0,
    1400000000.0,
    1600000000.0,
    1800000000.0,
    2000000000.0,
    2500000000.0,
    3000000000.0,
    3500000000.0,
    4000000000.0,
    4500000000.0,
    5000000000.0,
    6000000000.0,
    7000000000.0,
    8000000000.0,
    9000000000.0,
    1.0e200,
];

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/histogram.cc]
impl Histogram {
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            min_ = kBucketLimit[kNumBuckets - 1];
      max_ = 0;
      num_ = 0;
      sum_ = 0;
      sum_squares_ = 0;
      for (int i = 0; i < kNumBuckets; i++) {
        buckets_[i] = 0;
      }
        */
    }
    
    pub fn add(&mut self, value: f64)  {
        
        todo!();
        /*
            // Linear search is fast enough for our usage in db_bench
      int b = 0;
      while (b < kNumBuckets - 1 && kBucketLimit[b] <= value) {
        b++;
      }
      buckets_[b] += 1.0;
      if (min_ > value) min_ = value;
      if (max_ < value) max_ = value;
      num_++;
      sum_ += value;
      sum_squares_ += (value * value);
        */
    }
    
    pub fn merge(&mut self, other: &Histogram)  {
        
        todo!();
        /*
            if (other.min_ < min_) min_ = other.min_;
      if (other.max_ > max_) max_ = other.max_;
      num_ += other.num_;
      sum_ += other.sum_;
      sum_squares_ += other.sum_squares_;
      for (int b = 0; b < kNumBuckets; b++) {
        buckets_[b] += other.buckets_[b];
      }
        */
    }
    
    pub fn median(&self) -> f64 {
        
        todo!();
        /*
            return Percentile(50.0);
        */
    }
    
    pub fn percentile(&self, p: f64) -> f64 {
        
        todo!();
        /*
            double threshold = num_ * (p / 100.0);
      double sum = 0;
      for (int b = 0; b < kNumBuckets; b++) {
        sum += buckets_[b];
        if (sum >= threshold) {
          // Scale linearly within this bucket
          double left_point = (b == 0) ? 0 : kBucketLimit[b - 1];
          double right_point = kBucketLimit[b];
          double left_sum = sum - buckets_[b];
          double right_sum = sum;
          double pos = (threshold - left_sum) / (right_sum - left_sum);
          double r = left_point + (right_point - left_point) * pos;
          if (r < min_) r = min_;
          if (r > max_) r = max_;
          return r;
        }
      }
      return max_;
        */
    }
    
    pub fn average(&self) -> f64 {
        
        todo!();
        /*
            if (num_ == 0.0) return 0;
      return sum_ / num_;
        */
    }
    
    pub fn standard_deviation(&self) -> f64 {
        
        todo!();
        /*
            if (num_ == 0.0) return 0;
      double variance = (sum_squares_ * num_ - sum_ * sum_) / (num_ * num_);
      return sqrt(variance);
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            std::string r;
      char buf[200];
      snprintf(buf, sizeof(buf), "Count: %.0f  Average: %.4f  StdDev: %.2f\n", num_,
               Average(), StandardDeviation());
      r.append(buf);
      snprintf(buf, sizeof(buf), "Min: %.4f  Median: %.4f  Max: %.4f\n",
               (num_ == 0.0 ? 0.0 : min_), Median(), max_);
      r.append(buf);
      r.append("------------------------------------------------------\n");
      const double mult = 100.0 / num_;
      double sum = 0;
      for (int b = 0; b < kNumBuckets; b++) {
        if (buckets_[b] <= 0.0) continue;
        sum += buckets_[b];
        snprintf(buf, sizeof(buf), "[ %7.0f, %7.0f ) %7.0f %7.3f%% %7.3f%% ",
                 ((b == 0) ? 0.0 : kBucketLimit[b - 1]),  // left
                 kBucketLimit[b],                         // right
                 buckets_[b],                             // count
                 mult * buckets_[b],                      // percentage
                 mult * sum);                             // cumulative percentage
        r.append(buf);

        // Add hash marks based on percentage; 20 marks for 100%.
        int marks = static_cast<int>(20 * (buckets_[b] / num_) + 0.5);
        r.append(marks, '#');
        r.push_back('\n');
      }
      return r;
        */
    }
}
