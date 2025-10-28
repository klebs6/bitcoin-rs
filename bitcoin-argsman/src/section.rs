// ---------------- [ File: bitcoin-argsman/src/section.rs ]
crate::ix!();

#[derive(Clone)]
pub struct SectionInfo
{
    pub name: String,
    pub file: String,
    pub line: i32,
}

impl SectionInfo {

    pub fn new(name: &str, file: &str, line: i32) -> Self {
        Self { 
            name: name.to_string(), 
            file: file.to_string(), 
            line: line 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_info_new_sets_fields() {
        let s = SectionInfo::new("abc", "file", 7);
        assert_eq!(s.name, "abc");
        assert_eq!(s.file, "file");
        assert_eq!(s.line, 7);
    }
}
