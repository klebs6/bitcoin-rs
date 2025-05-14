// ---------------- [ File: bitcoin-test/src/test_scriptnum.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/scriptnum_tests.cpp]

#[cfg(test)]
#[fixture(BasicTestingSetup)]
pub mod scriptnum_tests {

    /**
      | A selection of numbers that do not trigger
      | int64_t overflow when added/subtracted.
      |
      */
    pub const values:  &[i64] = &[ 0, 1, -2, 127, 128, -255, 256, (1 << 15) - 1, -(1 << 16), (1 << 24) - 1, (1 << 31), 1 - (1 << 32), 1 << 40 ];
    pub const offsets: &[i64] = &[ 1, 0x79, 0x80, 0x81, 0xFF, 0x7FFF, 0x8000, 0xFFFF, 0x10000];

    pub fn verify(
            bignum:    &ScriptNum10,
            scriptnum: &ScriptNum) -> bool {
        
        todo!();
            /*
                return bignum.getvch() == scriptnum.getvch() && bignum.getint() == scriptnum.getint();
            */
    }

    pub fn check_create_vch(num: &i64)  {
        
        todo!();
            /*
                CScriptNum10 bignum(num);
            CScriptNum scriptnum(num);
            BOOST_CHECK(verify(bignum, scriptnum));

            CScriptNum10 bignum2(bignum.getvch(), false);
            CScriptNum scriptnum2(scriptnum.getvch(), false);
            BOOST_CHECK(verify(bignum2, scriptnum2));

            CScriptNum10 bignum3(scriptnum2.getvch(), false);
            CScriptNum scriptnum3(bignum2.getvch(), false);
            BOOST_CHECK(verify(bignum3, scriptnum3));
            */
    }

    pub fn check_create_int(num: &i64)  {
        
        todo!();
            /*
                CScriptNum10 bignum(num);
            CScriptNum scriptnum(num);
            BOOST_CHECK(verify(bignum, scriptnum));
            BOOST_CHECK(verify(CScriptNum10(bignum.getint()), CScriptNum(scriptnum.getint())));
            BOOST_CHECK(verify(CScriptNum10(scriptnum.getint()), CScriptNum(bignum.getint())));
            BOOST_CHECK(verify(CScriptNum10(CScriptNum10(scriptnum.getint()).getint()), CScriptNum(CScriptNum(bignum.getint()).getint())));
            */
    }

    pub fn check_add(
            num1: &i64,
            num2: &i64)  {
        
        todo!();
            /*
                const CScriptNum10 bignum1(num1);
            const CScriptNum10 bignum2(num2);
            const CScriptNum scriptnum1(num1);
            const CScriptNum scriptnum2(num2);
            CScriptNum10 bignum3(num1);
            CScriptNum10 bignum4(num1);
            CScriptNum scriptnum3(num1);
            CScriptNum scriptnum4(num1);

            // int64_t overflow is undefined.
            bool invalid = (((num2 > 0) && (num1 > (std::numeric_limits<int64_t>::max() - num2))) ||
                            ((num2 < 0) && (num1 < (std::numeric_limits<int64_t>::min() - num2))));
            if (!invalid)
            {
                BOOST_CHECK(verify(bignum1 + bignum2, scriptnum1 + scriptnum2));
                BOOST_CHECK(verify(bignum1 + bignum2, scriptnum1 + num2));
                BOOST_CHECK(verify(bignum1 + bignum2, scriptnum2 + num1));
            }
            */
    }

    pub fn check_negate(num: &i64)  {
        
        todo!();
            /*
                const CScriptNum10 bignum(num);
            const CScriptNum scriptnum(num);

            // -INT64_MIN is undefined
            if (num != std::numeric_limits<int64_t>::min())
                BOOST_CHECK(verify(-bignum, -scriptnum));
            */
    }

    pub fn check_subtract(
            num1: &i64,
            num2: &i64)  {
        
        todo!();
            /*
                const CScriptNum10 bignum1(num1);
            const CScriptNum10 bignum2(num2);
            const CScriptNum scriptnum1(num1);
            const CScriptNum scriptnum2(num2);

            // int64_t overflow is undefined.
            bool invalid = ((num2 > 0 && num1 < std::numeric_limits<int64_t>::min() + num2) ||
                            (num2 < 0 && num1 > std::numeric_limits<int64_t>::max() + num2));
            if (!invalid)
            {
                BOOST_CHECK(verify(bignum1 - bignum2, scriptnum1 - scriptnum2));
                BOOST_CHECK(verify(bignum1 - bignum2, scriptnum1 - num2));
            }

            invalid = ((num1 > 0 && num2 < std::numeric_limits<int64_t>::min() + num1) ||
                       (num1 < 0 && num2 > std::numeric_limits<int64_t>::max() + num1));
            if (!invalid)
            {
                BOOST_CHECK(verify(bignum2 - bignum1, scriptnum2 - scriptnum1));
                BOOST_CHECK(verify(bignum2 - bignum1, scriptnum2 - num1));
            }
            */
    }

    pub fn check_compare(
            num1: &i64,
            num2: &i64)  {
        
        todo!();
            /*
                const CScriptNum10 bignum1(num1);
            const CScriptNum10 bignum2(num2);
            const CScriptNum scriptnum1(num1);
            const CScriptNum scriptnum2(num2);

            BOOST_CHECK((bignum1 == bignum1) == (scriptnum1 == scriptnum1));
            BOOST_CHECK((bignum1 != bignum1) ==  (scriptnum1 != scriptnum1));
            BOOST_CHECK((bignum1 < bignum1) ==  (scriptnum1 < scriptnum1));
            BOOST_CHECK((bignum1 > bignum1) ==  (scriptnum1 > scriptnum1));
            BOOST_CHECK((bignum1 >= bignum1) ==  (scriptnum1 >= scriptnum1));
            BOOST_CHECK((bignum1 <= bignum1) ==  (scriptnum1 <= scriptnum1));

            BOOST_CHECK((bignum1 == bignum1) == (scriptnum1 == num1));
            BOOST_CHECK((bignum1 != bignum1) ==  (scriptnum1 != num1));
            BOOST_CHECK((bignum1 < bignum1) ==  (scriptnum1 < num1));
            BOOST_CHECK((bignum1 > bignum1) ==  (scriptnum1 > num1));
            BOOST_CHECK((bignum1 >= bignum1) ==  (scriptnum1 >= num1));
            BOOST_CHECK((bignum1 <= bignum1) ==  (scriptnum1 <= num1));

            BOOST_CHECK((bignum1 == bignum2) ==  (scriptnum1 == scriptnum2));
            BOOST_CHECK((bignum1 != bignum2) ==  (scriptnum1 != scriptnum2));
            BOOST_CHECK((bignum1 < bignum2) ==  (scriptnum1 < scriptnum2));
            BOOST_CHECK((bignum1 > bignum2) ==  (scriptnum1 > scriptnum2));
            BOOST_CHECK((bignum1 >= bignum2) ==  (scriptnum1 >= scriptnum2));
            BOOST_CHECK((bignum1 <= bignum2) ==  (scriptnum1 <= scriptnum2));

            BOOST_CHECK((bignum1 == bignum2) ==  (scriptnum1 == num2));
            BOOST_CHECK((bignum1 != bignum2) ==  (scriptnum1 != num2));
            BOOST_CHECK((bignum1 < bignum2) ==  (scriptnum1 < num2));
            BOOST_CHECK((bignum1 > bignum2) ==  (scriptnum1 > num2));
            BOOST_CHECK((bignum1 >= bignum2) ==  (scriptnum1 >= num2));
            BOOST_CHECK((bignum1 <= bignum2) ==  (scriptnum1 <= num2));
            */
    }

    pub fn run_create(num: &i64)  {
        
        todo!();
            /*
                CheckCreateInt(num);
            CScriptNum scriptnum(num);
            if (scriptnum.getvch().size() <= CScriptNum::nDefaultMaxNumSize)
                CheckCreateVch(num);
            else
            {
                BOOST_CHECK_THROW (CheckCreateVch(num), scriptnum10_error);
            }
            */
    }

    pub fn run_operators(
            num1: &i64,
            num2: &i64)  {
        
        todo!();
            /*
                CheckAdd(num1, num2);
            CheckSubtract(num1, num2);
            CheckNegate(num1);
            CheckCompare(num1, num2);
            */
    }

    #[test] fn creation() {
        todo!();
        /*
        
            for(size_t i = 0; i < std::size(values); ++i)
            {
                for(size_t j = 0; j < std::size(offsets); ++j)
                {
                    RunCreate(values[i]);
                    RunCreate(values[i] + offsets[j]);
                    RunCreate(values[i] - offsets[j]);
                }
            }

        */
    }

    #[test] fn operators() {
        todo!();
        /*
        
            for(size_t i = 0; i < std::size(values); ++i)
            {
                for(size_t j = 0; j < std::size(offsets); ++j)
                {
                    RunOperators(values[i], values[i]);
                    RunOperators(values[i], -values[i]);
                    RunOperators(values[i], values[j]);
                    RunOperators(values[i], -values[j]);
                    RunOperators(values[i] + values[j], values[j]);
                    RunOperators(values[i] + values[j], -values[j]);
                    RunOperators(values[i] - values[j], values[j]);
                    RunOperators(values[i] - values[j], -values[j]);
                    RunOperators(values[i] + values[j], values[i] + values[j]);
                    RunOperators(values[i] + values[j], values[i] - values[j]);
                    RunOperators(values[i] - values[j], values[i] + values[j]);
                    RunOperators(values[i] - values[j], values[i] - values[j]);
                }
            }

        */
    }
}
