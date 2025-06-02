crate::ix!();

impl Logger {

    /// Insert a new print-callback. We no longer return an iterator 
    /// (to avoid returning something referencing data locked in `inner`).
    pub fn push_back_callback(&mut self, fun: fn(&String) -> ()) {
        trace!("Logger::push_back_callback => adding a new print callback");
        let guard = self.cs.borrow();
        let mut inner = guard.lock();
        inner.print_callbacks_mut().push_back(Box::new(fun));
        trace!("Logger::push_back_callback => callback added");
    }

    /// Remove *all* callbacks, for simplicity. 
    /// (If you need to remove just one closure, store an ID or handle.)
    pub fn delete_callback(&mut self) {
        trace!("Logger::delete_callback => removing all callbacks");
        let guard = self.cs.borrow();
        let mut inner = guard.lock();
        inner.print_callbacks_mut().clear();
        trace!("Logger::delete_callback => cleared all callbacks");
    }
}
