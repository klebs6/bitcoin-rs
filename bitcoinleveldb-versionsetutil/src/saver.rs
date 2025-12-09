// ---------------- [ File: bitcoinleveldb-versionsetutil/src/saver.rs ]
crate::ix!();

/**
  | Callback from TableCache::Get()
  |
  */
pub enum SaverState {
    NotFound,
    Found,
    Deleted,
    Corrupt,
}

pub struct Saver {
    state:    SaverState,
    ucmp:     Box<dyn SliceComparator>,
    user_key_: Slice,
    value:    *mut String,
}

pub fn save_value(
        arg:  *mut c_void,
        ikey_: &Slice,
        v:    &Slice)  {
    
    todo!();
        /*
            Saver* s = reinterpret_cast<Saver*>(arg);
      ParsedInternalKey parsed_key;
      if (!ParseInternalKey(ikey, &parsed_key)) {
        s->state = kCorrupt;
      } else {
        if (s->ucmp->Compare(parsed_key.user_key, s->user_key) == 0) {
          s->state = (parsed_key.type == kTypeValue) ? kFound : kDeleted;
          if (s->state == kFound) {
            s->value->assign(v.data(), v.size());
          }
        }
      }
        */
}
