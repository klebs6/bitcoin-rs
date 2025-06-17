crate::ix!();

pub fn aes_encrypt(
        rounds:   *const AES_state,
        nrounds:  i32,
        cipher16: *mut u8,
        plain16:  *const u8)  {
    
    todo!();
        /*
            AES_state s = {{0}};
        int round;

        LoadBytes(&s, plain16);
        AddRoundKey(&s, rounds++);

        for (round = 1; round < nrounds; round++) {
            SubBytes(&s, 0);
            ShiftRows(&s);
            MixColumns(&s, 0);
            AddRoundKey(&s, rounds++);
        }

        SubBytes(&s, 0);
        ShiftRows(&s);
        AddRoundKey(&s, rounds);

        SaveBytes(cipher16, &s);
        */
}

pub fn aes_decrypt(
        rounds:   *const AES_state,
        nrounds:  i32,
        plain16:  *mut u8,
        cipher16: *const u8)  {
    
    todo!();
        /*
            /* Most AES decryption implementations use the alternate scheme
         * (the Equivalent Inverse Cipher), which allows for more code reuse between
         * the encryption and decryption code, but requires separate setup for both.
         */
        AES_state s = {{0}};
        int round;

        rounds += nrounds;

        LoadBytes(&s, cipher16);
        AddRoundKey(&s, rounds--);

        for (round = 1; round < nrounds; round++) {
            InvShiftRows(&s);
            SubBytes(&s, 1);
            AddRoundKey(&s, rounds--);
            MixColumns(&s, 1);
        }

        InvShiftRows(&s);
        SubBytes(&s, 1);
        AddRoundKey(&s, rounds);

        SaveBytes(plain16, &s);
        */
}
