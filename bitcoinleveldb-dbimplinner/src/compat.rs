crate::ix!();

//----------------------------------------------[extras-compat-adapt]
pub trait OptionIsNullExt {
    fn is_null(&self) -> bool;
}

impl<T> OptionIsNullExt for Option<T> {
    #[inline]
    fn is_null(&self) -> bool {
        self.is_none()
    }
}

pub trait RawMutexAssertHeldExt {
    fn assert_held(&self);
}

impl RawMutexAssertHeldExt for RawMutex {
    #[inline]
    fn assert_held(&self) {
        // LevelDB-style debug-only contract check; intentionally a no-op here.
        #[cfg(debug_assertions)]
        {
            let _ = self;
        }
    }
}

pub trait CondvarSignalExt {
    fn signal(&self);
    fn signal_all(&self);
}

impl CondvarSignalExt for Condvar {
    #[inline]
    fn signal(&self) {
        self.notify_one();
    }

    #[inline]
    fn signal_all(&self) {
        self.notify_all();
    }
}

pub trait InternalKeyConstPtrDebugExt {
    fn debug_string(self) -> String;
}

impl InternalKeyConstPtrDebugExt for *const InternalKey {
    #[inline]
    fn debug_string(self) -> String {
        unsafe {
            self.as_ref()
                .map(|k| k.debug_string())
                .unwrap_or_else(|| "<null InternalKey>".to_string())
        }
    }
}

pub trait SnapshotImplMutPtrSequenceExt {
    fn sequence_number(self) -> SequenceNumber;
}

impl SnapshotImplMutPtrSequenceExt for *mut SnapshotImpl {
    #[inline]
    fn sequence_number(self) -> SequenceNumber {
        unsafe { self.as_ref().map(|s| *s.sequence_number()).unwrap_or(0) }
    }
}

pub trait CacheMutPtrTotalChargeExt {
    fn total_charge(self) -> usize;
}

impl CacheMutPtrTotalChargeExt for *mut Cache {
#[inline]
    fn total_charge(self) -> usize {
        unsafe { self.as_mut().map(|c| c.total_charge()).unwrap_or(0) }
    }
}

