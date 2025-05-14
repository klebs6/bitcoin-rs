// ---------------- [ File: bitcoin-fuzz/src/fuzz_fuzzeddataprovider.rs ]
/*!
  |===- FuzzedDataProvider.h - Utility header for fuzz targets ---*- C++ -* ===//
  |
  | Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
  | See https://llvm.org/LICENSE.txt for license information.
  | SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
  |
  |===----------------------------------------------------------------------===//
  | A single header library providing an utility class to break up an array of
  | bytes. Whenever run on the same input, provides the same output, as long as
  | its methods are called in the same order, with the same arguments.
  |===----------------------------------------------------------------------===//
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/FuzzedDataProvider.h]

/**
  | In addition to the comments below, the
  | API is also briefly documented at 
  | https://github.com/google/fuzzing/blob/master/docs/split-inputs.md#fuzzed-data-provider
  |
  */
pub struct FuzzedDataProvider {
    data_ptr:        *const u8,
    remaining_bytes: usize,
}

impl FuzzedDataProvider {

    /**
      | |data| is an array of length |size| that the
      | FuzzedDataProvider wraps to provide more
      | granular access. |data| must outlive the
      | FuzzedDataProvider.
      */
    pub fn new(
        data: *const u8,
        size: usize) -> Self {
    
        todo!();
        /*
        : data_ptr(data),
        : remaining_bytes(size),
        */
    }

    /*
      | See the implementation below (after
      | the class definition) for more verbose
      | comments for each of the methods.
      |
      */

    /**
      | Reports the remaining bytes available
      | for fuzzed input.
      |
      */
    pub fn remaining_bytes(&mut self) -> usize {
        
        todo!();
        /*
            return remaining_bytes_;
        */
    }
    
    /**
      | Methods returning std::vector of bytes. These
      | are the most popular choice when splitting
      | fuzzing input into pieces, as every piece is
      | put into a separate buffer (i.e. ASan would
      | catch any under-/overflow) and the memory
      | will be released automatically.
      |
      =============================
      |
      | Returns a std::vector containing |num_bytes| of
      | input data. If fewer than |num_bytes| of data
      | remain, returns a shorter std::vector
      | containing all of the data that's left. Can be
      | used with any byte sized type, such as char,
      | unsigned char, uint8_t, etc.
      */
    pub fn consume_bytes<T>(&mut self, num_bytes: usize) -> Vec<T> {
    
        todo!();
        /*
            num_bytes = std::min(num_bytes, remaining_bytes_);
      return ConsumeBytes<T>(num_bytes, num_bytes);
        */
    }

    /**
      | Similar to |ConsumeBytes|, but also appends the
      | terminator value at the end of the resulting
      | vector. Useful, when a mutable null-terminated
      | C-string is needed, for example. But that is
      | a rare case. Better avoid it, if possible, and
      | prefer using |ConsumeBytes| or
      | |ConsumeBytesAsString| methods.
      */
    pub fn consume_bytes_with_terminator<T: num::Zero>(&mut self, 
        num_bytes:  usize,
        terminator: Option<T>) -> Vec<T> {

        let terminator: T = terminator.unwrap_or(T::zero());
    
        todo!();
        /*
            num_bytes = std::min(num_bytes, remaining_bytes_);
      std::vector<T> result = ConsumeBytes<T>(num_bytes + 1, num_bytes);
      result.back() = terminator;
      return result;
        */
    }

    /**
      | Returns a std::vector containing all
      | remaining bytes of the input data.
      |
      */
    pub fn consume_remaining_bytes<T>(&mut self) -> Vec<T> {
    
        todo!();
        /*
            return ConsumeBytes<T>(remaining_bytes_);
        */
    }

    /**
      | Methods returning strings. Use only when you
      | need a std::string or a null terminated
      | C-string. Otherwise, prefer the methods
      | returning std::vector.
      |
      ==================================
      |
      | Returns a std::string containing |num_bytes| of
      | input data. Using this and |.c_str()| on the
      | resulting string is the best way to get an
      | immutable null-terminated C string. If fewer
      | than |num_bytes| of data remain, returns
      | a shorter std::string containing all of the
      | data that's left.
      */
    #[inline] pub fn consume_bytes_as_string(&mut self, num_bytes: usize) -> String {
        
        todo!();
        /*
            const_assert(sizeof(std::string::value_type) == sizeof(uint8_t),
                    "ConsumeBytesAsString cannot convert the data to a string.");

      num_bytes = std::min(num_bytes, remaining_bytes_);
      std::string result(
          reinterpret_cast<const std::string::value_type *>(data_ptr_), num_bytes);
      Advance(num_bytes);
      return result;
        */
    }

    /**
      | Returns a std::string of length from 0 to
      | |max_length|. When it runs out of input data,
      | returns what remains of the input. Designed to
      | be more stable with respect to a fuzzer
      | inserting characters than just picking a random
      | length and then consuming that many bytes with
      | |ConsumeBytes|.
      */
    #[inline] pub fn consume_random_length_string_with_maxlen(&mut self, max_length: usize) -> String {
        
        todo!();
        /*
            // Reads bytes from the start of |data_ptr_|. Maps "\\" to "\", and maps "\"
      // followed by anything else to the end of the string. As a result of this
      // logic, a fuzzer can insert characters into the string, and the string
      // will be lengthened to include those new characters, resulting in a more
      // stable fuzzer than picking the length of a string independently from
      // picking its contents.
      std::string result;

      // Reserve the anticipated capaticity to prevent several reallocations.
      result.reserve(std::min(max_length, remaining_bytes_));
      for (size_t i = 0; i < max_length && remaining_bytes_ != 0; ++i) {
        char next = ConvertUnsignedToSigned<char>(data_ptr_[0]);
        Advance(1);
        if (next == '\\' && remaining_bytes_ != 0) {
          next = ConvertUnsignedToSigned<char>(data_ptr_[0]);
          Advance(1);
          if (next != '\\')
            break;
        }
        result += next;
      }

      result.shrink_to_fit();
      return result;
        */
    }

    /**
      | Returns a std::string of length from
      | 0 to remaining_bytes_|.
      |
      */
    #[inline] pub fn consume_random_length_string(&mut self) -> String {
        
        todo!();
        /*
            return ConsumeRandomLengthString(remaining_bytes_);
        */
    }

    /**
      | Returns a std::string containing all remaining
      | bytes of the input data.
      |
      | Prefer using |ConsumeRemainingBytes| unless you
      | actually need a std::string object.
      */
    #[inline] pub fn consume_remaining_bytes_as_string(&mut self) -> String {
        
        todo!();
        /*
            return ConsumeBytesAsString(remaining_bytes_);
        */
    }

    /**
      | Methods returning integer values.
      |
      =============================
      | Returns a number in the range [Type's min,
      | Type's max]. The value might not be uniformly
      | distributed in the given range. If there's no
      | input data left, always returns |min|.
      */
    pub fn consume_integral<T>(&mut self) -> T {
    
        todo!();
        /*
            return ConsumeIntegralInRange(std::numeric_limits<T>::min(),
                                    std::numeric_limits<T>::max());
        */
    }

    /**
      | Returns a number in the range [min, max] by
      | consuming bytes from the input data. The value
      | might not be uniformly distributed in the given
      | range. If there's no input data left, always
      | returns |min|. |min| must be less than or equal
      | to |max|.
      */
    pub fn consume_integral_in_range<T>(&mut self, min: T, max: T) -> T {
    
        todo!();
        /*
            const_assert(std::is_integral<T>::value, "An integral type is required.");
      const_assert(sizeof(T) <= sizeof(uint64_t), "Unsupported integral type.");

      if (min > max)
        abort();

      // Use the biggest type possible to hold the range and the result.
      uint64_t range = static_cast<uint64_t>(max) - min;
      uint64_t result = 0;
      size_t offset = 0;

      while (offset < sizeof(T) * CHAR_BIT && (range >> offset) > 0 &&
             remaining_bytes_ != 0) {
        // Pull bytes off the end of the seed data. Experimentally, this seems to
        // allow the fuzzer to more easily explore the input space. This makes
        // sense, since it works by modifying inputs that caused new code to run,
        // and this data is often used to encode length of data read by
        // |ConsumeBytes|. Separating out read lengths makes it easier modify the
        // contents of the data that is actually read.
        --remaining_bytes_;
        result = (result << CHAR_BIT) | data_ptr_[remaining_bytes_];
        offset += CHAR_BIT;
      }

      // Avoid division by 0, in case |range + 1| results in overflow.
      if (range != std::numeric_limits<decltype(range)>::max())
        result = result % (range + 1);

      return static_cast<T>(min + result);
        */
    }

    /**
      | Methods returning floating point values.
      |
      =============================
      | Returns a floating point value in the range
      | [Type's lowest, Type's max] by consuming bytes
      | from the input data. If there's no input data
      | left, always returns approximately 0.
      */
    pub fn consume_floating_point<T>(&mut self) -> T {
    
        todo!();
        /*
            return ConsumeFloatingPointInRange<T>(std::numeric_limits<T>::lowest(),
                                            std::numeric_limits<T>::max());
        */
    }

    /**
      | Returns a floating point value in the given
      | range by consuming bytes from the input
      | data. If there's no input data left, returns
      | |min|. Note that |min| must be less than or
      | equal to |max|.
      */
    pub fn consume_floating_point_in_range<T>(&mut self, min: T, max: T) -> T {
    
        todo!();
        /*
            if (min > max)
        abort();

      T range = .0;
      T result = min;
      constexpr T zero(.0);
      if (max > zero && min < zero && max > min + std::numeric_limits<T>::max()) {
        // The diff |max - min| would overflow the given floating point type. Use
        // the half of the diff as the range and consume a bool to decide whether
        // the result is in the first of the second part of the diff.
        range = (max / 2.0) - (min / 2.0);
        if (ConsumeBool()) {
          result += range;
        }
      } else {
        range = max - min;
      }

      return result + range * ConsumeProbability<T>();
        */
    }

    /**
      | Returns a floating point number in the
      | range [0.0, 1.0]. If there's no input
      | data left, always returns 0.
      |
      | 0 <= return value <= 1.
      |
      */
    pub fn consume_probability<T>(&mut self) -> T {
    
        todo!();
        /*
            const_assert(std::is_floating_point<T>::value,
                    "A floating point type is required.");

      // Use different integral types for different floating point types in order
      // to provide better density of the resulting values.
      using IntegralType =
          typename std::conditional<(sizeof(T) <= sizeof(uint32_t)), uint32_t,
                                    uint64_t>::type;

      T result = static_cast<T>(ConsumeIntegral<IntegralType>());
      result /= static_cast<T>(std::numeric_limits<IntegralType>::max());
      return result;
        */
    }

    /**
      | Reads one byte and returns a bool, or
      | false when no data remains.
      |
      */
    #[inline] pub fn consume_bool(&mut self) -> bool {
        
        todo!();
        /*
            return 1 & ConsumeIntegral<uint8_t>();
        */
    }

    /**
      | Returns a value chosen from the given
      | enum.
      |
      =====================================
      | Returns an enum value. The enum must start at
      | 0 and be contiguous. It must also contain
      | |kMaxValue| aliased to its largest (inclusive)
      | value. Such as: enum class Foo { SomeValue,
      | OtherValue, kMaxValue = OtherValue };
      */
    pub fn consume_enum<T>(&mut self) -> T {
    
        todo!();
        /*
            const_assert(std::is_enum<T>::value, "|T| must be an enum type.");
      return static_cast<T>(
          ConsumeIntegralInRange<uint32_t>(0, static_cast<uint32_t>(T::kMaxValue)));
        */
    }

    /**
      | Returns a copy of the value selected
      | from the given fixed-size |array|.
      |
      */
    pub fn pick_value_in_array<T, const size: usize>(&mut self, array: &[T; size]) -> T {
    
        todo!();
        /*
            const_assert(size > 0, "The array must be non empty.");
      return array[ConsumeIntegralInRange<size_t>(0, size - 1)];
        */
    }
    
    pub fn pick_value_in_array_with_initlist<T>(&mut self, list: InitializerList<T>) -> T {
    
        todo!();
        /*
            // TODO(Dor1s): switch to const_assert once C++14 is allowed.
      if (!list.size())
        abort();

      return *(list.begin() + ConsumeIntegralInRange<size_t>(0, list.size() - 1));
        */
    }

    /**
      | Writes data to the given destination
      | and returns number of bytes written.
      |
      =======================================
      | Writes |num_bytes| of input data to the given
      | destination pointer. If there is not enough
      | data left, writes all remaining bytes. Return
      | value is the number of bytes written.
      |
      | In general, it's better to avoid using this
      | function, but it may be useful in cases when
      | it's necessary to fill a certain buffer or
      | object with fuzzing data.
      */
    #[inline] pub fn consume_data(&mut self, 
        destination: *mut c_void,
        num_bytes:   usize) -> usize {
        
        todo!();
        /*
            num_bytes = std::min(num_bytes, remaining_bytes_);
      CopyAndAdvance(destination, num_bytes);
      return num_bytes;
        */
    }

    /**
      | Private methods.
      |
      */
    #[inline] pub fn copy_and_advance(&mut self, 
        destination: *mut c_void,
        num_bytes:   usize)  {
        
        todo!();
        /*
            std::memcpy(destination, data_ptr_, num_bytes);
      Advance(num_bytes);
        */
    }
    
    #[inline] pub fn advance(&mut self, num_bytes: usize)  {
        
        todo!();
        /*
            if (num_bytes > remaining_bytes_)
        abort();

      data_ptr_ += num_bytes;
      remaining_bytes_ -= num_bytes;
        */
    }
    
    pub fn consume_bytes_with_size<T>(&mut self, 
        size:      usize,
        num_bytes: usize) -> Vec<T> {
    
        todo!();
        /*
            const_assert(sizeof(T) == sizeof(uint8_t), "Incompatible data type.");

      // The point of using the size-based constructor below is to increase the
      // odds of having a vector object with capacity being equal to the length.
      // That part is always implementation specific, but at least both libc++ and
      // libstdc++ allocate the requested number of bytes in that constructor,
      // which seems to be a natural choice for other implementations as well.
      // To increase the odds even more, we also call |shrink_to_fit| below.
      std::vector<T> result(size);
      if (size == 0) {
        if (num_bytes != 0)
          abort();
        return result;
      }

      CopyAndAdvance(result.data(), num_bytes);

      // Even though |shrink_to_fit| is also implementation specific, we expect it
      // to provide an additional assurance in case vector's constructor allocated
      // a buffer which is larger than the actual amount of data we put inside it.
      result.shrink_to_fit();
      return result;
        */
    }
   
    pub fn convert_unsigned_to_signed<TS, TU>(&mut self, value: TU) -> TS {
    
        todo!();
        /*
            const_assert(sizeof(TS) == sizeof(TU), "Incompatible data types.");
      const_assert(!std::numeric_limits<TU>::is_signed,
                    "Source type must be unsigned.");

      // TODO(Dor1s): change to `if constexpr` once C++17 becomes mainstream.
      if (std::numeric_limits<TS>::is_modulo)
        return static_cast<TS>(value);

      // Avoid using implementation-defined unsigned to signed conversions.
      // To learn more, see https://stackoverflow.com/questions/13150449.
      if (value <= std::numeric_limits<TS>::max()) {
        return static_cast<TS>(value);
      } else {
        constexpr auto TS_min = std::numeric_limits<TS>::min();
        return TS_min + static_cast<char>(value - TS_min);
      }
        */
    }
}
