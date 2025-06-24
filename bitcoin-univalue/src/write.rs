// ---------------- [ File: bitcoin-univalue/src/write.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/univalue/lib/univalue_write.cpp]

impl UniValue {
    /// Serialise this value to JSON.  
    /// `pretty_indent == 0` → compact, single‑line output.
    #[instrument(level = "trace", skip(self))]
    pub fn write(&self, pretty_indent: Option<u32>, indent_level: Option<u32>) -> String {
        let pretty_indent = pretty_indent.unwrap_or(0);
        let indent_level  = indent_level.unwrap_or(0);
        let mod_indent    = if indent_level == 0 { 1 } else { indent_level };

        let mut s = String::with_capacity(128);

        match *self.typ() {
            uni_value::VType::VNULL => s.push_str("null"),
            uni_value::VType::VOBJ  => self.write_object(pretty_indent, mod_indent, &mut s),
            uni_value::VType::VARR  => self.write_array(pretty_indent, mod_indent, &mut s),
            uni_value::VType::VSTR  => {
                s.push('"');
                s.push_str(&json_escape(self.val()));
                s.push('"');
            }
            uni_value::VType::VNUM  => s.push_str(self.val()),
            uni_value::VType::VBOOL => {
                s.push_str(if self.val() == "1" { "true" } else { "false" });
            }
        }
        s
    }

    /* ------------------------------------------------------------------ */
    /* helpers                                                             */
    /* ------------------------------------------------------------------ */
    fn write_array(&self, pretty_indent: u32, indent_level: u32, s: &mut String) {

        s.push('[');

        if pretty_indent != 0 {
            s.push('\n');
        }

        for (idx, val) in self.values().iter().enumerate() {

            if pretty_indent != 0 {
                indent_str(pretty_indent, indent_level, s);
            }

            s.push_str(&val.write(Some(pretty_indent), Some(indent_level + 1)));

            if idx + 1 != self.values().len() {
                s.push(',');
            }
            if pretty_indent != 0 {
                s.push('\n');
            }
        }

        if pretty_indent != 0 && indent_level > 0 {
            indent_str(pretty_indent, indent_level - 1, s);
        }

        s.push(']');
    }

    fn write_object(&self, pretty_indent: u32, indent_level: u32, s: &mut String) {
        s.push('{');
        if pretty_indent != 0 {
            s.push('\n');
        }

        for i in 0..self.keys().len() {
            if pretty_indent != 0 {
                indent_str(pretty_indent, indent_level, s);
            }

            s.push('"');
            s.push_str(&json_escape(&self.keys()[i]));
            s.push('"');
            s.push(':');
            if pretty_indent != 0 {
                s.push(' ');
            }
            s.push_str(&self.values()[i].write(Some(pretty_indent), Some(indent_level + 1)));

            if i + 1 != self.keys().len() {
                s.push(',');
            }
            if pretty_indent != 0 {
                s.push('\n');
            }
        }

        if pretty_indent != 0 && indent_level > 0 {
            indent_str(pretty_indent, indent_level - 1, s);
        }
        s.push('}');
    }
}

#[cfg(test)]
mod write_spec {
    use super::*;

    #[traced_test]
    fn scalars_compact() {
        assert_eq!(UniValue::default().write(None, None), "null");
        assert_eq!(UniValue::from(true).write(None, None), "true");
        assert_eq!(UniValue::from(42u64).write(None, None), "42");
        assert_eq!(UniValue::from("hi").write(None, None), r#""hi""#);
    }

    #[traced_test]
    fn array_compact_and_pretty() {
        let mut arr = UniValue::new(uni_value::VType::VARR, None);
        arr.values_mut().extend([1u64.into(), 2u64.into(), 3u64.into()]);

        assert_eq!(arr.write(None, None), "[1,2,3]");

        let pretty = arr.write(Some(2), Some(1));
        let expect = "[\n  1,\n  2,\n  3\n]";
        assert_eq!(pretty, expect);
    }

    #[traced_test]
    fn object_pretty() {
        let mut obj = UniValue::new(uni_value::VType::VOBJ, None);
        obj.pushkv("a", 1u64);
        obj.pushkv("b", false);

        let out = obj.write(Some(4), Some(1));
        let expect = "{\n    \"a\": 1,\n    \"b\": false\n}";
        assert_eq!(out, expect);
    }

    #[traced_test]
    fn nested_pretty() {
        let mut inner = UniValue::new(uni_value::VType::VARR, None);
        inner.values_mut().push("x".into());

        let mut outer = UniValue::new(uni_value::VType::VOBJ, None);
        outer.pushkv("inner", inner);

        let out = outer.write(Some(2), Some(1));
        let expect = "{\n  \"inner\": [\n    \"x\"\n  ]\n}";
        assert_eq!(out, expect);
    }
}
