/*!
  | To re-create univalue_escapes.h:
  | $ g++ -o gen gen.cpp
  | $ ./gen > univalue_escapes.h
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/univalue/gen/gen.cpp]

lazy_static!{
    /*
    static bool initEscapes;
    static std::string escapes[256];
    */
}

pub fn init_json_escape()  {
    
    todo!();
        /*
            // Escape all lower control characters (some get overridden with smaller sequences below)
        for (int ch=0x00; ch<0x20; ++ch) {
            char tmpbuf[20];
            snprintf(tmpbuf, sizeof(tmpbuf), "\\u%04x", ch);
            escapes[ch] = std::string(tmpbuf);
        }

        escapes[(int)'"'] = "\\\"";
        escapes[(int)'\\'] = "\\\\";
        escapes[(int)'\b'] = "\\b";
        escapes[(int)'\f'] = "\\f";
        escapes[(int)'\n'] = "\\n";
        escapes[(int)'\r'] = "\\r";
        escapes[(int)'\t'] = "\\t";
        escapes[(int)'\x7f'] = "\\u007f"; // U+007F DELETE

        initEscapes = true;
        */
}

pub fn output_escape()  {
    
    todo!();
        /*
            printf( "// Automatically generated file. Do not modify.\n"
            "#ifndef BITCOIN_UNIVALUE_UNIVALUE_ESCAPES_H\n"
            "#define BITCOIN_UNIVALUE_UNIVALUE_ESCAPES_H\n"
            "static const char *escapes[256] = {\n");

        for (unsigned int i = 0; i < 256; i++) {
            if (escapes[i].empty()) {
                printf("\tnullptr,\n");
            } else {
                printf("\t\"");

                unsigned int si;
                for (si = 0; si < escapes[i].size(); si++) {
                    char ch = escapes[i][si];
                    switch (ch) {
                    case '"':
                        printf("\\\"");
                        break;
                    case '\\':
                        printf("\\\\");
                        break;
                    default:
                        printf("%c", escapes[i][si]);
                        break;
                    }
                }

                printf("\",\n");
            }
        }

        printf( "};\n"
            "#endif // BITCOIN_UNIVALUE_UNIVALUE_ESCAPES_H\n");
        */
}

pub fn univalue_gen_main(
        argc: i32,
        argv: &[*mut u8]) -> i32 {
    
    todo!();
        /*
        initJsonEscape();
        outputEscape();
        return 0;
        */
}
