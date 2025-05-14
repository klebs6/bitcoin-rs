// ---------------- [ File: bitcoin-banman/src/banmap.rs ]
crate::ix!();

pub type BanMap = HashMap<SubNet,BanEntry>;

pub const BANMAN_JSON_ADDR_KEY: &'static str = "address";

/**
  | Convert a `banmap_t` object to a JSON
  | array.
  | 
  | -----------
  | @param[in] bans
  | 
  | Bans list to convert.
  | 
  | -----------
  | @return
  | 
  | a JSON array, similar to the one returned
  | by the `listbanned` RPC. Suitable for
  | passing to `BanMapFromJson()`.
  |
  */
pub fn ban_map_to_json(bans: &BanMap) -> UniValue {
    
    let mut bans_json: UniValue 
    = UniValue::new(uni_value::VType::VARR, None);

    for it in bans.iter() {

        let address   = &it.0;
        let ban_entry = &it.1;

        let mut j: UniValue = ban_entry.to_json();

        j.pushkv(BANMAN_JSON_ADDR_KEY, &address.to_string());

        bans_json.push_back(&j);
    }

    bans_json
}

/**
  | Convert a JSON array to a `banmap_t`
  | object.
  | 
  | -----------
  | @param[in] bans_json
  | 
  | JSON to convert, must be as returned
  | by `BanMapToJson()`.
  | ----------
  | @param[out] bans
  | 
  | Bans list to create from the JSON. @throws
  | std::runtime_error if the JSON does
  | not have the expected fields or they
  | contain unparsable values.
  |
  */
pub fn ban_map_from_json(
        bans_json: &UniValue,
        bans:      &mut BanMap) -> Result<(), StdException> {

    match bans_json.get_values() {
        Ok(values) => {

            for ban_entry_json in values.iter() {

                let mut subnet = SubNet::default();

                let subnet_str = &ban_entry_json[BANMAN_JSON_ADDR_KEY].get_str();

                if !lookup_sub_net(subnet_str, &mut subnet, None) {

                    let msg = format!{
                        "Cannot parse banned address or subnet: {}",
                        subnet_str
                    };

                    return Err(runtime_error(&msg));
                }

                bans.insert(
                    subnet, 
                    BanEntry::from(ban_entry_json)
                );
            }

            Ok(())
        },
        Err(e) => {
            return Err(e)
        }
    }
}
