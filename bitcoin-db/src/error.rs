crate::ix!();

pub struct DbWrapperError { }

impl DbWrapperError {

    pub fn new(msg: &String) -> Self {
    
        todo!();
        /*
            : std::runtime_error(msg)
        */
    }
}

pub fn dbwrapper_error<'a>(x: &'a str) -> &'a str {
    x
}
