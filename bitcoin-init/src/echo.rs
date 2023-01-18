crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/interfaces/echo.h]
//-------------------------------------------[.cpp/bitcoin/src/interfaces/echo.cpp]

/**
  | Simple string echoing interface for
  | testing.
  |
  */
pub trait Echo {

    /**
      | Echo provided string.
      |
      */
    fn echo(&mut self, echo: &String) -> String;
}

pub struct EchoImpl { }

impl Echo for EchoImpl {

    fn echo(&mut self, echo: &String) -> String {
        echo.clone()
    }
}

/**
  | Return implementation of Echo interface.
  |
  */
pub fn make_echo() -> Box<dyn Echo> {
    
    todo!();
        /*
            return std::make_unique<EchoImpl>();
        */
}

