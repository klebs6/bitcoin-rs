// ---------------- [ File: bitcoin-settings/src/read_settings.rs ]
crate::ix!();

/**
  | Read settings file.
  |
  */
pub fn read_settings(
    path:   &Path,
    values: &mut HashMap<String,SettingsValue>,
    errors: &mut Vec<String>) -> bool {
    
    todo!();
        /*
            values.clear();
        errors.clear();

        // Ok for file to not exist
        if (!fs::exists(path)) return true;

        fsbridge::ifstream file;
        file.open(path);
        if (!file.is_open()) {
          errors.emplace_back(strprintf("%s. Please check permissions.", fs::PathToString(path)));
          return false;
        }

        SettingsValue in;
        if (!in.read(std::string{std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>()})) {
            errors.emplace_back(strprintf("Unable to parse settings file %s", fs::PathToString(path)));
            return false;
        }

        if (file.fail()) {
            errors.emplace_back(strprintf("Failed reading settings file %s", fs::PathToString(path)));
            return false;
        }
        file.close(); // Done with file descriptor. Release while copying data.

        if (!in.isObject()) {
            errors.emplace_back(strprintf("Found non-object value %s in settings file %s", in.write(), fs::PathToString(path)));
            return false;
        }

        const std::vector<std::string>& in_keys = in.getKeys();
        const std::vector<SettingsValue>& in_values = in.getValues();
        for (size_t i = 0; i < in_keys.size(); ++i) {
            auto inserted = values.emplace(in_keys[i], in_values[i]);
            if (!inserted.second) {
                errors.emplace_back(strprintf("Found duplicate key %s in settings file %s", in_keys[i], fs::PathToString(path)));
            }
        }
        return errors.empty();
        */
}
