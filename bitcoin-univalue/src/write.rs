// ---------------- [ File: bitcoin-univalue/src/write.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/univalue/lib/univalue_write.cpp]

pub fn json_escape(ins: &String) -> String {
    
    todo!();
        /*
            std::string outS;
        outS.reserve(inS.size() * 2);

        for (unsigned int i = 0; i < inS.size(); i++) {
            unsigned char ch = static_cast<unsigned char>(inS[i]);
            const char *escStr = escapes[ch];

            if (escStr)
                outS += escStr;
            else
                outS += static_cast<char>(ch);
        }

        return outS;
        */
}

pub fn indent_str(
        pretty_indent: u32,
        indent_level:  u32,
        s:             &mut String)  {
    
    todo!();
        /*
            s.append(prettyIndent * indentLevel, ' ');
        */
}

impl UniValue {
    
    pub fn write(&self, 
        pretty_indent: Option<u32>,
        indent_level:  Option<u32>) -> String {

        let pretty_indent: u32 = pretty_indent.unwrap_or(0);
        let indent_level: u32 = indent_level.unwrap_or(0);
        
        todo!();
        /*
            std::string s;
        s.reserve(1024);

        unsigned int modIndent = indentLevel;
        if (modIndent == 0)
            modIndent = 1;

        switch (typ) {
        case VNULL:
            s += "null";
            break;
        case VOBJ:
            writeObject(prettyIndent, modIndent, s);
            break;
        case VARR:
            writeArray(prettyIndent, modIndent, s);
            break;
        case VSTR:
            s += "\"" + json_escape(val) + "\"";
            break;
        case VNUM:
            s += val;
            break;
        case VBOOL:
            s += (val == "1" ? "true" : "false");
            break;
        }

        return s;
        */
    }
    
    pub fn write_array(&self, 
        pretty_indent: u32,
        indent_level:  u32,
        s:             &mut String)  {
        
        todo!();
        /*
            s += "[";
        if (prettyIndent)
            s += "\n";

        for (unsigned int i = 0; i < values.size(); i++) {
            if (prettyIndent)
                indentStr(prettyIndent, indentLevel, s);
            s += values[i].write(prettyIndent, indentLevel + 1);
            if (i != (values.size() - 1)) {
                s += ",";
            }
            if (prettyIndent)
                s += "\n";
        }

        if (prettyIndent)
            indentStr(prettyIndent, indentLevel - 1, s);
        s += "]";
        */
    }
    
    pub fn write_object(&self, 
        pretty_indent: u32,
        indent_level:  u32,
        s:             &mut String)  {
        
        todo!();
        /*
            s += "{";
        if (prettyIndent)
            s += "\n";

        for (unsigned int i = 0; i < keys.size(); i++) {
            if (prettyIndent)
                indentStr(prettyIndent, indentLevel, s);
            s += "\"" + json_escape(keys[i]) + "\":";
            if (prettyIndent)
                s += " ";
            s += values.at(i).write(prettyIndent, indentLevel + 1);
            if (i != (values.size() - 1))
                s += ",";
            if (prettyIndent)
                s += "\n";
        }

        if (prettyIndent)
            indentStr(prettyIndent, indentLevel - 1, s);
        s += "}";
        */
    }
}
