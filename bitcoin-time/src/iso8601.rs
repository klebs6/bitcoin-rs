// ---------------- [ File: bitcoin-time/src/iso8601.rs ]
crate::ix!();

/**
  | ISO 8601 formatting is preferred. Use
  | the FormatISO8601{DateTime,Date}
  | helper functions if possible.
  |
  */
pub fn format_iso8601date_time(n_time: i64) -> String {
    
    todo!();
        /*
            struct tm ts;
        time_t time_val = nTime;
    #ifdef HAVE_GMTIME_R
        if (gmtime_r(&time_val, &ts) == nullptr) {
    #else
        if (gmtime_s(&ts, &time_val) != 0) {
    #endif
            return {};
        }
        return strprintf("%04i-%02i-%02iT%02i:%02i:%02iZ", ts.tm_year + 1900, ts.tm_mon + 1, ts.tm_mday, ts.tm_hour, ts.tm_min, ts.tm_sec);
        */
}

pub fn format_iso8601date(n_time: i64) -> String {
    
    todo!();
        /*
            struct tm ts;
        time_t time_val = nTime;
    #ifdef HAVE_GMTIME_R
        if (gmtime_r(&time_val, &ts) == nullptr) {
    #else
        if (gmtime_s(&ts, &time_val) != 0) {
    #endif
            return {};
        }
        return strprintf("%04i-%02i-%02i", ts.tm_year + 1900, ts.tm_mon + 1, ts.tm_mday);
        */
}

pub fn parse_iso8601date_time(str_: &String) -> i64 {
    
    todo!();
        /*
            static const boost::posix_time::ptime epoch = boost::posix_time::from_time_t(0);
        static const std::locale loc(std::locale::classic(),
            new boost::posix_time::time_input_facet("%Y-%m-%dT%H:%M:%SZ"));
        std::istringstream iss(str);
        iss.imbue(loc);
        boost::posix_time::ptime ptime(boost::date_time::not_a_date_time);
        iss >> ptime;
        if (ptime.is_not_a_date_time() || epoch > ptime)
            return 0;
        return (ptime - epoch).total_seconds();
        */
}
