// ---------------- [ File: bitcoinleveldb-status/src/status.rs ]
/*!
  | A Status encapsulates the result of an
  | operation.  It may indicate success, or it may
  | indicate an error with an associated error
  | message.
  |
  | Multiple threads can invoke const methods on
  | a Status without external synchronization, but
  | if any of the threads may call a non-const
  | method, all threads accessing the same Status
  | must use external synchronization.
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/status.cc]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/status.h]

/// Status encapsulates the result of an operation. 
/// It may indicate success, or it may indicate an error with an associated message.
#[derive(Clone,Debug, Getters, Setters, Builder)]
#[getset(get = "pub", set = "pub")]
pub struct Status {
    /// If `None`, the status is `OK`. Otherwise:
    ///   - bytes[0..4] is a u32 (little-endian) giving the length of the message
    ///   - bytes[4] is the code (StatusCode, stored as u8)
    ///   - bytes[5..] is the message text
    #[builder(default = "None")]
    state: Option<Box<[u8]>>,
}

/// An enum matching the C++ codes:
///   - Ok = 0
///   - NotFound = 1
///   - Corruption = 2
///   - NotSupported = 3
///   - InvalidArgument = 4
///   - IOError = 5
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    Ok              = 0,
    NotFound        = 1,
    Corruption      = 2,
    NotSupported    = 3,
    InvalidArgument = 4,
    IOError         = 5,
}

impl Default for Status {

    /// Create a success status (OK).
    fn default() -> Self {
        info!("Creating a default (OK) Status");
        Self { state: None }
    }
}

impl Drop for Status {

    /// In Rust, dropping `Option<Box<[u8]>>` automatically frees the memory.
    /// We include a log for completeness (matching the C++ destructor).
    fn drop(&mut self) {
        if self.state.is_some() {
            debug!("Dropping non-OK Status with message/data");
        } else {
            debug!("Dropping OK Status");
        }
    }
}

impl Status {

    /// Move-constructor equivalent: take ownership of `rhs`'s state.
    /// In C++: `Status(Status&& rhs)`.
    pub fn new_from_other(mut rhs: Status) -> Self {
        info!("new_from_other (move) constructor invoked");
        let ret = Self { state: rhs.state.take() };
        // In C++, we'd set rhs.state_ = nullptr; which is analogous to `rhs.state.take()` in Rust.
        ret
    }

    /// Copy-constructor equivalent: allocate new state identical to `rhs`.
    /// In C++: `Status(const Status& rhs)`.
    pub fn new_from_other_copy(rhs: &Status) -> Self {
        info!("new_from_other_copy invoked");
        if let Some(ref boxed) = rhs.state {
            // Copy it
            let new_copy = Self::copy_state(boxed);
            Self { state: Some(new_copy) }
        } else {
            // OK
            Self { state: None }
        }
    }

    /// Return a success status.
    pub fn ok() -> Self {
        info!("Creating an OK status");
        Self { state: None }
    }

    /// Return error statuses of various kinds
    pub fn not_found(msg: &Slice, msg2: Option<&Slice>) -> Self {
        Self::new_error(StatusCode::NotFound, msg, msg2)
    }
    pub fn corruption(msg: &Slice, msg2: Option<&Slice>) -> Self {
        Self::new_error(StatusCode::Corruption, msg, msg2)
    }
    pub fn not_supported(msg: &Slice, msg2: Option<&Slice>) -> Self {
        Self::new_error(StatusCode::NotSupported, msg, msg2)
    }
    pub fn invalid_argument(msg: &Slice, msg2: Option<&Slice>) -> Self {
        Self::new_error(StatusCode::InvalidArgument, msg, msg2)
    }
    pub fn io_error(msg: &Slice, msg2: Option<&Slice>) -> Self {
        Self::new_error(StatusCode::IOError, msg, msg2)
    }

    /// Returns true iff the status indicates success.
    pub fn is_ok(&self) -> bool {
        trace!("Status::is_ok called");
        self.state.is_none()
    }

    /// Returns true iff the status indicates a NotFound error.
    pub fn is_not_found(&self) -> bool {
        self.code() == StatusCode::NotFound
    }

    /// Returns true iff the status indicates a Corruption error.
    pub fn is_corruption(&self) -> bool {
        self.code() == StatusCode::Corruption
    }

    /// Returns true iff the status indicates an IOError.
    pub fn is_io_error(&self) -> bool {
        self.code() == StatusCode::IOError
    }

    /// Returns true iff the status indicates a NotSupportedError.
    pub fn is_not_supported_error(&self) -> bool {
        self.code() == StatusCode::NotSupported
    }

    /// Returns true iff the status indicates an InvalidArgument.
    pub fn is_invalid_argument(&self) -> bool {
        self.code() == StatusCode::InvalidArgument
    }

    /// Extract the `StatusCode` from the stored state.
    pub fn code(&self) -> StatusCode {
        if let Some(ref st) = self.state {
            // st[4] is the code
            let code_raw = st[4];
            match code_raw {
                0 => StatusCode::Ok,
                1 => StatusCode::NotFound,
                2 => StatusCode::Corruption,
                3 => StatusCode::NotSupported,
                4 => StatusCode::InvalidArgument,
                5 => StatusCode::IOError,
                _ => {
                    warn!("Unknown code encountered: {}", code_raw);
                    StatusCode::IOError
                }
            }
        } else {
            StatusCode::Ok
        }
    }

    /// Copy-assignment operator equivalent: `status_a = status_b;`
    pub fn assign_from_other_copy(&mut self, rhs: &Status) -> &mut Status {
        info!("assign_from_other_copy invoked");
        if self.state.as_ref().map(|s| s.as_ptr()) != rhs.state.as_ref().map(|s| s.as_ptr()) {
            // They differ; free old state and copy
            self.state = if let Some(ref boxed) = rhs.state {
                Some(Self::copy_state(boxed))
            } else {
                None
            };
        }
        self
    }

    /// Move-assignment operator equivalent: `status_a = std::move(status_b);`
    pub fn assign_from_other_move(&mut self, rhs: &mut Status) -> &mut Status {
        info!("assign_from_other_move invoked");
        let old = self.state.take();
        debug!("Dropping old state in move-assign");
        drop(old); // drop any existing data
        self.state = rhs.state.take();
        self
    }

    /// Return a string representation of this status suitable for printing.
    /// Returns "OK" for success.
    pub fn to_string(&self) -> String {
        if self.is_ok() {
            return "OK".to_owned();
        }
        let code_part = match self.code() {
            StatusCode::Ok => "OK",
            StatusCode::NotFound => "NotFound: ",
            StatusCode::Corruption => "Corruption: ",
            StatusCode::NotSupported => "Not implemented: ",
            StatusCode::InvalidArgument => "Invalid argument: ",
            StatusCode::IOError => "IO error: ",
        };
        let mut result = String::from(code_part);

        // The stored message is after byte 5, with length stored in [0..4].
        if let Some(ref st) = self.state {
            use std::slice;
            // read length
            let mut length_bytes = [0u8; 4];
            length_bytes.copy_from_slice(&st[0..4]);
            let msg_len = u32::from_le_bytes(length_bytes) as usize;
            let msg_data = &st[5..(5 + msg_len)];
            let msg_str = String::from_utf8_lossy(msg_data);
            result.push_str(&msg_str);
        }
        result
    }

    // -------------------- Private / internal methods --------------------

    /// Build a new `Status` in error form with the given code and two optional slices.
    fn new_error(code: StatusCode, msg: &Slice, msg2: Option<&Slice>) -> Self {
        let binding = Slice::default();
        let msg2 = msg2.unwrap_or(&binding);
        // Compute the total size
        let len1 = msg.size();
        let len2 = msg2.size();
        // If len2 > 0, we'll add ": " to separate them
        let total_len = len1 + if *len2 > 0 { 2 + len2 } else { 0 };
        // We'll store [4 bytes: total_len, 1 byte: code, rest: message(s)]
        let mut data = vec![0u8; total_len + 5];
        // Fill first 4 bytes with the message length in little-endian
        let len_bytes = (total_len as u32).to_le_bytes();
        data[0..4].copy_from_slice(&len_bytes);
        // code in data[4]
        data[4] = code as u8;
        // copy msg data if non-empty
        if *len1 > 0 {
            unsafe {
                // We explicitly specify <u8> so the compiler matches `*const u8` -> `*mut u8`.
                std::ptr::copy_nonoverlapping::<u8>(
                    *msg.data() as *const u8,
                    data.as_mut_ptr().add(5),
                    *len1,
                );
            }
        }
        // if there's a second message, add ": "
        if *len2 > 0 {
            let idx = 5 + len1;
            data[idx] = b':';
            data[idx + 1] = b' ';
            unsafe {
                std::ptr::copy_nonoverlapping::<u8>(
                    *msg2.data() as *const u8,
                    data.as_mut_ptr().add(idx + 2),
                    *len2,
                );
            }
        }
        info!(
            "Creating an error status with code={:?}, length={} ({} + {})",
            code, total_len, len1, len2
        );
        Self {
            state: Some(data.into_boxed_slice()),
        }
    }

    /// Replicate the "copy" logic from C++: allocate new space and copy the entire buffer.
    fn copy_state(src: &Box<[u8]>) -> Box<[u8]> {
        info!("Copying status state");
        let mut clone_vec = vec![0u8; src.len()];
        clone_vec.copy_from_slice(src);
        clone_vec.into_boxed_slice()
    }
}

#[cfg(test)]
mod test_status_interface {
    use super::*;

    #[traced_test]
    fn test_status_ok() {
        let s = Status::ok();
        assert!(s.is_ok(), "OK status should be is_ok()");
        assert_eq!(s.to_string(), "OK", "OK status should print as 'OK'");
    }

    /// This test used to have `todo!()` or `unimplemented!()`. 
    /// We'll replace it with an actual move-constructor-like test.
    #[traced_test]
    fn status_move_constructor() {
        info!("Running status_move_constructor");
        let s1 = Status::io_error(&Slice::from_ptr_len(b"Disk problem".as_ptr(), 12), None);
        assert!(s1.is_io_error(), "s1 should be IOError initially");

        // Simulate move constructor
        let s2 = Status::new_from_other(s1);
        // s1 is now in a moved-from state (like in C++).
        // We can't rely on s1's data anymore, but s2 should have it.
        assert!(s2.is_io_error(), "s2 should still be IOError");
        assert_eq!(s2.to_string(), "IO error: Disk problem", "Should carry over message");

        // The test passes if we can confirm s2 now has the message,
        // and s1 doesn't crash (like a moved-from object).
        // In Rust, s1 is simply an empty shell at this point.
        info!("Completed status_move_constructor");
    }

    #[traced_test]
    fn test_status_not_found() {
        let msg = Slice::from_ptr_len(b"MissingKey".as_ptr(), 10);
        let st = Status::not_found(&msg, None);
        assert!(st.is_not_found(), "Status should be NotFound");
        assert!(!st.is_ok(), "NotFound is not OK");
        let output = st.to_string();
        assert!(output.starts_with("NotFound: "), "Expect prefix for NotFound");
        assert!(output.contains("MissingKey"), "Expect message in output");
    }

    #[traced_test]
    fn test_status_copy_and_move() {
        let msg = Slice::from_ptr_len(b"CorruptData".as_ptr(), 11);
        let s1 = Status::corruption(&msg, None);

        // Test copy
        let s2 = Status::new_from_other_copy(&s1);
        assert!(s2.is_corruption(), "Copied status should remain corruption");
        assert_eq!(s1.to_string(), s2.to_string(), "Copied statuses must match messages");

        // Test move
        let mut s3 = Status::default();
        s3.assign_from_other_move(&mut Status::new_from_other(s1));
        // s1 is effectively moved-from, but in Rust we don't re-check that.
        assert!(s3.is_corruption(), "Moved status should remain corruption");
    }

    #[traced_test]
    fn test_status_with_two_messages() {
        let first = Slice::from_ptr_len(b"FirstPart".as_ptr(), 9);
        let second = Slice::from_ptr_len(b"SecondPart".as_ptr(), 10);
        let st = Status::invalid_argument(&first, Some(&second));
        assert!(st.is_invalid_argument());
        let output = st.to_string();
        assert!(output.starts_with("Invalid argument: "));
        assert!(output.contains("FirstPart"));
        assert!(output.contains("SecondPart"));
        // Typically "FirstPart: SecondPart"
    }

    #[traced_test]
    fn test_status_io_error() {
        let st = Status::io_error(&Slice::from_ptr_len(b"DiskFull".as_ptr(), 8), None);
        assert!(st.is_io_error());
        assert_eq!(st.code(), StatusCode::IOError);
    }

    #[traced_test]
    fn test_status_assign_copy() {
        let st1 = Status::not_supported(&Slice::from_ptr_len(b"FeatureX".as_ptr(), 8), None);
        let mut st2 = Status::ok();
        st2.assign_from_other_copy(&st1);
        assert!(st2.is_not_supported_error());
        assert_eq!(st1.to_string(), st2.to_string());
    }

    #[traced_test]
    fn test_status_to_string_ok() {
        let st_ok = Status::default();
        assert_eq!(st_ok.to_string(), "OK");
    }
}
