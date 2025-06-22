// ---------------- [ File: bitcoin-univalue/src/output_escape.rs ]
crate::ix!();

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
