// ---------------- [ File: bitcoinleveldb-env/src/rc_writable_file_adapter.rs ]
crate::ix!();

pub struct RcWritableFileAdapter {
    pub inner: Rc<RefCell<dyn WritableFile>>,
}

impl WritableFile for RcWritableFileAdapter {}

impl Named for RcWritableFileAdapter {
    fn name(&self) -> Cow<'_,str> {
        trace!("RcWritableFileAdapter::get_name");
        Cow::Owned("[rc-writable-file-adapter]".to_string())
    }
}

impl WritableFileAppend for RcWritableFileAdapter {
    fn append(&mut self, data: &Slice) -> Status {
        trace!(
            len = *data.size(),
            "RcWritableFileAdapter::append forwarding to inner WritableFile"
        );
        let status = self.inner.borrow_mut().append(data);
        debug!(
            ok = status.is_ok(),
            "RcWritableFileAdapter::append completed"
        );
        status
    }
}

impl WritableFileClose for RcWritableFileAdapter {
    fn close(&mut self) -> Status {
        trace!("RcWritableFileAdapter::close forwarding to inner WritableFile");
        let status = self.inner.borrow_mut().close();
        debug!(
            ok = status.is_ok(),
            "RcWritableFileAdapter::close completed"
        );
        status
    }
}

impl WritableFileFlush for RcWritableFileAdapter {
    fn flush(&mut self) -> Status {
        trace!("RcWritableFileAdapter::flush forwarding to inner WritableFile");
        let status = self.inner.borrow_mut().flush();
        debug!(
            ok = status.is_ok(),
            "RcWritableFileAdapter::flush completed"
        );
        status
    }
}

impl WritableFileSync for RcWritableFileAdapter {
    fn sync(&mut self) -> Status {
        trace!("RcWritableFileAdapter::sync forwarding to inner WritableFile");
        let status = self.inner.borrow_mut().sync();
        debug!(
            ok = status.is_ok(),
            "RcWritableFileAdapter::sync completed"
        );
        status
    }
}
