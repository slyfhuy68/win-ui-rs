use super::Result;
use crate::PreCompilePruduct;
use crate::pre_compile_lang_id;
use capdows::i18n::LangID;
use capdows::win32::core::ResourceNumberId;
pub use capdows::win32::help::HelpId;
pub use capdows::win32::menu::MenuItemID;
pub use capdows::win32::menu::MenuItemStyle;
use std::collections::HashMap;
//STRINGTABLE  [optional-statements] {stringID string  ...}
pub struct StringTable {
    pub language: Option<LangID>,
    pub strings: HashMap<ResourceNumberId, String>,
}
impl StringTable {
    pub fn pre_compile(self) -> Result<PreCompilePruduct> {
        Ok(PreCompilePruduct::from(format!(
            "STRINGTABLE{}{{
{}
    	}}",
            pre_compile_lang_id(self.language).get(),
            self.strings
                .into_iter()
                .map(|(id, string)| { format!("  {}, \"{}\"", id, string) })
                .collect::<Vec<_>>()
                .join("\n")
        )))
    }
}
