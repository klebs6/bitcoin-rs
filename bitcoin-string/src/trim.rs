// ---------------- [ File: bitcoin-string/src/string.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/string.h]
//-------------------------------------------[.cpp/bitcoin/src/util/string.cpp]

#[inline] pub fn trim_string(
        str_:    &String,
        pattern: Option<&str>) -> String {

    //note: removed \f and \v from the c++ default
    //pattern
    let pattern: &str = pattern.unwrap_or(" \n\r\t");

    todo!();
        /*
            std::string::size_type front = str.find_first_not_of(pattern);
        if (front == std::string::npos) {
            return std::string();
        }
        std::string::size_type end = str.find_last_not_of(pattern);
        return str.substr(front, end - front + 1);
        */
}

#[inline] pub fn remove_prefix(
        str_:   &String,
        prefix: &String) -> String {
    
    todo!();
        /*
            if (str.substr(0, prefix.size()) == prefix) {
            return str.substr(prefix.size());
        }
        return str;
        */
}

/**
  | Join a list of items
  | 
  | -----------
  | @param list
  | 
  | The list to join
  | ----------
  | @param separator
  | 
  | The separator
  | ----------
  | @param unary_op
  | 
  | Apply this operator to each item in the
  | list
  |
  */
lazy_static!{
    /*
    template <typename T, typename BaseType, typename UnaryOp>
    auto Join(const std::vector<T>& list, const BaseType& separator, UnaryOp unary_op)
        -> decltype(unary_op(list.at(0)))
    {
        decltype(unary_op(list.at(0))) ret;
        for (size_t i = 0; i < list.size(); ++i) {
            if (i > 0) ret += separator;
            ret += unary_op(list.at(i));
        }
        return ret;
    }
    */
}

pub fn join(
        list:      &Vec<String>,
        separator: &str) -> String {

    todo!();
        /*
            return Join(list, separator, [](const T& i) { return i; });
        */
}

/**
  | Create an unordered multi-line list
  | of items.
  |
  */
#[inline] pub fn make_unordered_list(items: &Vec<String>) -> String {
    
    todo!();
        /*
            return Join(items, "\n", [](const std::string& item) { return "- " + item; });
        */
}

/**
  | Check if a string does not contain any
  | embedded NUL (\0) characters
  |
  */
#[inline] pub fn valid_as_cstring(str_: &str) -> bool {
    
    todo!();
        /*
            return str.size() == strlen(str.c_str());
        */
}

/**
  | Locale-independent version of std::to_string
  |
  */

pub fn to_string<T>(t: &T) -> String {

    todo!();
        /*
            std::ostringstream oss;
        oss.imbue(std::locale::classic());
        oss << t;
        return oss.str();
        */
}

/**
  | Check whether a container begins with
  | the given prefix.
  |
  */
#[inline] pub fn has_prefix<T1, const PREFIX_LEN: usize>(
        obj:    &T1,
        prefix: &[u8;PREFIX_LEN]) -> bool {

    todo!();
        /*
            return obj.size() >= PREFIX_LEN &&
               std::equal(std::begin(prefix), std::end(prefix), std::begin(obj));
        */
}
