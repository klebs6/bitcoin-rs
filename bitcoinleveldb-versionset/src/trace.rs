crate::ix!();

impl VersionSet {
    #[cfg(any(test, debug_assertions))]
    pub(crate) fn trace_version_list_pointers_for_versionset_move_diagnostics(
        &mut self,
        context: &'static str,
    ) {
        let self_ptr: *mut VersionSet = self as *mut VersionSet;

        let self_iface_ptr: *mut dyn VersionSetInterface =
            (self as &mut dyn VersionSetInterface) as *mut dyn VersionSetInterface;
        let self_iface_data: *mut () = self_iface_ptr as *mut ();
        let self_iface_meta = core::ptr::metadata(self_iface_ptr);

        let dummy_ptr: *mut Version = self.dummy_versions_mut_ptr();
        let dummy_offset: usize = (dummy_ptr as usize).wrapping_sub(self_ptr as usize);

        let current_ptr: *mut Version = self.current();

        let (dummy_next, dummy_prev, dummy_vset_ptr): (*mut Version, *mut Version, *mut dyn VersionSetInterface) =
            unsafe {
                let d: &mut Version = &mut *dummy_ptr;
                (*d.next(), *d.prev(), d.vset())
            };

        let dummy_vset_data: *mut () = dummy_vset_ptr as *mut ();
        let dummy_vset_meta = core::ptr::metadata(dummy_vset_ptr);

        let head_ptr: *mut Version = dummy_next;
        let tail_ptr: *mut Version = dummy_prev;

        let head_prev: *mut Version = if !head_ptr.is_null() {
            unsafe { *(*head_ptr).prev() }
        } else {
            core::ptr::null_mut()
        };

        let tail_next: *mut Version = if !tail_ptr.is_null() {
            unsafe { *(*tail_ptr).next() }
        } else {
            core::ptr::null_mut()
        };

        let current_prev: *mut Version = if !current_ptr.is_null() {
            unsafe { *(*current_ptr).prev() }
        } else {
            core::ptr::null_mut()
        };

        let current_next: *mut Version = if !current_ptr.is_null() {
            unsafe { *(*current_ptr).next() }
        } else {
            core::ptr::null_mut()
        };

        let head_vset_ptr: *mut dyn VersionSetInterface = if !head_ptr.is_null() {
            unsafe { (*head_ptr).vset() }
        } else {
            Self::null_versionset_interface_ptr()
        };
        let head_vset_data: *mut () = head_vset_ptr as *mut ();
        let head_vset_meta = core::ptr::metadata(head_vset_ptr);

        let tail_vset_ptr: *mut dyn VersionSetInterface = if !tail_ptr.is_null() {
            unsafe { (*tail_ptr).vset() }
        } else {
            Self::null_versionset_interface_ptr()
        };
        let tail_vset_data: *mut () = tail_vset_ptr as *mut ();
        let tail_vset_meta = core::ptr::metadata(tail_vset_ptr);

        tracing::info!(
            dbname = %self.dbname(),
            context,
            self_ptr = self_ptr as usize,
            self_iface_data = self_iface_data as usize,
            self_iface_meta = ?self_iface_meta,
            dummy_ptr = dummy_ptr as usize,
            dummy_offset,
            dummy_next = dummy_next as usize,
            dummy_prev = dummy_prev as usize,
            dummy_vset_data = dummy_vset_data as usize,
            dummy_vset_meta = ?dummy_vset_meta,
            current_ptr = current_ptr as usize,
            current_prev = current_prev as usize,
            current_next = current_next as usize,
            head_ptr = head_ptr as usize,
            head_prev = head_prev as usize,
            head_vset_data = head_vset_data as usize,
            head_vset_meta = ?head_vset_meta,
            tail_ptr = tail_ptr as usize,
            tail_next = tail_next as usize,
            tail_vset_data = tail_vset_data as usize,
            tail_vset_meta = ?tail_vset_meta,
            "VersionSet move diagnostics: version-list and trait-object pointers"
        );

        eprintln!(
            "[versionset-move-diagnostics] dbname='{}' ctx='{}' self={:#x} dummy={:#x} head={:#x} tail={:#x} head.prev={:#x} tail.next={:#x} self_iface.data={:#x} head.vset.data={:#x}",
            self.dbname(),
            context,
            self_ptr as usize,
            dummy_ptr as usize,
            head_ptr as usize,
            tail_ptr as usize,
            head_prev as usize,
            tail_next as usize,
            self_iface_data as usize,
            head_vset_data as usize
        );
    }
}

impl VersionSet {
    #[cfg(any(test, debug_assertions))]
    pub(crate) fn assert_version_list_sentinel_consistency_for_versionset_move_diagnostics(
        &mut self,
        context: &'static str,
    ) {
        let self_iface_ptr: *mut dyn VersionSetInterface =
            (self as &mut dyn VersionSetInterface) as *mut dyn VersionSetInterface;
        let self_iface_data: *mut () = self_iface_ptr as *mut ();

        let dummy_ptr: *mut Version = self.dummy_versions_mut_ptr();

        let (head_ptr, tail_ptr): (*mut Version, *mut Version) = unsafe {
            let d: &mut Version = &mut *dummy_ptr;
            (*d.next(), *d.prev())
        };

        // Empty list: dummy is self-linked.
        if head_ptr == dummy_ptr && tail_ptr == dummy_ptr {
            let self_ptr = (self as *mut VersionSet) as usize;

            tracing::debug!(
                dbname = %self.dbname(),
                context,
                self_ptr = self_ptr,
                dummy_ptr = dummy_ptr as usize,
                "VersionSet sentinel consistency: empty list (dummy self-loop)"
            );
            return;
        }

        if head_ptr.is_null() || tail_ptr.is_null() {
            self.trace_version_list_pointers_for_versionset_move_diagnostics(context);
            panic!(
                "VersionSet sentinel consistency failed: head/tail null (head={:p} tail={:p} dummy={:p}) in {}",
                head_ptr,
                tail_ptr,
                dummy_ptr,
                context
            );
        }

        let head_prev: *mut Version = unsafe { *(*head_ptr).prev() };
        let tail_next: *mut Version = unsafe { *(*tail_ptr).next() };

        let head_vset_ptr: *mut dyn VersionSetInterface = unsafe { (*head_ptr).vset() };
        let head_vset_data: *mut () = head_vset_ptr as *mut ();

        let mut ok: bool = true;

        if head_prev != dummy_ptr {
            ok = false;
            tracing::error!(
                dbname = %self.dbname(),
                context,
                head_ptr = head_ptr as usize,
                head_prev = head_prev as usize,
                dummy_ptr = dummy_ptr as usize,
                "VersionSet sentinel consistency failed: head.prev != dummy"
            );
        }

        if tail_next != dummy_ptr {
            ok = false;
            tracing::error!(
                dbname = %self.dbname(),
                context,
                tail_ptr = tail_ptr as usize,
                tail_next = tail_next as usize,
                dummy_ptr = dummy_ptr as usize,
                "VersionSet sentinel consistency failed: tail.next != dummy"
            );
        }

        if head_vset_data != self_iface_data {
            ok = false;
            tracing::error!(
                dbname = %self.dbname(),
                context,
                self_iface_data = self_iface_data as usize,
                head_vset_data = head_vset_data as usize,
                head_vset_meta = ?core::ptr::metadata(head_vset_ptr),
                self_iface_meta = ?core::ptr::metadata(self_iface_ptr),
                "VersionSet sentinel consistency failed: head.vset trait-object data != self trait-object data"
            );
        }

        if !ok {
            self.trace_version_list_pointers_for_versionset_move_diagnostics(context);
            panic!(
                "VersionSet sentinel consistency failed in {} (likely VersionSet moved after sentinel/link initialization)",
                context
            );
        }

        tracing::debug!(
            dbname = %self.dbname(),
            context,
            self_iface_data = self_iface_data as usize,
            dummy_ptr = dummy_ptr as usize,
            head_ptr = head_ptr as usize,
            tail_ptr = tail_ptr as usize,
            "VersionSet sentinel consistency: OK"
        );
    }
}
