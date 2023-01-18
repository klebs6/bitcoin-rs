crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/univalue/lib/univalue_utffilter.h]

/**
  | Filter that generates and validates
  | UTF-8, as well as collates UTF-16 surrogate
  | pairs as specified in RFC4627.
  |
  */
pub struct JSONUTF8StringFilter {

    str_:      Rc<RefCell<String>>,
    is_valid:  bool,

    /**
      | Current UTF-8 decoding state
      |
      */
    codepoint: u32,

    /**
      | Top bit to be filled in for next UTF-8
      | byte, or 0
      |
      */
    state:     i32,

    /*
      | Keep track of the following state to handle
      | the following section of RFC4627:
      |
      |    To escape an extended character that is
      |    not in the Basic Multilingual Plane, the
      |    character is represented as
      |    a twelve-character sequence, encoding
      |    the UTF-16 surrogate pair.  So, for
      |    example, a string containing only the
      |    G clef character (U+1D11E) may be
      |    represented as "\uD834\uDD1E".
      |
      |  Two subsequent \u.... may have to be
      |  replaced with one actual codepoint.
      */

    /**
      | First half of open UTF-16 surrogate
      | pair, or 0
      |
      */
    surpair: u32,
}

impl JSONUTF8StringFilter {
    
    pub fn new(s: &mut String) -> Self {
    
        todo!();
        /*
            :
            str(s), is_valid(true), codepoint(0), state(0), surpair(0)
        */
    }

    /**
      | Write single 8-bit char (may be part
      | of UTF-8 sequence)
      |
      */
    pub fn push_back(&mut self, ch: u8)  {
        
        todo!();
        /*
            if (state == 0) {
                if (ch < 0x80) // 7-bit ASCII, fast direct pass-through
                    str.push_back(ch);
                else if (ch < 0xc0) // Mid-sequence character, invalid in this state
                    is_valid = false;
                else if (ch < 0xe0) { // Start of 2-byte sequence
                    codepoint = (ch & 0x1f) << 6;
                    state = 6;
                } else if (ch < 0xf0) { // Start of 3-byte sequence
                    codepoint = (ch & 0x0f) << 12;
                    state = 12;
                } else if (ch < 0xf8) { // Start of 4-byte sequence
                    codepoint = (ch & 0x07) << 18;
                    state = 18;
                } else // Reserved, invalid
                    is_valid = false;
            } else {
                if ((ch & 0xc0) != 0x80) // Not a continuation, invalid
                    is_valid = false;
                state -= 6;
                codepoint |= (ch & 0x3f) << state;
                if (state == 0)
                    push_back_u(codepoint);
            }
        */
    }

    /**
      | Write codepoint directly, possibly
      | collating surrogate pairs
      |
      */
    pub fn push_back_u(&mut self, codepoint: u32)  {
        
        todo!();
        /*
            if (state) // Only accept full codepoints in open state
                is_valid = false;
            if (codepoint_ >= 0xD800 && codepoint_ < 0xDC00) { // First half of surrogate pair
                if (surpair) // Two subsequent surrogate pair openers - fail
                    is_valid = false;
                else
                    surpair = codepoint_;
            } else if (codepoint_ >= 0xDC00 && codepoint_ < 0xE000) { // Second half of surrogate pair
                if (surpair) { // Open surrogate pair, expect second half
                    // Compute code point from UTF-16 surrogate pair
                    append_codepoint(0x10000 | ((surpair - 0xD800)<<10) | (codepoint_ - 0xDC00));
                    surpair = 0;
                } else // Second half doesn't follow a first half - fail
                    is_valid = false;
            } else {
                if (surpair) // First half of surrogate pair not followed by second - fail
                    is_valid = false;
                else
                    append_codepoint(codepoint_);
            }
        */
    }

    /**
      | Check that we're in a state where the
      | string can be ended No open sequences,
      | no open surrogate pairs, etc
      |
      */
    pub fn finalize(&mut self) -> bool {
        
        todo!();
        /*
            if (state || surpair)
                is_valid = false;
            return is_valid;
        */
    }
    
    pub fn append_codepoint(&mut self, codepoint: u32)  {
        
        todo!();
        /*
            if (codepoint_ <= 0x7f)
                str.push_back((char)codepoint_);
            else if (codepoint_ <= 0x7FF) {
                str.push_back((char)(0xC0 | (codepoint_ >> 6)));
                str.push_back((char)(0x80 | (codepoint_ & 0x3F)));
            } else if (codepoint_ <= 0xFFFF) {
                str.push_back((char)(0xE0 | (codepoint_ >> 12)));
                str.push_back((char)(0x80 | ((codepoint_ >> 6) & 0x3F)));
                str.push_back((char)(0x80 | (codepoint_ & 0x3F)));
            } else if (codepoint_ <= 0x1FFFFF) {
                str.push_back((char)(0xF0 | (codepoint_ >> 18)));
                str.push_back((char)(0x80 | ((codepoint_ >> 12) & 0x3F)));
                str.push_back((char)(0x80 | ((codepoint_ >> 6) & 0x3F)));
                str.push_back((char)(0x80 | (codepoint_ & 0x3F)));
            }
        */
    }
}
