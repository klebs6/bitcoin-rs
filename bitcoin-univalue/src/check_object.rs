crate::ix!();

// ---------------- [ File: bitcoin-univalue/src/univalue.rs ]  (additional impl block)

impl UniValue {
    /// Validate that **every** entry in *template* exists in `self`
    /// **and** has the requested `VType`.
    #[instrument(level = "trace", skip(self, template))]
    pub fn check_object(&self, template: &HashMap<String, uni_value::VType>) -> bool {
        if *self.typ() != uni_value::VType::VOBJ {
            trace!("receiver not an object");
            return false;
        }

        for (key, want) in template {
            let mut idx = 0usize;
            if !self.find_key(key, &mut idx) {
                trace!(missing = key);
                return false;
            }
            let got = self.values()[idx].get_type();
            if got != *want {
                trace!(key, ?got, ?want, "type mismatch");
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod check_object_spec {
    use super::*;

    #[traced_test]
    fn validates_shape() {
        let mut obj = UniValue::new(uni_value::VType::VOBJ, None);
        obj.pushkv("num", 7i64);
        obj.pushkv("flag", false);

        let mut tmpl = HashMap::new();
        tmpl.insert("num".into(),  uni_value::VType::VNUM);
        tmpl.insert("flag".into(), uni_value::VType::VBOOL);

        assert!(obj.check_object(&tmpl));

        // wrong type
        tmpl.insert("num".into(), uni_value::VType::VSTR);
        assert!(!obj.check_object(&tmpl));

        // missing key
        tmpl.remove("num");
        tmpl.insert("oops".into(), uni_value::VType::VNUM);
        assert!(!obj.check_object(&tmpl));
    }
}
