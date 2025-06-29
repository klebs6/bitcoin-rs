crate::ix!();

/**
  | Write settings file.
  |
  */
pub fn write_settings(
    path:   &Path,
    values: &HashMap<String,SettingsValue>,
    errors: &mut Vec<String>) -> bool {
    
    todo!();
        /*
            SettingsValue out(SettingsValue::VOBJ);
        for (const auto& value : values) {
            out.__pushKV(value.first, value.second);
        }
        fsbridge::ofstream file;
        file.open(path);
        if (file.fail()) {
            errors.emplace_back(strprintf("Error: Unable to open settings file %s for writing", fs::PathToString(path)));
            return false;
        }
        file << out.write(/* prettyIndent= */ 1, /* indentLevel= */ 4) << std::endl;
        file.close();
        return true;
        */
}
