crate::ix!();

pub trait GetSnapshot {

    /**
      | Return a handle to the current DB state.
      | Iterators created with this handle will all
      | observe a stable snapshot of the current DB
      | state.
      |
      | The caller must call ReleaseSnapshot(result)
      | when the snapshot is no longer needed.
      */
    fn get_snapshot(&mut self) -> Box<dyn Snapshot>;
}
