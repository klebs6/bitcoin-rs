crate::ix!();

/**
  | Interface to let node manage chain clients
  | (wallets, or maybe tools for monitoring and
  | analysis in the future).
  */
pub trait ChainClient: 
RegisterRpcs
+ Verify
+ Load
+ Start
+ Flush
+ Stop
+ SetMockTime { }

pub trait RegisterRpcs {

    /**
      | Register rpcs.
      |
      */
    fn register_rpcs(&mut self);
}

pub trait Verify {

    /**
      | Check for errors before loading.
      |
      */
    fn verify(&mut self) -> bool;
}

pub trait Load {

    /**
      | Load saved state.
      |
      */
    fn load(&mut self) -> bool;
}

pub trait Start {

    /**
      | Start client execution and provide
      | a scheduler.
      |
      */
    fn start(&mut self, scheduler: &mut Scheduler);
}

pub trait Stop {

    /**
      | Shut down client.
      |
      */
    fn stop(&mut self);
}

pub trait SetMockTime {

    /**
      | Set mock time.
      |
      */
    fn set_mock_time(&mut self, time: i64);
}
