// ---------------- [ File: bitcoin-net/src/url.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/url.h]

pub type UrlDecodeFn = fn(url_encoded: &String) -> String;

lazy_static!{
    /*
    UrlDecodeFn urlDecode;
    extern UrlDecodeFn* const URL_DECODE;
    */
}

//-------------------------------------------[.cpp/bitcoin/src/util/url.cpp]
pub fn url_decode(url_encoded: &String) -> String {
    
    todo!();
        /*
            std::string res;
        if (!urlEncoded.empty()) {
            char *decoded = evhttp_uridecode(urlEncoded.c_str(), false, nullptr);
            if (decoded) {
                res = std::string(decoded);
                free(decoded);
            }
        }
        return res;
        */
}
