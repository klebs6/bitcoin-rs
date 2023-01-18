crate::ix!();

/**
  | Get the human-readable name for a filter
  | type. Returns empty string for unknown
  | types.
  |
  */
pub fn block_filter_type_name<'a>(filter_type: &BlockFilterType) -> &'a str {
    
    lazy_static!{
        static ref UNKNOWN_RETVAL: String = "".to_string();
    }

    match G_FILTER_TYPES.get(filter_type) {
        Some(val) => val,
        None      => &UNKNOWN_RETVAL
    }
}

/**
  | Find a filter type by its human-readable
  | name.
  |
  */
pub fn block_filter_type_by_name(
        name:        &String,
        filter_type: &mut BlockFilterType) -> bool {

    for entry in G_FILTER_TYPES.iter() {

        if entry.1 == name {
            *filter_type = entry.0.clone();
            return true;
        }
    }

    false
}

/**
  | Get a list of known filter types.
  |
  */
pub fn all_block_filter_types<'a>() -> &'a HashSet<BlockFilterType> {

    lazy_static!{
        static ref TYPES: HashSet::<BlockFilterType> = {

            let mut x = HashSet::<BlockFilterType>::default();

            for entry in G_FILTER_TYPES.iter() {
                x.insert(entry.0.clone());
            }

            x
        };
    }
    
    &TYPES
}

/**
  | Get a comma-separated list of known
  | filter type names.
  |
  */
pub fn list_block_filter_types<'a>() -> &'a str {

    lazy_static!{
        static ref TYPE_LIST: String = {

            let mut x = String::default();

            let mut first: bool = true;

            for entry in G_FILTER_TYPES.iter() {

                if !first {
                    x.push_str(", ");
                }

                x.push_str(entry.1);

                first = false;
            }

            x
        };
    }

    &TYPE_LIST
}

pub fn basic_filter_elements(
        block:      &Block,
        block_undo: &BlockUndo) -> gcs_filter::ElementSet {
    
    let mut elements = gcs_filter::ElementSet::default();

    for tx in block.vtx.iter() {

        for txout in (*tx).get().vout.iter() {

            let script: &Script = &txout.script_pub_key;

            if script.is_empty() || script[0] == ScriptError::OP_RETURN as u8 {
                continue;
            }

            elements.insert(script.base.to_vec()); //elements.emplace(script.begin(), script.end());
        }
    }

    for tx_undo in block_undo.vtxundo.iter() {

        for prevout in tx_undo.vprevout.iter() {

            let script: &Script = &prevout.out.script_pub_key;

            if script.is_empty() {
                continue;
            }

            elements.insert(script.base.to_vec()); //elements.emplace(script.begin(), script.end());
        }
    }

    elements
}
