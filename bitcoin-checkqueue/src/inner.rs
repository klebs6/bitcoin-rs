// ---------------- [ File: bitcoin-checkqueue/src/inner.rs ]
crate::ix!();

#[derive(Getters,MutGetters,Setters)]
#[getset(get="pub",set="pub",get_mut="pub")]
pub struct CheckQueueInner<T> {

    /**
      |The queue of elements to be processed.
      |
      |As the order of booleans doesn't matter, it
      |is used as a LIFO (stack)
      */
    queue:        Vec<T>,

    /**
      | The number of workers (including the
      | master) that are idle.
      |
      */
    n_idle:       i32, // default = { 0 }

    /**
      | The total number of workers (including
      | the master).
      |
      */
    n_total:      i32, // default = { 0 }

    /**
      | The temporary evaluation result.
      |
      */
    all_ok:       bool, // default = { true }

    /**
      | Number of verifications that haven't
      | completed yet.
      | 
      | This includes elements that are no longer
      | queued, but still in the worker's own
      | batches.
      |
      */
    n_todo:       u32, // default = { 0 }

    request_stop: bool, // default = { false }
}
