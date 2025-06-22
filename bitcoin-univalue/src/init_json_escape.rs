crate::ix!();

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
