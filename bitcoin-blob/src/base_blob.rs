// ---------------- [ File: bitcoin-blob/src/base_blob.rs ]
crate::ix!();

#[macro_export]
macro_rules! base_blob {
    (
        $blob_ty:ident,
        $bits:expr,
        $bytes:expr
    ) => {

        define_base_blob_struct!{
            $blob_ty,    
            $bits,   
            $bytes
        }

        define_base_blob_iter!{
            $blob_ty, 
            $bits, 
            $bytes 
        }

        define_base_blob_serialization!{
            $blob_ty, 
            $bits, 
            $bytes 
        }

        define_base_blob_from_bytes!{
            $blob_ty, 
            $bits, 
            $bytes 
        }

        define_base_blob_ord_eq!{
            $blob_ty, 
            $bits, 
            $bytes 
        }

        define_base_blob_hex!{
            $blob_ty, 
            $bits, 
            $bytes 
        }

        define_base_blob_basic!{
            $blob_ty, 
            $bits, 
            $bytes 
        }
    }
}

base_blob!(BaseBlob8,    8,   1);
base_blob!(BaseBlob64,   64,  8);
base_blob!(BaseBlob128,  128, 16);
base_blob!(BaseBlob160,  160, 20);
base_blob!(BaseBlob256,  256, 32);
