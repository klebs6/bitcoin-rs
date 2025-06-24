// ---------------- [ File: bitcoin-univalue/src/utffilter.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/univalue/lib/univalue_utffilter.h]

use std::{cell::RefCell, rc::Rc};

/// Filter that generates and validates UTF-8, as well as collates UTF-16
/// surrogate pairs as specified in RFC4627.
/// 
/// Validates UTF‑8 byte‑by‑byte and, when the input comes from JSON `\uXXXX`
/// escapes, additionally collates UTF‑16 surrogate pairs into a single scalar
/// value.
///
/// Keep track of the following state to handle the following section of
/// RFC4627:
/// 
///    To escape an extended character that is not in the Basic Multilingual
///    Plane, the character is represented as a twelve-character sequence,
///    encoding the UTF-16 surrogate pair.  
///
///    So, for example, a string containing only the G clef character (U+1D11E)
///    may be represented as "\uD834\uDD1E".
/// 
///  Two subsequent \u.... may have to be replaced with one actual codepoint.
///
pub struct JSONUTF8StringFilter {
    str_:      Rc<RefCell<String>>,
    is_valid:  bool,

    /// Current UTF-8 decoding state
    ///
    codepoint: u32, // assembling UCS‑4 value

    /// Top bit to be filled in for next UTF-8
    /// byte, or 0
    /// 
    ///
    state:     u8,  // remaining continuation bits (0 / 6 / 12 / 18)
   
    /// First half of open UTF-16 surrogate
    /// pair, or 0
    ///
    surpair:   u32, // open UTF‑16 surrogate, else 0
}

impl JSONUTF8StringFilter {

    /// Create a new filter writing into *target*.
    ///
    ///
    pub fn new(target: Rc<RefCell<String>>) -> Self {
        Self {
            str_: target,
            is_valid: true,
            codepoint: 0,
            state: 0,
            surpair: 0,
        }
    }

    /// Feed a single input byte.
    ///
    /// Write single 8-bit char (may be part of UTF-8 sequence)
    #[instrument(level = "trace", skip(self))]
    pub fn push_back(&mut self, ch: u8) {
        if self.state == 0 {
            match ch {
                0x00..=0x7F => self.str_.borrow_mut().push(ch as char),
                0xC0..=0xDF => {
                    self.codepoint = ((ch & 0x1F) as u32) << 6;
                    self.state = 6;
                }
                0xE0..=0xEF => {
                    self.codepoint = ((ch & 0x0F) as u32) << 12;
                    self.state = 12;
                }
                0xF0..=0xF7 => {
                    self.codepoint = ((ch & 0x07) as u32) << 18;
                    self.state = 18;
                }
                _ => self.is_valid = false, // stray continuation or reserved
            }
        } else {
            if (ch & 0xC0) != 0x80 {
                self.is_valid = false;
            }
            self.state -= 6;
            self.codepoint |= ((ch & 0x3F) as u32) << self.state;
            if self.state == 0 {
                self.push_back_u(self.codepoint);
            }
        }
    }

    /// Inject a full scalar value (used when parsing `\uXXXX`) and deal with
    /// UTF‑16 surrogate bookkeeping.
    ///
    /// Write codepoint directly, possibly collating surrogate pairs
    ///
    #[instrument(level = "trace", skip(self))]
    pub fn push_back_u(&mut self, cp: u32) {
        if self.state != 0 {
            self.is_valid = false;
            return;
        }

        match cp {
            0xD800..=0xDBFF => {
                if self.surpair != 0 {
                    self.is_valid = false; // two high surrogates in a row
                } else {
                    self.surpair = cp;
                }
            }
            0xDC00..=0xDFFF => {
                if self.surpair != 0 {
                    let full = 0x10000 | ((self.surpair - 0xD800) << 10) | (cp - 0xDC00);
                    self.append_codepoint(full);
                    self.surpair = 0;
                } else {
                    self.is_valid = false; // low surrogate without opener
                }
            }
            _ => {
                if self.surpair != 0 {
                    self.is_valid = false; // opener not followed by closer
                } else {
                    self.append_codepoint(cp);
                }
            }
        }
    }

    /// Finalise the stream – no open sequences or surrogate pairs allowed.
    ///
    /// Check that we're in a state where the string can be ended No open
    /// sequences, no open surrogate pairs, etc
    ///
    #[instrument(level = "trace", skip(self))]
    pub fn finalize(&mut self) -> bool {
        if self.state != 0 || self.surpair != 0 {
            self.is_valid = false;
        }
        self.is_valid
    }

    /// Emit *cp* as UTF‑8 into the target string.
    fn append_codepoint(&mut self, cp: u32) {
        if let Some(ch) = char::from_u32(cp) {
            self.str_.borrow_mut().push(ch);
        } else {
            self.is_valid = false;
        }
    }
}

#[cfg(test)]
mod utffilter_spec {
    use super::*;

    fn run_bytes(bytes: &[u8]) -> (bool, String) {
        let buf = Rc::new(RefCell::new(String::new()));
        let mut f = JSONUTF8StringFilter::new(buf.clone());
        for &b in bytes {
            f.push_back(b);
        }
        let ok = f.finalize();
        let x = (ok, buf.borrow().clone());
        x
    }

    #[traced_test]
    fn accepts_plain_ascii() {
        let (ok, s) = run_bytes(b"hello");
        assert!(ok);
        assert_eq!(s, "hello");
    }

    #[traced_test]
    fn accepts_multibyte_utf8() {
        // U+20AC (€) -> E2 82 AC
        let (ok, s) = run_bytes(&[0xE2, 0x82, 0xAC]);
        assert!(ok);
        assert_eq!(s, "€");
    }

    #[traced_test]
    fn surrogate_pair_collation() {
        let buf = Rc::new(RefCell::new(String::new()));
        let mut f = JSONUTF8StringFilter::new(buf.clone());
        f.push_back_u(0xD834); // high surrogate
        f.push_back_u(0xDD1E); // low surrogate → U+1D11E
        assert!(f.finalize());
        assert_eq!(buf.borrow().as_str(), "𝄞"); // musical G‑clef
    }

    #[traced_test]
    fn detects_invalid_sequence() {
        // lone continuation byte
        let (ok, _) = run_bytes(&[0x80]);
        assert!(!ok);

        // high surrogate without low
        let buf = Rc::new(RefCell::new(String::new()));
        let mut f = JSONUTF8StringFilter::new(buf);
        f.push_back_u(0xD800);
        assert!(!f.finalize());
    }
}
