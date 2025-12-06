crate::ix!();

impl DBImpl {
    
    pub fn maybe_ignore_error(&self, s: *mut Status)  {
        
        todo!();
        /*
          if (s->ok() || options_.paranoid_checks) {
            // No change needed
          } else {
            Log(options_.info_log, "Ignoring error %s", s->ToString().c_str());
            *s = Status::OK();
          }
        */
    }
}
