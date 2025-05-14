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
