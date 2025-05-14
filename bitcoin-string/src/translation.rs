// ---------------- [ File: bitcoin-string/src/translation.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/translation.h]

/**
  | Bilingual messages:
  | 
  | - in GUI: user's native language + untranslated
  | (i.e. English)
  | 
  | - in log and stderr: untranslated only
  |
  */
#[derive(Default)]
pub struct BilingualStr {
    original:   String,
    translated: String,
}

impl AddAssign<&BilingualStr> for BilingualStr {
    
    #[inline]fn add_assign(&mut self, other: &BilingualStr) {
        todo!();
        /*
            original += rhs.original;
            translated += rhs.translated;
            return *this;
        */
    }
}

impl BilingualStr {

    pub fn empty(&self) -> bool {
        
        todo!();
        /*
            return original.empty();
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            original.clear();
            translated.clear();
        */
    }
}

impl Add<&BilingualStr> for BilingualStr {

    type Output = BilingualStr;
    
    fn add(self, other: &BilingualStr) -> Self::Output {
        todo!();
        /*
            lhs += rhs;
        return lhs;
        */
    }
}

impl From<&str> for BilingualStr {
    fn from(x: &str) -> Self {
        untranslated(x)
    }
}

/**
  | Mark a bilingual_str as untranslated
  |
  */
#[inline] pub fn untranslated(original: &str) -> BilingualStr {
    
    todo!();
        /*
            return {original, original};
        */
}

pub mod tinyformat {

    use super::*;

    pub fn format<Args>(
            fmt:  &BilingualStr,
            args: &Args) -> BilingualStr {

        todo!();
            /*
                return bilingual_str{format(fmt.original, args...), format(fmt.translated, args...)};
            */
    }
}

/**
  | Translate a message to the native language
  | of the user.
  |
  */
lazy_static!{
    /*
    const extern std::function<std::string(const char*)> G_TRANSLATION_FUN;
    */
}

/**
  | Translation function.
  | 
  | If no translation function is set, simply
  | return the input.
  |
  */
lazy_static!{
    /*
    inline bilingual_str _(const char* psz)
    {
        return bilingual_str{psz, G_TRANSLATION_FUN ? (G_TRANSLATION_FUN)(psz) : psz};
    }
    */
}
