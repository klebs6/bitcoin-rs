// ---------------- [ File: bitcoinleveldb-env/src/global_state.rs ]
crate::ix!();

/**
  | Returns a stable identity key for a wrapped Env target.
  |
  | Invariant:
  | wrappers constructed from the same Rc allocation must map to the same key so
  | that serialized forwarding remains shared across all aliases of that target.
  */
pub fn env_wrapper_target_identity_key(
    target: &Rc<RefCell<dyn Env>>,
) -> usize {
    let raw_target_ptr: *const RefCell<dyn Env> = Rc::as_ptr(target);
    let identity_key: usize = raw_target_ptr as *const () as usize;

    trace!(
        target: "bitcoinleveldb_env::global_state",
        label = "env_wrapper_target_identity_key",
        identity_key,
    );

    identity_key
}

/**
  | Returns the shared forwarding gate for `target`.
  |
  | Invariant:
  | every EnvWrapper that forwards to the same wrapped target must serialize
  | access through the same mutex so that RefCell borrow state is never observed
  | concurrently from foreground and background LevelDB threads.
  */
pub fn env_wrapper_shared_borrow_gate_for_target(
    target: &Rc<RefCell<dyn Env>>,
) -> Arc<Mutex<()>> {
    let identity_key = env_wrapper_target_identity_key(target);

    trace!(
        target: "bitcoinleveldb_env::global_state",
        label = "env_wrapper_shared_borrow_gate_for_target.entry",
        identity_key,
    );

    let registry_lock_result = env_wrapper_shared_borrow_gate_registry_state().lock();

    let mut registry_guard = match registry_lock_result {
        Ok(registry_guard) => registry_guard,
        Err(poisoned_registry_guard) => {
            warn!(
                target: "bitcoinleveldb_env::global_state",
                label = "env_wrapper_shared_borrow_gate_for_target.registry_poisoned",
                identity_key,
            );
            poisoned_registry_guard.into_inner()
        }
    };

    match registry_guard.get(&identity_key) {
        Some(existing_gate) => {
            trace!(
                target: "bitcoinleveldb_env::global_state",
                label = "env_wrapper_shared_borrow_gate_for_target.reuse_existing",
                identity_key,
            );
            existing_gate.clone()
        }
        None => {
            let new_gate = Arc::new(Mutex::new(()));

            trace!(
                target: "bitcoinleveldb_env::global_state",
                label = "env_wrapper_shared_borrow_gate_for_target.install_new",
                identity_key,
            );

            registry_guard.insert(identity_key, new_gate.clone());
            new_gate
        }
    }
}

/**
  | Stores the per-target serialization gates used by EnvWrapper forwarding.
  |
  | Invariant:
  | the registry is initialized once and then reused for the process lifetime so
  | that gate identity is stable across repeated wrapper construction.
  */
fn env_wrapper_shared_borrow_gate_registry_state(
) -> &'static Mutex<BTreeMap<usize, Arc<Mutex<()>>>> {
    static ENV_WRAPPER_SHARED_BORROW_GATE_REGISTRY_STATE:
        OnceLock<Mutex<BTreeMap<usize, Arc<Mutex<()>>>>> = OnceLock::new();

    ENV_WRAPPER_SHARED_BORROW_GATE_REGISTRY_STATE.get_or_init(|| {
        trace!(
            target: "bitcoinleveldb_env::global_state",
            label = "env_wrapper_shared_borrow_gate_registry_state.initialize",
        );
        Mutex::new(BTreeMap::new())
    })
}
