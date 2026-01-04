// ---------------- [ File: bitcoinsecp256k1-group/src/gej_const.rs ]
crate::ix!();

#[macro_export]
macro_rules! gej_const {
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
        Gej {
            x: fe_const!(($a), ($b), ($c), ($d), ($e), ($f), ($g), ($h)),
            y: fe_const!(($i), ($j), ($k), ($l), ($m), ($n), ($o), ($p)),
            z: fe_const!(0, 0, 0, 0, 0, 0, 0, 1),
            infinity: 0,
        }
    };
}

#[macro_export]
macro_rules! gej_const_infinity {
    () => {
        Gej {
            x: fe_const!(0, 0, 0, 0, 0, 0, 0, 0),
            y: fe_const!(0, 0, 0, 0, 0, 0, 0, 0),
            z: fe_const!(0, 0, 0, 0, 0, 0, 0, 0),
            infinity: 1,
        }
    };
}
