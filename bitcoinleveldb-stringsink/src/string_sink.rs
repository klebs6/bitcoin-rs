// ---------------- [ File: bitcoinleveldb-stringsink/src/string_sink.rs ]
crate::ix!();

#[derive(Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct StringSink {
    contents: Vec<u8>,
}

impl StringSink {

    pub fn new() -> Self {
        StringSink {
            contents: Vec::new(),
        }
    }

    pub fn with(contents: &str) -> Self {
        Self {
            contents: contents.as_bytes().to_vec(),
        }
    }
}

impl WritableFile for StringSink {}

impl WritableFileClose for StringSink {
    fn close(&mut self) -> crate::Status {
        trace!(
            "StringSink::close: length={} bytes",
            self.contents.len()
        );
        crate::Status::ok()
    }
}

impl WritableFileFlush for StringSink {
    fn flush(&mut self) -> crate::Status {
        trace!(
            "StringSink::flush: length={} bytes",
            self.contents.len()
        );
        crate::Status::ok()
    }
}

impl WritableFileSync for StringSink {
    fn sync(&mut self) -> crate::Status {
        trace!(
            "StringSink::sync: length={} bytes",
            self.contents.len()
        );
        crate::Status::ok()
    }
}

impl WritableFileAppend for StringSink {
    fn append(&mut self, data: &Slice) -> crate::Status {
        unsafe {
            let ptr   = *data.data();
            let len   = *data.size();
            let bytes = core::slice::from_raw_parts(ptr, len);

            trace!(
                "StringSink::append: appending {} bytes (current_len={})",
                len,
                self.contents.len()
            );

            // Treat contents as raw binary; preserve bytes exactly as written.
            self.contents.extend_from_slice(bytes);
        }

        crate::Status::ok()
    }
}

impl Named for StringSink {
    fn name(&self) -> std::borrow::Cow<'_, str> {
        std::borrow::Cow::Owned(String::from("StringSink"))
    }
}
#[cfg(test)]
mod string_sink_behavior_tests {
    use super::*;

    #[traced_test]
    fn string_sink_append_accumulates_content_in_order() {
        let mut sink = StringSink {
            contents: Vec::new(),
        };

        let part1 = Slice::from("hello".as_bytes());
        let part2 = Slice::from(" ".as_bytes());
        let part3 = Slice::from("world".as_bytes());

        let st1 = sink.append(&part1);
        let st2 = sink.append(&part2);
        let st3 = sink.append(&part3);

        assert!(st1.is_ok());
        assert!(st2.is_ok());
        assert!(st3.is_ok());

        assert_eq!(sink.contents(), String::from("hello world").as_bytes());
    }

    #[traced_test]
    fn string_sink_flush_sync_close_are_noops_but_return_ok() {
        let mut sink = StringSink {
            contents: String::from("abc").into(),
        };

        let flush_status = sink.flush();
        let sync_status = sink.sync();
        let close_status = sink.close();

        assert!(flush_status.is_ok());
        assert!(sync_status.is_ok());
        assert!(close_status.is_ok());

        assert_eq!(sink.contents(), String::from("abc").as_bytes());
    }
}
