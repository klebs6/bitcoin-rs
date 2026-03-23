// ---------------- [ File: bitcoinleveldbt-faultinjection/src/test_writable_file.rs ]
crate::ix!();

/**
  | A wrapper around WritableFile which
  | informs another Env whenever this file
  | is written to or sync'ed.
  |
  */
#[derive(Getters,Setters,MutGetters,Builder)]
#[builder(setter(into))]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct TestWritableFile {
    state:                FileState,
    target:               *mut Box<dyn WritableFile>,
    writable_file_opened: bool,
    env:                  *mut FaultInjectionTestEnv,
}

impl TestWritableFile {
    pub fn new(
        state: &FileState,
        f:     *mut Box<dyn WritableFile>,
        env:   *mut FaultInjectionTestEnv,
    ) -> Self {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "test_writable_file_new_entry",
            target_is_null = f.is_null(),
            env_is_null = env.is_null()
        );

        assert!(!f.is_null());

        let out = Self {
            state: FileStateBuilder::default()
                .filename(state.filename().clone())
                .pos(*state.pos())
                .pos_at_last_sync(*state.pos_at_last_sync())
                .pos_at_last_flush(*state.pos_at_last_flush())
                .build()
                .unwrap(),
            target: f,
            writable_file_opened: true,
            env,
        };

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "test_writable_file_new_exit",
            writable_file_opened = out.writable_file_opened
        );

        out
    }
}

impl Drop for TestWritableFile {
    fn drop(&mut self) {
        debug!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "test_writable_file_drop_entry",
            writable_file_opened = self.writable_file_opened,
            target_is_null = self.target.is_null(),
            env_is_null = self.env.is_null()
        );

        if self.writable_file_opened {
            let _ = self.close();
        }

        if !self.target.is_null() {
            unsafe {
                drop(Box::from_raw(self.target));
            }
        }

        debug!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "test_writable_file_drop_exit"
        );
    }
}

impl WritableFile for TestWritableFile {

}

impl WritableFileAppend for TestWritableFile {
    fn append(&mut self, data: &Slice) -> crate::Status {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "test_writable_file_append_entry",
            filename = %self.state().filename(),
            data_size = *data.size()
        );

        let s = unsafe { (&mut *self.target).append(data) };

        if s.is_ok() && unsafe { (&mut *self.env).is_filesystem_active() } {
            *self.state.pos_mut() += *data.size() as i64;
        }

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "test_writable_file_append_exit",
            filename = %self.state().filename(),
            ok = s.is_ok(),
            pos = self.state().pos()
        );

        s
    }
}

impl WritableFileClose for TestWritableFile {
    fn close(&mut self) -> crate::Status {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "test_writable_file_close_entry",
            filename = %self.state().filename(),
            writable_file_opened = self.writable_file_opened()
        );

        self.set_writable_file_opened(false);
        let s = unsafe { (&mut *self.target).close() };

        if s.is_ok() {
            unsafe {
                (&mut *self.env).writable_file_closed(&self.state);
            }
        }

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "test_writable_file_close_exit",
            filename = %self.state().filename(),
            ok = s.is_ok(),
            writable_file_opened = self.writable_file_opened()
        );

        s
    }
}

impl WritableFileFlush for TestWritableFile {
    fn flush(&mut self) -> crate::Status {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "test_writable_file_flush_entry",
            filename = %self.state().filename(),
            pos = self.state().pos()
        );

        let s = unsafe { (&mut *self.target).flush() };

        if s.is_ok() && unsafe { (&mut *self.env).is_filesystem_active() } {
            *self.state_mut().pos_at_last_flush_mut() = *self.state().pos();
        }

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "test_writable_file_flush_exit",
            filename = %self.state().filename(),
            ok = s.is_ok(),
            pos_at_last_flush = self.state().pos_at_last_flush()
        );

        s
    }
}

impl WritableFileSync for TestWritableFile {
    fn sync(&mut self) -> crate::Status {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "test_writable_file_sync_entry",
            filename = %self.state().filename(),
            pos = self.state().pos()
        );

        if !unsafe { (&mut *self.env).is_filesystem_active() } {
            trace!(
                target: "bitcoinleveldbt_faultinjection::fault_injection_test",
                event = "test_writable_file_sync_exit",
                filename = %self.state().filename(),
                ok = true,
                filesystem_active = false
            );

            return Status::ok();
        }

        // Ensure new files referred to by the manifest are in the filesystem.
        let mut s = unsafe { (&mut *self.target).sync() };

        if s.is_ok() {
            *self.state_mut().pos_at_last_sync_mut() = *self.state().pos();
        }

        if unsafe {
            (&mut *self.env).is_file_created_since_last_dir_sync(&self.state().filename())
        } {
            let ps = self.sync_parent();
            if s.is_ok() && !ps.is_ok() {
                s = ps;
            }
        }

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "test_writable_file_sync_exit",
            filename = %self.state().filename(),
            ok = s.is_ok(),
            pos_at_last_sync = self.state().pos_at_last_sync()
        );

        s
    }
}

impl Named for TestWritableFile {
    
    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("".to_string())
    }
}
