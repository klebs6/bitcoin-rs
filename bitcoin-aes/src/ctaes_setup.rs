// ---------------- [ File: bitcoin-aes/src/ctaes_setup.rs ]
crate::ix!();

/**
  | Expand the cipher key into the key schedule.
  | state must be a pointer to an array of
  | size nrounds + 1. key must be a pointer
  | to 4 * nkeywords bytes.
  | 
  | AES128 uses nkeywords = 4, nrounds =
  | 10
  | 
  | AES192 uses nkeywords = 6, nrounds =
  | 12
  | 
  | AES256 uses nkeywords = 8, nrounds =
  | 14
  |
  */
pub fn aes_setup(
        rounds:    *mut AES_state,
        key:       *const u8,
        nkeywords: i32,
        nrounds:   i32)  {
    
    todo!();
        /*
            int i;

        /* The one-byte round constant */
        AES_state rcon = {{1,0,0,0,0,0,0,0}};
        /* The number of the word being generated, modulo nkeywords */
        int pos = 0;
        /* The column representing the word currently being processed */
        AES_state column;

        for (i = 0; i < nrounds + 1; i++) {
            int b;
            for (b = 0; b < 8; b++) {
                rounds[i].slice[b] = 0;
            }
        }

        /* The first nkeywords round columns are just taken from the key directly. */
        for (i = 0; i < nkeywords; i++) {
            int r;
            for (r = 0; r < 4; r++) {
                LoadByte(&rounds[i >> 2], *(key++), r, i & 3);
            }
        }

        GetOneColumn(&column, &rounds[(nkeywords - 1) >> 2], (nkeywords - 1) & 3);

        for (i = nkeywords; i < 4 * (nrounds + 1); i++) {
            /* Transform column */
            if (pos == 0) {
                SubBytes(&column, 0);
                KeySetupTransform(&column, &rcon);
                MultX(&rcon);
            } else if (nkeywords > 6 && pos == 4) {
                SubBytes(&column, 0);
            }
            if (++pos == nkeywords) pos = 0;
            KeySetupColumnMix(&column, &rounds[i >> 2], &rounds[(i - nkeywords) >> 2], i & 3, (i - nkeywords) & 3);
        }
        */
}
