// ---------------- [ File: bitcoin-banman/src/entry.rs ]
crate::ix!();

#[derive(Clone)]
pub struct BanEntry {
    pub n_version:     i32,
    pub n_create_time: OffsetDateTime,
    pub n_ban_until:   OffsetDateTime,
}

impl Default for BanEntry {
    fn default() -> Self {
        Self {
            n_version:     BAN_ENTRY_CURRENT_VERSION,
            n_create_time: OffsetDateTime::UNIX_EPOCH,
            n_ban_until:   OffsetDateTime::UNIX_EPOCH,
        }
    }
}

impl PartialEq<BanEntry> for BanEntry {
    
    #[inline] fn eq(&self, other: &BanEntry) -> bool {
        self.n_version        == other.n_version 
        && self.n_create_time == other.n_create_time 
        && self.n_ban_until   == other.n_ban_until
    }
}

impl Eq for BanEntry {}

pub const BAN_ENTRY_CURRENT_VERSION: i32 = 1;

impl From<&UniValue> for BanEntry {

    /**
      | Create a ban entry from JSON.
      | 
      | -----------
      | @param[in] json
      | 
      | A JSON representation of a ban entry,
      | as created by `ToJson()`. @throw std::runtime_error
      | if the JSON does not have the expected
      | fields.
      |
      */
    fn from(json: &UniValue) -> Self {
    
        Self {
            n_version:     json["version"].get_int(),
            n_create_time: OffsetDateTime::from_unix_timestamp(json["ban_created"].get_int64()).unwrap(),
            n_ban_until:   OffsetDateTime::from_unix_timestamp(json["banned_until"].get_int64()).unwrap(),
        }
    }
}

impl BanEntry {
    
    pub fn new_from_create_time(n_create_time_in: OffsetDateTime) -> Self {
    
        Self {
            n_create_time: n_create_time_in,
            ..Default::default()
        }
    }
    
    /**
      | Generate a JSON representation of this
      | ban entry.
      | 
      | -----------
      | @return
      | 
      | JSON suitable for passing to the `CBanEntry(const
      | UniValue&)` constructor.
      |
      */
    pub fn to_json(&self) -> UniValue {
        
        let mut json: UniValue = UniValue::new(uni_value::VType::VOBJ, None);

        json.pushkv("version",      &self.n_version);
        json.pushkv("ban_created",  &self.n_create_time);
        json.pushkv("banned_until", &self.n_ban_until);
        json
    }
}
