crate::ix!();

pub fn save_errors(
        errors:        Vec<String>,
        mut error_out: Option<&mut Vec<String>>)  {
    
    for error in errors.iter() {

        if let Some(ref mut vec) = error_out {
            vec.push(error.to_string());

        } else {
            log_printf!("{}\n", error);
        }
    }
}

pub fn rename_over(
        src: &Path,
        dst: &Path) -> bool {
    std::fs::rename(src, dst).is_ok()
}

impl ArgsManagerInner {

    /**
      | Access settings with lock held.
      |
      */
    pub fn lock_settings<F: Fn(&Settings) -> ()>(&mut self, fn_: F)  {
    
        (fn_)(&self.settings);
    }

    /**
      | Read and update settings file with saved
      | settings. This needs to be called after
      | SelectParams() because the settings
      | file location is network-specific.
      |
      */
    pub fn init_settings(&mut self, error: &mut String) -> Result<(),String> {

        if !self.get_settings_path(None, None) {

            // Do nothing if settings file
            // disabled.
            return Ok(());
        }

        let mut errors = Vec::<String>::default();

        let read_settings_result  = self.read_settings_file(Some(&mut errors));

        if read_settings_result == false {
            return Err(format!("Failed loading settings file:\n{}\n",make_unordered_list(&errors)));
        }

        let write_settings_result = self.write_settings_file(Some(&mut errors));

        if !write_settings_result.is_err() || write_settings_result.unwrap() == false {
            return Err(format!("Failed saving settings file:\n{}\n",make_unordered_list(&errors)));
        }

        Ok(())
    }
    
    /**
      | Read settings file. Push errors to vector,
      | or log them if null.
      |
      */
    pub fn read_settings_file(&mut self, errors: Option<&mut Vec<String>>) -> bool {
        
        let temp = false;

        let mut path: Option<Box<Path>> = None;

        if !self.get_settings_path(
            path.as_mut(),
            Some(temp)) 
        {
            // Do nothing if settings file disabled.
            return true;
        }

        self.settings.rw_settings.clear();

        let mut read_errors = Vec::<String>::default();

        if !read_settings(
            path.as_ref().unwrap(),
            &mut self.settings.rw_settings,
            &mut read_errors) 
        {
            save_errors(read_errors, errors);
            return false;
        }

        for setting in self.settings.rw_settings.iter() {

            let mut section = String::default();

            let mut key: String = setting.0.to_string();

            //  Split setting key into section and argname
            interpret_option(&mut section,&mut key,&Default::default());

            let arg = format!{"-{}", key};

            if self.get_arg_flags(&arg).is_none() {
                log_printf!(
                    "Ignoring unknown rw_settings value {}\n", 
                    setting.0
                );
            }
        }

        true
    }
    
    /**
      | Write settings file. Push errors to
      | vector, or log them if null.
      |
      */
    pub fn write_settings_file(&self, 
        mut errors: Option<&mut Vec<String>>) -> Result<bool,StdException> {

        let mut path:     Option<Box<Path>> = None;
        let mut path_tmp: Option<Box<Path>> = None;

        if !self.get_settings_path(path.as_mut(),     /* temp= */ Some(false)) 
        || !self.get_settings_path(path_tmp.as_mut(), /* temp= */ Some(true)) {
            return Err(logic_error("Attempt to write settings file when dynamic settings are disabled."));
        }

        let mut write_errors = Vec::<String>::default();

        if !write_settings(
            path_tmp.as_mut().unwrap(),
            &self.settings.rw_settings,
            &mut write_errors) 
        {
            save_errors(write_errors, errors);
            return Ok(false);
        }

        if !rename_over(
            path_tmp.as_ref().unwrap(),
            path.as_ref().unwrap()) 
        {

            let msg = format!{
                "Failed renaming settings file {:?} to {:?}\n",
                path_tmp.as_ref().unwrap(), 
                path.as_ref().unwrap()
            };

            save_errors(vec!{msg}, errors);

            return Ok(false);
        }

        Ok(true)
    }
}
