crate::ix!();

/**
  | Test Env to override default Env behavior
  | for testing.
  |
  */
pub struct TestEnv {
    base:             crate::EnvWrapper,
    ignore_dot_files: bool,
}

impl TestEnv {

    pub fn new(base: Rc<RefCell<dyn crate::Env>>) -> Self {
    
        todo!();
        /*
        : env_wrapper(base),
        : ignore_dot_files(false),
        */
    }
    
    pub fn set_ignore_dot_files(&mut self, ignored: bool)  {
        
        todo!();
        /*
            ignore_dot_files_ = ignored;
        */
    }
    
    pub fn get_children(&mut self, 
        dir:    &String,
        result: *mut Vec<String>) -> crate::Status {
        
        todo!();
        /*
            Status s = target()->GetChildren(dir, result);
        if (!s.ok() || !ignore_dot_files_) {
          return s;
        }

        std::vector<std::string>::iterator it = result->begin();
        while (it != result->end()) {
          if ((*it == ".") || (*it == "..")) {
            it = result->erase(it);
          } else {
            ++it;
          }
        }

        return s;
        */
    }
}
