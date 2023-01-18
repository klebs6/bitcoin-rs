crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/main.cpp]

/**
  | Translate string to current locale
  | using Qt.
  |
  */
lazy_static!{
    /*
    extern const std::function<std::string(const char*)> G_TRANSLATION_FUN = [](const char* psz) {
        return QCoreApplication::translate("bitcoin-core", psz).toStdString();
    };
    */
}

lazy_static!{
    /*
    UrlDecodeFn* const URL_DECODE = urlDecode;
    */
}

pub fn qt_main(
        argc: i32,
        argv: &[*mut u8]) -> i32 {
    
    todo!();
        /*
            return GuiMain(argc, argv);
        */
}

