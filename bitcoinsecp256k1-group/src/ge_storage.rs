// ---------------- [ File: bitcoinsecp256k1-group/src/ge_storage.rs ]
crate::ix!();

pub struct GeStorage {
    pub x: FeStorage,
    pub y: FeStorage,
}

#[macro_export]
macro_rules! ge_storage_const {
    ($a:expr,
     $b:expr,
     $c:expr,
     $d:expr,
     $e:expr,
     $f:expr,
     $g:expr,
     $h:expr,
     $i:expr,
     $j:expr,
     $k:expr,
     $l:expr,
     $m:expr,
     $n:expr,
     $o:expr,
     $p:expr) => {
        GeStorage {
            x: fe_storage_const!(($a), ($b), ($c), ($d), ($e), ($f), ($g), ($h)),
            y: fe_storage_const!(($i), ($j), ($k), ($l), ($m), ($n), ($o), ($p)),
        }
    };
}

#[macro_export]
macro_rules! ge_storage_const_get {
    ($t:ident) => {
        fe_storage_const_get!($t.x), fe_storage_const_get!($t.y)
    };
}
