crate::ix!();

impl Logger {

    /// Insert a new print-callback
    pub fn push_back_callback(&mut self, fun: fn(&String) -> ()) {
        let mut inner = self.cs().lock();
        inner.print_callbacks_mut().push_back(Box::new(fun));
    }

    /// Remove *all* callbacks
    pub fn delete_callback(&mut self) {
        let mut inner = self.cs().lock();
        inner.print_callbacks_mut().clear();
    }
}
