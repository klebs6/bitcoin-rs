// ---------------- [ File: bitcoinleveldb-versionset/src/version_set.rs ]
crate::ix!();

#[derive(Builder, CopyGetters, Getters, MutGetters, Setters)]
#[builder(name = "VersionSetStateBuilder", vis = "pub(crate)", pattern = "owned")]
pub struct VersionSet {
    env: Box<dyn Env>,

    #[getset(get = "pub(crate)")]
    dbname: String,

    #[getset(get_copy = "pub(crate)")]
    options: *const Options,

    #[getset(get_copy = "pub(crate)")]
    table_cache: *const TableCache,

    #[getset(get = "pub(crate)")]
    icmp: InternalKeyComparator,

    #[getset(get_copy = "pub(crate)", set = "pub(crate)")]
    next_file_number: u64,

    #[getset(get_copy = "pub(crate)", set = "pub(crate)")]
    manifest_file_number: u64,

    #[getset(get_copy = "pub(crate)", set = "pub(crate)")]
    last_sequence: u64,

    #[getset(get_copy = "pub(crate)", set = "pub(crate)")]
    log_number: u64,

    /// 0 or backing store for memtable being
    /// compacted
    /// 
    #[getset(get_copy = "pub(crate)", set = "pub(crate)")]
    prev_log_number: u64,

    /// Opened lazily
    /// 
    #[getset(get_copy = "pub(crate)", set = "pub(crate)")]
    descriptor_file: *mut dyn WritableFile,

    #[getset(get_copy = "pub(crate)", set = "pub(crate)")]
    descriptor_log: *mut LogWriter,

    /// Head of circular doubly-linked list
    /// of versions.
    /// 
    #[getset(get_mut = "pub(crate)")]
    dummy_versions: Version,

    /// == dummy_versions_.prev_
    /// 
    #[getset(get_copy = "pub(crate)", set = "pub(crate)")]
    current: *mut Version,

    /// Per-level key at which the next compaction
    /// at that level should start.
    /// 
    /// Either an empty string, or a valid
    /// 
    /// InternalKey.
    /// 
    #[getset(get_mut = "pub(crate)")]
    compact_pointer: [String; NUM_LEVELS],
}

impl VersionSet {
    pub(crate) fn new_internal(
        dbname: &String,
        options: *const Options,
        table_cache: *mut TableCache,
        cmp: *const InternalKeyComparator,
    ) -> Self {
        assert!(!options.is_null(), "VersionSet::new_internal: options is null");
        assert!(!cmp.is_null(), "VersionSet::new_internal: cmp is null");

        let env_box: Box<dyn Env> = unsafe {
            let env_opt = (*options).env();
            let env_rc = env_opt
                .as_ref()
                .expect("VersionSet::new_internal: Options.env is None")
                .clone();
            Box::new(EnvWrapper::new(env_rc))
        };

        let icmp_copy: InternalKeyComparator =
            unsafe { InternalKeyComparator::new((*cmp).user_comparator()) };

        let dummy_files: [Vec<*mut FileMetaData>; NUM_LEVELS] = core::array::from_fn(|_| Vec::new());

        let compact_pointer_init: [String; NUM_LEVELS] = core::array::from_fn(|_| String::new());

        let dummy_versions = VersionBuilder::default()
            .vset(Self::null_versionset_interface_ptr())
            .next(core::ptr::null_mut())
            .prev(core::ptr::null_mut())
            .refs(0)
            .files(dummy_files)
            .file_to_compact(core::ptr::null_mut())
            .file_to_compact_level(-1)
            .compaction_score(-1.0)
            .compaction_level(-1)
            .build()
            .unwrap();

        let mut vset: VersionSet = VersionSetStateBuilder::default()
            .env(env_box)
            .dbname(dbname.clone())
            .options(options)
            .table_cache(table_cache as *const TableCache)
            .icmp(icmp_copy)
            .next_file_number(2)
            .manifest_file_number(0)
            .last_sequence(0)
            .log_number(0)
            .prev_log_number(0)
            .descriptor_file(Self::null_writable_file_ptr())
            .descriptor_log(core::ptr::null_mut())
            .dummy_versions(dummy_versions)
            .current(core::ptr::null_mut())
            .compact_pointer(compact_pointer_init)
            .build()
            .unwrap();

        {
            let vset_iface_ptr: *mut dyn VersionSetInterface =
                (&mut vset as &mut dyn VersionSetInterface) as *mut dyn VersionSetInterface;

            let dummy_ptr: *mut Version = vset.dummy_versions_mut() as *mut Version;

            unsafe {
                (*dummy_ptr).set_next(dummy_ptr);
                (*dummy_ptr).set_prev(dummy_ptr);
            }

            let initial_files: [Vec<*mut FileMetaData>; NUM_LEVELS] = core::array::from_fn(|_| Vec::new());

            let mut initial_v = Box::new(
                VersionBuilder::default()
                    .vset(vset_iface_ptr)
                    .next(core::ptr::null_mut())
                    .prev(core::ptr::null_mut())
                    .refs(0)
                    .files(initial_files)
                    .file_to_compact(core::ptr::null_mut())
                    .file_to_compact_level(-1)
                    .compaction_score(-1.0)
                    .compaction_level(-1)
                    .build()
                    .unwrap(),
            );

            assert!(
                *initial_v.refs() == 0,
                "VersionSet::new_internal: initial version refs must be 0"
            );

            let v_ptr: *mut Version = &mut *initial_v;

            vset.set_current(v_ptr);
            initial_v.ref_();

            initial_v.set_prev(dummy_ptr);
            initial_v.set_next(dummy_ptr);

            unsafe {
                (*dummy_ptr).set_next(v_ptr);
                (*dummy_ptr).set_prev(v_ptr);
            }

            let _leaked: *mut Version = Box::into_raw(initial_v);
            let _ = _leaked;
        }

        vset
    }
}

impl VersionSetInterface for VersionSet {}
impl VersionSetVersionInterface for VersionSet {}
impl CompactionInterface for VersionSet {}
