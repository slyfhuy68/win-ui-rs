use super::*;
use capdows::ui::core::ResourceNumberId;
use std::collections::HashMap;
//STRINGTABLE  [optional-statements] {stringID string  ...}
pub struct StringTable {
    pub language: Option<LangID>,
    pub strings: HashMap<ResourceNumberId, String>,
}
impl StringTable {
    pub fn pre_compile(self) -> PreCompilePruduct {
        PreCompilePruduct::from(format!(
            "STRINGTABLE{}{{
{}
    	}}",
            pre_compile_lang_id(self.language).get(),
            self.strings
                .into_iter()
                .map(|(id, string)| { format!("  {}, \"{}\"", id, string) })
                .collect::<Vec<_>>()
                .join("\n")
        ))
    }
}
