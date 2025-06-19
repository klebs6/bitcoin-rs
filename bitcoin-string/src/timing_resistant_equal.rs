crate::ix!();

/**
  | Timing-attack-resistant comparison.
  | 
  | Takes time proportional to length of
  | first argument.
  |
  */
pub fn timing_resistant_equal<T>(a: &T, b: &T) -> bool {

    todo!();
        /*
            if (b.size() == 0) return a.size() == 0;
        size_t accumulator = a.size() ^ b.size();
        for (size_t i = 0; i < a.size(); i++)
            accumulator |= a[i] ^ b[i%b.size()];
        return accumulator == 0;
        */
}
