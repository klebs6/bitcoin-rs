// ---------------- [ File: bitcoinleveldb-dumpfile/src/capturing_writable_file.rs ]
crate::ix!();

#[cfg(test)]
#[derive(Default)]
pub struct CapturingWritableFile {
    name: String,
    buf: Vec<u8>,
    append_calls: usize,
    append_override: Option<Status>,
}

#[cfg(test)]
impl CapturingWritableFile {
    pub fn new_named(name: &str) -> Self {
        Self {
            name: name.to_string(),
            buf: Vec::new(),
            append_calls: 0,
            append_override: None,
        }
    }

    pub fn contents_string(&self) -> String {
        String::from_utf8_lossy(&self.buf).to_string()
    }

    pub fn append_call_count(&self) -> usize {
        self.append_calls
    }

    pub fn force_append_status(&mut self, st: Status) {
        self.append_override = Some(st);
    }
}

#[cfg(test)]
impl WritableFile for CapturingWritableFile {}

#[cfg(test)]
impl WritableFileAppend for CapturingWritableFile {
    fn append(&mut self, data: &Slice) -> Status {
        self.append_calls += 1;
        self.buf.extend_from_slice(slice_as_bytes(data));
        if let Some(ref st) = self.append_override {
            st.clone()
        } else {
            Status::ok()
        }
    }
}

#[cfg(test)]
impl WritableFileClose for CapturingWritableFile {
    fn close(&mut self) -> Status {
        Status::ok()
    }
}

#[cfg(test)]
impl WritableFileFlush for CapturingWritableFile {
    fn flush(&mut self) -> Status {
        Status::ok()
    }
}

#[cfg(test)]
impl WritableFileSync for CapturingWritableFile {
    fn sync(&mut self) -> Status {
        Status::ok()
    }
}

#[cfg(test)]
impl Named for CapturingWritableFile {
    fn name(&self) -> std::borrow::Cow<'_, str> {
        std::borrow::Cow::Owned(self.name.clone())
    }
}
