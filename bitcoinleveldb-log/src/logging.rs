/*!
  | Must not be included from any .h files
  | to avoid polluting the namespace with
  | macros.
  |
  */

crate::ix!();

/**
  | An interface for writing log messages.
  |
  */
pub trait Logger: Logv { }

pub trait Logv {

    /**
      | Write an entry to the log file with the
      | specified format.
      |
      */
    fn logv(&mut self, 
            format: *const u8,
            ap:     &[&str]);

}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/logging.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/logging.cc]

/**
  | Append a human-readable printout of
  | "num" to *str
  |
  */
pub fn append_number_to(
        str_: *mut String,
        num:  u64)  {
    
    todo!();
        /*
            char buf[30];
      snprintf(buf, sizeof(buf), "%llu", (unsigned long long)num);
      str->append(buf);
        */
}

/**
  | Append a human-readable printout of
  | "value" to *str.
  | 
  | Escapes any non-printable characters
  | found in "value".
  |
  */
pub fn append_escaped_string_to(
        str_:  *mut String,
        value: &Slice)  {
    
    todo!();
        /*
            for (size_t i = 0; i < value.size(); i++) {
        char c = value[i];
        if (c >= ' ' && c <= '~') {
          str->push_back(c);
        } else {
          char buf[10];
          snprintf(buf, sizeof(buf), "\\x%02x",
                   static_cast<unsigned int>(c) & 0xff);
          str->append(buf);
        }
      }
        */
}

/**
  | Return a human-readable printout of
  | "num"
  |
  */
pub fn number_to_string(num: u64) -> String {
    
    todo!();
        /*
            std::string r;
      AppendNumberTo(&r, num);
      return r;
        */
}

/**
  | Return a human-readable version of "value".
  |
  | Escapes any non-printable characters found in
  | "value".
  */
pub fn escape_string(value: &Slice) -> String {
    
    todo!();
        /*
            std::string r;
      AppendEscapedStringTo(&r, value);
      return r;
        */
}

/**
  | Parse a human-readable number from "*in" into
  | *value.  On success, advances "*in" past the
  | consumed number and sets "*val" to the numeric
  | value.  Otherwise, returns false and leaves *in
  | in an unspecified state.
  */
pub fn consume_decimal_number(
        in_: *mut Slice,
        val: *mut u64) -> bool {
    
    todo!();
        /*
            // Constants that will be optimized away.
      constexpr const uint64_t kMaxUint64 = std::numeric_limits<uint64_t>::max();
      constexpr const char kLastDigitOfMaxUint64 =
          '0' + static_cast<char>(kMaxUint64 % 10);

      uint64_t value = 0;

      // reinterpret_cast-ing from char* to uint8_t* to avoid signedness.
      const uint8_t* start = reinterpret_cast<const uint8_t*>(in->data());

      const uint8_t* end = start + in->size();
      const uint8_t* current = start;
      for (; current != end; ++current) {
        const uint8_t ch = *current;
        if (ch < '0' || ch > '9') break;

        // Overflow check.
        // kMaxUint64 / 10 is also constant and will be optimized away.
        if (value > kMaxUint64 / 10 ||
            (value == kMaxUint64 / 10 && ch > kLastDigitOfMaxUint64)) {
          return false;
        }

        value = (value * 10) + (ch - '0');
      }

      *val = value;
      const size_t digits_consumed = current - start;
      in->remove_prefix(digits_consumed);
      return digits_consumed != 0;
        */
}
