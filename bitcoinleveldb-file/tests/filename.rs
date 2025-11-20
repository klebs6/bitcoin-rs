// ---------------- [ File: bitcoinleveldb-file/tests/filename.rs ]
use bitcoinleveldb_file::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/filename_test.cc]

struct FileNameTest {}

#[traced_test]
fn file_name_test_parse() {
    use tracing::info;

    let mut number: u64 = 0;
    let mut ty: FileType = FileType::LogFile;

    struct Case {
        fname: &'static str,
        number: u64,
        ty: FileType,
    }

    // Successful parses
    let cases = [
        Case { fname: "100.log", number: 100, ty: FileType::LogFile },
        Case { fname: "0.log", number: 0, ty: FileType::LogFile },
        Case { fname: "0.sst", number: 0, ty: FileType::TableFile },
        Case { fname: "0.ldb", number: 0, ty: FileType::TableFile },
        Case { fname: "CURRENT", number: 0, ty: FileType::CurrentFile },
        Case { fname: "LOCK", number: 0, ty: FileType::DBLockFile },
        Case { fname: "MANIFEST-2", number: 2, ty: FileType::DescriptorFile },
        Case { fname: "MANIFEST-7", number: 7, ty: FileType::DescriptorFile },
        Case { fname: "LOG", number: 0, ty: FileType::InfoLogFile },
        Case { fname: "LOG.old", number: 0, ty: FileType::InfoLogFile },
        Case { fname: "18446744073709551615.log", number: u64::MAX, ty: FileType::LogFile },
    ];

    for c in &cases {
        let f = c.fname.to_string();
        number = 0;
        ty = FileType::LogFile; // seed with something
        let ok = parse_file_name(&f, &mut number as *mut u64, &mut ty as *mut FileType);
        assert!(ok, "expected success parsing: {}", f);
        assert_eq!(number, c.number, "number mismatch for {}", f);
        assert!(std::mem::discriminant(&ty) == std::mem::discriminant(&c.ty), "type mismatch for {}", f);
        info!("parsed OK: {}", f);
    }

    // Errors
    let errors = [
        "",
        "foo",
        "foo-dx-100.log",
        ".log",
        "",
        "manifest",
        "CURREN",
        "CURRENTX",
        "MANIFES",
        "MANIFEST",
        "MANIFEST-",
        "XMANIFEST-3",
        "MANIFEST-3x",
        "LOC",
        "LOCKx",
        "LO",
        "LOGx",
        "18446744073709551616.log",
        "184467440737095516150.log",
        "100",
        "100.",
        "100.lop",
    ];

    for e in &errors {
        let f = e.to_string();
        number = 0;
        ty = FileType::LogFile;
        let ok = parse_file_name(&f, &mut number as *mut u64, &mut ty as *mut FileType);
        assert!(!ok, "expected failure parsing: {}", f);
        info!("correctly rejected: {}", f);
    }
}

#[traced_test]
fn file_name_test_construction() {
    use tracing::info;

    let mut number: u64 = 0;
    let mut ty: FileType = FileType::LogFile;

    // CURRENT
    let fname = current_file_name(&"foo".to_string());
    assert!(fname.len() >= 4);
    assert_eq!(&fname[0..4], "foo/");
    let rest = fname[4..].to_string();
    assert!(parse_file_name(&rest, &mut number as *mut u64, &mut ty as *mut FileType));
    assert_eq!(number, 0);
    assert!(std::mem::discriminant(&ty) == std::mem::discriminant(&FileType::CurrentFile));
    info!("current_file_name OK: {}", fname);

    // LOCK
    let fname = lock_file_name(&"foo".to_string());
    assert!(fname.len() >= 4);
    assert_eq!(&fname[0..4], "foo/");
    let rest = fname[4..].to_string();
    assert!(parse_file_name(&rest, &mut number as *mut u64, &mut ty as *mut FileType));
    assert_eq!(number, 0);
    assert!(std::mem::discriminant(&ty) == std::mem::discriminant(&FileType::DBLockFile));
    info!("lock_file_name OK: {}", fname);

    // LOG
    let fname = log_file_name(&"foo".to_string(), 192);
    assert!(fname.len() >= 4);
    assert_eq!(&fname[0..4], "foo/");
    let rest = fname[4..].to_string();
    assert!(parse_file_name(&rest, &mut number as *mut u64, &mut ty as *mut FileType));
    assert_eq!(number, 192);
    assert!(std::mem::discriminant(&ty) == std::mem::discriminant(&FileType::LogFile));
    info!("log_file_name OK: {}", fname);

    // Table (ldb)
    let fname = table_file_name(&"bar".to_string(), 200);
    assert!(fname.len() >= 4);
    assert_eq!(&fname[0..4], "bar/");
    let rest = fname[4..].to_string();
    assert!(parse_file_name(&rest, &mut number as *mut u64, &mut ty as *mut FileType));
    assert_eq!(number, 200);
    assert!(std::mem::discriminant(&ty) == std::mem::discriminant(&FileType::TableFile));
    info!("table_file_name OK: {}", fname);

    // Descriptor
    let fname = descriptor_file_name(&"bar".to_string(), 100);
    assert!(fname.len() >= 4);
    assert_eq!(&fname[0..4], "bar/");
    let rest = fname[4..].to_string();
    assert!(parse_file_name(&rest, &mut number as *mut u64, &mut ty as *mut FileType));
    assert_eq!(number, 100);
    assert!(std::mem::discriminant(&ty) == std::mem::discriminant(&FileType::DescriptorFile));
    info!("descriptor_file_name OK: {}", fname);

    // Temp
    let fname = temp_file_name(&"tmp".to_string(), 999);
    assert!(fname.len() >= 4);
    assert_eq!(&fname[0..4], "tmp/");
    let rest = fname[4..].to_string();
    assert!(parse_file_name(&rest, &mut number as *mut u64, &mut ty as *mut FileType));
    assert_eq!(number, 999);
    assert!(std::mem::discriminant(&ty) == std::mem::discriminant(&FileType::TempFile));
    info!("temp_file_name OK: {}", fname);

    // Info log
    let fname = info_log_file_name(&"foo".to_string());
    assert!(fname.len() >= 4);
    assert_eq!(&fname[0..4], "foo/");
    let rest = fname[4..].to_string();
    assert!(parse_file_name(&rest, &mut number as *mut u64, &mut ty as *mut FileType));
    assert_eq!(number, 0);
    assert!(std::mem::discriminant(&ty) == std::mem::discriminant(&FileType::InfoLogFile));
    info!("info_log_file_name OK: {}", fname);

    // Old info log
    let fname = old_info_log_file_name(&"foo".to_string());
    assert!(fname.len() >= 4);
    assert_eq!(&fname[0..4], "foo/");
    let rest = fname[4..].to_string();
    assert!(parse_file_name(&rest, &mut number as *mut u64, &mut ty as *mut FileType));
    assert_eq!(number, 0);
    assert!(std::mem::discriminant(&ty) == std::mem::discriminant(&FileType::InfoLogFile));
    info!("old_info_log_file_name OK: {}", fname);
}
