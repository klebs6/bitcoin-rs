// ---------------- [ File: bitcoinsecp256k1-group/src/ge_const.rs ]
crate::ix!();

#[macro_export]
macro_rules! ge_const {
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
        Ge {
            x: fe_const!(($a), ($b), ($c), ($d), ($e), ($f), ($g), ($h)),
            y: fe_const!(($i), ($j), ($k), ($l), ($m), ($n), ($o), ($p)),
            infinity: 0,
        }
    };
}

#[macro_export]
macro_rules! ge_const_infinity {
    () => {
        Ge {
            x: fe_const!(0, 0, 0, 0, 0, 0, 0, 0),
            y: fe_const!(0, 0, 0, 0, 0, 0, 0, 0),
            infinity: 1,
        }
    };
}
