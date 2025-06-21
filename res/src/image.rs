use super::*;
#[derive(Debug)]
pub struct Bitmap(pub PathBuf, pub Option<LangID>);
impl Bitmap {
    pub fn pre_compile(self, id: ResourceID) -> Result<PreCompilePruduct> {
        Ok(PreCompilePruduct::from(format!(
            "{}{} BITMAP {}",
            pre_compile_lang_id(self.1).get(),
            pre_compile_resource_id(id)?.get(),
            (std::fs::canonicalize(self.0)?)
                .into_os_string()
                .into_string()
                .map_err(|_| ERROR_NO_UNICODE_TRANSLATION)?
        )))
    }
}
#[derive(Debug)]
pub struct Icon(pub PathBuf, pub Option<LangID>);
impl Icon {
    pub fn pre_compile(self, id: ResourceID) -> Result<PreCompilePruduct> {
        Ok(PreCompilePruduct::from(format!(
            "{}{} ICON {}",
            pre_compile_lang_id(self.1).get(),
            pre_compile_resource_id(id)?.get(),
            (std::fs::canonicalize(self.0)?)
                .into_os_string()
                .into_string()
                .map_err(|_| ERROR_NO_UNICODE_TRANSLATION)?
        )))
    }
}
#[derive(Debug)]
pub struct Cursor(pub PathBuf, pub Option<LangID>);
impl Cursor {
    pub fn pre_compile(self, id: ResourceID) -> Result<PreCompilePruduct> {
        Ok(PreCompilePruduct::from(format!(
            "{}{} CURSOR {}",
            pre_compile_lang_id(self.1).get(),
            pre_compile_resource_id(id)?.get(),
            (std::fs::canonicalize(self.0)?)
                .into_os_string()
                .into_string()
                .map_err(|_| ERROR_NO_UNICODE_TRANSLATION)?
        )))
    }
}
