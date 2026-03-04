// ---------------- [ File: bitcoinleveldb-harness/src/to_string.rs ]
crate::ix!();

impl Harness {

    pub fn to_string_with_data<'a>(
        &self,
        _data: &KVMap,
        it: Option<(&'a String, &'a String)>,
    ) -> String {
        /*
            if (it == data.end()) {
          return "END";
        } else {
          return "'" + it->first + "->" + it->second + "'";
        }
        */
        match it {
            None => "END".to_owned(),
            Some((k, v)) => {
                let mut out: String = String::with_capacity(1 + k.len() + 2 + v.len() + 1);
                out.push('\'');
                out.push_str(k);
                out.push_str("->");
                out.push_str(v);
                out.push('\'');
                out
            }
        }
    }

    pub fn to_string_rev<'a>(&self, data: &KVMap, it: Option<(&'a String, &'a String)>) -> String {
        /*
            if (it == data.rend()) {
          return "END";
        } else {
          return "'" + it->first + "->" + it->second + "'";
        }
        */
        self.to_string_with_data(data, it)
    }

    pub fn to_string(&self, it: *const LevelDBIterator) -> String {
        /*
            if (!it->Valid()) {
          return "END";
        } else {
          return "'" + it->key().ToString() + "->" + it->value().ToString() + "'";
        }
        */
        unsafe {
            if !(&*it).valid() {
                "END".to_owned()
            } else {
                let k: String = (&*it).key().to_string();
                let v: String = (&*it).value().to_string();

                let mut out: String = String::with_capacity(1 + k.len() + 2 + v.len() + 1);
                out.push('\'');
                out.push_str(&k);
                out.push_str("->");
                out.push_str(&v);
                out.push('\'');
                out
            }
        }
    }
}
