// ---------------- [ File: bitcoin-coinsview/src/error_catcher.rs ]
crate::ix!();

/**
  | This is a minimally invasive approach
  | to shutdown on LevelDB read errors from
  | the chainstate, while keeping user
  | interface out of the common library,
  | which is shared between bitcoind, and
  | bitcoin-qt and non-server tools.
  | 
  | Writes do not need similar protection,
  | as failure to write is handled by the
  | caller.
  |
  */
pub struct CoinsViewErrorCatcher {
    base: CoinsViewBacked,

    /**
      | A list of callbacks to execute upon leveldb
      | read error.
      |
      */
    err_callbacks: Vec<fn() -> ()>,
}

impl From<*mut dyn CoinsView> for CoinsViewErrorCatcher {

    fn from(view: *mut dyn CoinsView) -> Self {
    
        todo!();
        /*
        : coins_view_backed(view),
        */
    }
}
    
impl CoinsViewErrorCatcher {
    pub fn add_read_err_callback(&mut self, f: fn() -> ())  {
        
        todo!();
        /*
            m_err_callbacks.emplace_back(std::move(f));
        */
    }
    
    pub fn get_coin(&self, 
        outpoint: &OutPoint,
        coin:     &mut Coin) -> bool {
        
        todo!();
        /*
            try {
            return CoinsViewBacked::GetCoin(outpoint, coin);
        } catch(const std::runtime_error& e) {
            for (auto f : m_err_callbacks) {
                f();
            }
            LogPrintf("Error reading from database: %s\n", e.what());
            // Starting the shutdown sequence and returning false to the caller would be
            // interpreted as 'entry not found' (as opposed to unable to read data), and
            // could lead to invalid interpretation. Just exit immediately, as we can't
            // continue anyway, and all writes should be atomic.
            std::abort();
        }
        */
    }
}
