// ---------------- [ File: bitcoin-univalue/src/test_object.rs ]
use bitcoin_univalue::*;
use bitcoin_imports::*;
use std::panic;

/// Convenience wrapper: parse *src* as JSON and return `(ok, value)`.
fn read_from(src: &str) -> (bool, UniValue) {
    let mut uv = UniValue::default();
    let ok = uv.read(src.as_ptr(), src.len());
    (ok, uv)
}

fn assert_panics<F: FnOnce() + panic::UnwindSafe>(f: F) {
    assert!(panic::catch_unwind(f).is_err(), "code did not panic as expected");
}

//-------------------------------------------[.cpp/bitcoin/src/univalue/test/object.cpp]

#[traced_test]
fn univalue_constructor() {
    // default → null
    let v1 = UniValue::default();
    assert!(v1.is_null());

    // type‑only ctor
    let v2 = UniValue::new(uni_value::VType::VSTR, None);
    assert!(v2.is_str());

    // type + initial string
    let v3 = UniValue::new(uni_value::VType::VSTR, Some("foo"));
    assert!(v3.is_str());
    assert_eq!(v3.get_val_str(), "foo");

    // numeric from string
    let mut num_test = UniValue::default();
    assert!(num_test.set_num_str(&"82".to_string()));
    assert!(num_test.is_num());
    assert_eq!(num_test.get_val_str(), "82");

    // integral ctors
    let vu64: u64 = 82;
    let v4: UniValue = vu64.into();
    assert!(v4.is_num());
    assert_eq!(v4.get_val_str(), "82");

    let vi64: i64 = -82;
    let v5: UniValue = vi64.into();
    assert!(v5.is_num());
    assert_eq!(v5.get_val_str(), "-82");

    let vi: i32 = -688;
    let v6: UniValue = vi.into();
    assert!(v6.is_num());
    assert_eq!(v6.get_val_str(), "-688");

    let vd: f64 = -7.21;
    let v7: UniValue = vd.into();
    assert!(v7.is_num());
    assert_eq!(v7.get_val_str(), "-7.2100000000000001"); // 16‑digit fmt

    // string ctors
    let v_str = String::from("yawn");
    let v8: UniValue = v_str.clone().into();
    assert!(v8.is_str());
    assert_eq!(v8.get_val_str(), "yawn");

    let vcs: *const u8 = b"zappa\0".as_ptr();
    let v9: UniValue = vcs.into();
    assert!(v9.is_str());
    assert_eq!(v9.get_val_str(), "zappa");
}

#[traced_test]
fn univalue_typecheck() {
    // numeric but accessed as bool → panic
    let mut v1 = UniValue::default();
    assert!(v1.set_num_str(&"1".to_string()));
    assert!(v1.is_num());
    assert_panics(|| {
        let _ = v1.get_bool();
    });

    // bool ok, int access panics
    let mut v2 = UniValue::default();
    v2.set_bool(true);
    assert_eq!(v2.get_bool(), true);
    assert_panics(|| {
        let _ = v2.get_int();
    });

    // 64‑bit overflow / ok branch
    let mut v3 = UniValue::default();
    assert!(v3.set_num_str(&"32482348723847471234".to_string()));
    assert_panics(|| {
        let _ = v3.get_int64();
    });
    assert!(v3.set_num_str(&"1000".to_string()));
    assert_eq!(v3.get_int64(), 1000);

    // i32 overflow detection
    let mut v4 = UniValue::default();
    assert!(v4.set_num_str(&"2147483648".to_string()));
    assert_eq!(v4.get_int64(), 2_147_483_648);
    assert_panics(|| {
        let _ = v4.get_int();
    });
    assert!(v4.set_num_str(&"1000".to_string()));
    assert_eq!(v4.get_int(), 1000);
    assert_panics(|| {
        let _ = v4.get_str();
    });
    assert_eq!(v4.get_real(), 1000.0);
    assert_panics(|| {
        let _ = v4.get_array();
    });
    assert_panics(|| {
        let _ = v4.get_keys();
    });
    assert_panics(|| {
        let _ = v4.get_values();
    });
    assert_panics(|| {
        let _ = v4.get_obj();
    });

    // array of mixed values
    let mut v5 = UniValue::default();
    assert!(v5.read(b"[true, 10]".as_ptr(), 9));
    let arr = v5.get_array();
    let vals = arr.values();
    assert_panics(|| {
        let _ = vals[0].get_int();
    });
    assert!(vals[0].get_bool());
    assert_eq!(vals[1].get_int(), 10);
    assert_panics(|| {
        let _ = vals[1].get_bool();
    });
}

#[traced_test]
fn univalue_set() {
    let mut v = UniValue::new(uni_value::VType::VSTR, Some("foo"));
    v.clear();
    assert!(v.is_null());
    assert_eq!(v.get_val_str(), "");

    assert!(v.set_object());
    assert!(v.is_object());
    assert_eq!(v.size(), 0);
    assert_eq!(v.get_type(), uni_value::VType::VOBJ);
    assert!(v.empty());

    assert!(v.set_array());
    assert!(v.is_array());
    assert_eq!(v.size(), 0);

    assert!(v.set_str("zum"));
    assert!(v.is_str());
    assert_eq!(v.get_val_str(), "zum");

    assert!(v.set_float(-1.01));
    assert!(v.is_num());
    assert_eq!(v.get_val_str(), "-1.0100000000000000");

    assert!(v.set_int(1023i32));
    assert!(v.is_num());
    assert_eq!(v.get_val_str(), "1023");

    assert!(v.set_int(-1023i64));
    assert_eq!(v.get_val_str(), "-1023");

    assert!(v.set_int(1023u64));
    assert_eq!(v.get_val_str(), "1023");

    assert!(v.set_num_str(&"-688".to_string()));
    assert_eq!(v.get_val_str(), "-688");

    assert!(v.set_bool(false));
    assert!(v.is_bool());
    assert!(!v.is_true());
    assert!(v.is_false());
    assert!(!v.get_bool());

    assert!(v.set_bool(true));
    assert!(v.is_bool());
    assert!(v.is_true());
    assert!(!v.is_false());
    assert!(v.get_bool());

    assert!(!v.set_num_str(&"zombocom".to_string()));

    v.set_null();
    assert!(v.is_null());
}

#[traced_test]
fn univalue_array() {
    let mut arr = UniValue::new(uni_value::VType::VARR, None);

    arr.push_back(&UniValue::from(1023i64));
    arr.push_back(&"zippy".to_string());
    arr.push_back(&"pippy");

    let mut vec = vec![];
    vec.push(UniValue::from("boing"));
    vec.push(UniValue::from("going"));
    arr.push_backv(&vec);

    arr.push_back(&400u64);
    arr.push_back(&(-400i64));
    arr.push_back(&(-401i32));
    arr.push_back(&-40.1f64);
    arr.push_back(&true);

    assert!(!arr.empty());
    assert_eq!(arr.size(), 10);

    assert_eq!(arr[0].get_val_str(), "1023");
    assert_eq!(arr[0].get_type(), uni_value::VType::VNUM);
    assert_eq!(arr[1].get_val_str(), "zippy");
    assert_eq!(arr[2].get_val_str(), "pippy");
    assert_eq!(arr[3].get_val_str(), "boing");
    assert_eq!(arr[4].get_val_str(), "going");
    assert_eq!(arr[5].get_val_str(), "400");
    assert_eq!(arr[6].get_val_str(), "-400");
    assert_eq!(arr[7].get_val_str(), "-401");
    assert_eq!(arr[8].get_val_str(), "-40.1");
    assert_eq!(arr[9].get_val_str(), "1");
    assert!(arr[999].is_null());

    arr.clear();
    assert!(arr.empty());
    assert_eq!(arr.size(), 0);
}

#[traced_test]
fn univalue_object() {
    let mut obj = UniValue::new(uni_value::VType::VOBJ, None);

    obj.pushkv("age", 100i64);
    obj.pushkv("first", "John");
    obj.pushkv("last", "Smith");
    obj.pushkv("distance", 25i64);
    obj.pushkv("time", 3600u64);
    obj.pushkv("calories", 12i32);
    obj.pushkv("temperature", 90.012f64);
    obj.pushkv("moon", true);
    obj.pushkv("spoon", false);

    let mut obj2 = UniValue::new(uni_value::VType::VOBJ, None);
    obj2.pushkv("cat1", 9000i64);
    obj2.pushkv("cat2", 12345i64);
    obj.push_kvs(&obj2);

    assert!(!obj.empty());
    assert_eq!(obj.size(), 11);

    assert_eq!(obj["age"].get_val_str(), "100");
    assert_eq!(obj["first"].get_val_str(), "John");
    assert_eq!(obj["last"].get_val_str(), "Smith");
    assert_eq!(obj["distance"].get_val_str(), "25");
    assert_eq!(obj["time"].get_val_str(), "3600");
    assert_eq!(obj["calories"].get_val_str(), "12");
    assert_eq!(obj["temperature"].get_val_str(), "90.012");
    assert_eq!(obj["moon"].get_val_str(), "1");
    assert_eq!(obj["spoon"].get_val_str(), "0");
    assert_eq!(obj["cat1"].get_val_str(), "9000");
    assert_eq!(obj["cat2"].get_val_str(), "12345");
    assert!(obj["nyuknyuknyuk"].is_null());

    for k in [
        "age", "first", "last", "distance", "time", "calories", "temperature", "moon", "spoon",
        "cat1", "cat2",
    ] {
        assert!(obj.exists(k));
    }
    assert!(!obj.exists("nyuknyuknyuk"));

    // shape validation
    let mut obj_types = HashMap::<String, uni_value::VType>::new();
    obj_types.insert("age".into(), uni_value::VType::VNUM);
    obj_types.insert("first".into(), uni_value::VType::VSTR);
    obj_types.insert("last".into(), uni_value::VType::VSTR);
    obj_types.insert("distance".into(), uni_value::VType::VNUM);
    obj_types.insert("time".into(), uni_value::VType::VNUM);
    obj_types.insert("calories".into(), uni_value::VType::VNUM);
    obj_types.insert("temperature".into(), uni_value::VType::VNUM);
    obj_types.insert("moon".into(), uni_value::VType::VBOOL);
    obj_types.insert("spoon".into(), uni_value::VType::VBOOL);
    obj_types.insert("cat1".into(), uni_value::VType::VNUM);
    obj_types.insert("cat2".into(), uni_value::VType::VNUM);
    assert!(obj.check_object(&obj_types));

    // wrong type
    obj_types.insert("cat2".into(), uni_value::VType::VSTR);
    assert!(!obj.check_object(&obj_types));

    /* ---------- clear / rebuild ---------- */
    obj.clear();
    assert!(obj.empty());
    assert_eq!(obj.size(), 0);
    assert_eq!(obj.get_type(), uni_value::VType::VNULL);

    obj.set_object();
    obj.pushkv("age", 42i64);
    assert_eq!(obj.size(), 1);
    assert_eq!(obj["age"].get_val_str(), "42");

    obj.pushkv("age", 43i64); // replace
    assert_eq!(obj.size(), 1);
    assert_eq!(obj["age"].get_val_str(), "43");

    obj.pushkv("name", "foo bar");

    let mut kv = HashMap::<String, UniValue>::new();
    obj.get_obj_map(&mut kv);
    assert_eq!(kv["age"].get_val_str(), "43");
    assert_eq!(kv["name"].get_val_str(), "foo bar");
}

const JSON1: &str = r#"[1.10000000,{"key1":"str\u0000","key2":800,"key3":{"name":"martian http://test.com"}}]"#;

#[traced_test]
fn univalue_readwrite() {
    // from const str
    let (ok, v) = read_from(JSON1);
    assert!(ok);

    // from owned String
    let json_owned = JSON1.to_string();
    let (ok2, v2) = read_from(&json_owned);
    assert!(ok2);
    assert!(v2.is_array());
    assert_eq!(v2.size(), 2);
    assert_eq!(v2[0].get_val_str(), "1.10000000");

    let obj = &v2[1];
    assert!(obj.is_object());
    assert_eq!(obj.size(), 3);
    assert!(obj["key1"].is_str());

    let mut correct_value = String::from("str");
    correct_value.push('\0');
    assert_eq!(*obj["key1"].get_val_str(), correct_value);
    assert!(obj["key2"].is_num());
    assert_eq!(obj["key2"].get_val_str(), "800");
    assert!(obj["key3"].is_object());

    assert_eq!(json_owned, v2.write(None, None));

    /* ---------- trailing‑junk detection ---------- */
    let mut v3 = UniValue::default();
    assert!(v3.read(b"  {}\n  ".as_ptr(), 7));
    assert!(v3.is_object());
    assert!(v3.read(b"  []\n  ".as_ptr(), 7));
    assert!(v3.is_array());

    assert!(!v3.read(b"@{}".as_ptr(), 3));
    assert!(!v3.read(b"{} garbage".as_ptr(), 9));
    assert!(!v3.read(b"[]{}".as_ptr(), 4));
    assert!(!v3.read(b"{}[]".as_ptr(), 4));
    assert!(!v3.read(b"{} 42".as_ptr(), 5));
}

#[traced_test]
fn unescape_unicode_test() {
    let mut val = UniValue::default();

    // Escaped quote
    assert!(val.read(b"[\"\\u0022\"]".as_ptr(), 10));
    assert_eq!(val[0].get_str(), "\"");

    // Escaped BMP 2‑byte
    assert!(val.read(b"[\"\\u0191\"]".as_ptr(), 10));
    assert_eq!(val[0].get_str(), "\u{0191}");

    // Escaped BMP 3‑byte
    assert!(val.read(b"[\"\\u2191\"]".as_ptr(), 10));
    assert_eq!(val[0].get_str(), "\u{2191}");

    // Escaped supplementary plane (surrogate pair)
    assert!(val.read(b"[\"\\ud834\\udd61\"]".as_ptr(), 18));
    assert_eq!(val[0].get_str(), "\u{1D161}");
}
