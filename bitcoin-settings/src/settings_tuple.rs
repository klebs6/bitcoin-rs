crate::ix!();

pub struct SettingsTuple((String,SettingsValue));

impl std::fmt::Display for SettingsTuple {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
        /*
            SettingsValue out(SettingsValue::VOBJ);
        out.__pushKV(kv.first, kv.second);
        os << out.write();
        return os;
        */
    }
}
