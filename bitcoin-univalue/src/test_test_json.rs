/*!
  | Test program that can be called by the JSON
  | test suite at
  | https://github.com/nst/JSONTestSuite.
  |
  | It reads JSON input from stdin and exits with
  | code 0 if it can be parsed successfully. It
  | also pretty prints the parsed JSON value to
  | stdout.
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/univalue/test/test_json.cpp]

pub fn univalue_test_json_main(
        argc: i32,
        argv: &[*mut u8]) -> i32 {
    
    todo!();
        /*
            UniValue val;
        if (val.read(string(istreambuf_iterator<char>(cin),
                            istreambuf_iterator<char>()))) {
            cout << val.write(1 /* prettyIndent */, 4 /* indentLevel */) << endl;
            return 0;
        } else {
            cerr << "JSON Parse Error." << endl;
            return 1;
        }
        */
}
