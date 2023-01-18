crate::ix!();

/**
  | JSON key under which the data is stored
  | in the json database.
  |
  */
pub const JSON_KEY: &'static str = "banned_nets";

/**
  | Access to the banlist database (banlist.json)
  |
  */
pub struct BanDB {
    banlist_dat:  PathBuf,
    banlist_json: PathBuf,
}

impl BanDB {

    pub fn new(ban_list_path: PathBuf) -> Self {

        let mut dat  = ban_list_path.clone();
        dat.push(".dat");

        let mut json = ban_list_path.clone();
        json.push(".json");

        Self {
            banlist_dat:  dat,
            banlist_json: json,
        }
    }
    
    pub fn write(&mut self, ban_set: &BanMap) -> bool {
        
        let mut errors = Vec::<String>::default();

        let map = {
            let mut x = HashMap::new();
            x.insert(JSON_KEY.to_string(), SettingsValue(ban_map_to_json(ban_set)));
            x
        };

        if write_settings(
            &self.banlist_json,
            &map,
            &mut errors) 
        {
            return true;
        }

        for err in errors.iter() {
            eprintln!("{}", err);
        }

        false
    }
    
    /**
      | Read the banlist from disk.
      | 
      | -----------
      | @param[out] banSet
      | 
      | The loaded list. Set if `true` is returned,
      | otherwise it is left in an undefined
      | state.
      | 
      | -----------
      | @return
      | 
      | true on success
      |
      */
    pub fn read(&mut self, ban_set: &mut BanMap) -> bool {
        
        if self.banlist_dat.exists() {
            log_printf!(
                "banlist.dat ignored because it can only be read by {} version 22.x. Remove {} to silence this warning.\n", 
                PACKAGE_NAME, 
                quoted(banlist_dat.to_string())
            );
        }

        // If the JSON banlist does not exist,
        // then recreate it
        if !self.banlist_json.exists() {
            return false;
        }

        let mut settings = HashMap::<String,SettingsValue>::default();
        let mut errors = Vec::<String>::default();

        if !read_settings(&self.banlist_json,&mut settings,&mut errors) {

            for err in errors.iter() {
                log_printf!(
                    "Cannot load banlist {}: {}\n", 
                    self.banlist_json.to_string(), 
                    err
                );
            }

            return false;
        }

        let mut try_block = || -> TryBlockResult::<_,&'static str> {
            ban_map_from_json(&settings[JSON_KEY].0, ban_set);
            TryBlockResult::Success
        };

        match try_block() {
            TryBlockResult::Return(v)  => return v,
            TryBlockResult::Err(e)  => {
                log_printf!(
                    "Cannot parse banlist {}: {}\n", 
                    self.banlist_json.to_string(), 
                    e.what()
                );
                return false;
            },

            TryBlockResult::Break   => { }
            TryBlockResult::Success => { }
        }

        true
    }
}
