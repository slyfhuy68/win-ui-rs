use super::*;
#[derive(Debug)]
#[repr(transparent)]
pub struct Bitmap(pub PathBuf);
impl Bitmap {
    pub fn pre_compile(self, id: ResourceID) -> Result<PreCompilePruduct> {
        Ok(PreCompilePruduct::from(format!(
            "{} BITMAP {:?}",
            pre_compile_resource_id(id)?.get(),
            (std::fs::canonicalize(self.0)?).as_os_str()
        )))
    }
}
#[derive(Debug)]
#[repr(transparent)]
pub struct Icon(pub PathBuf);
impl Icon {
    pub fn pre_compile(self, id: ResourceID) -> Result<PreCompilePruduct> {
        Ok(PreCompilePruduct::from(format!(
            "{} ICON {:?}",
            pre_compile_resource_id(id)?.get(),
            (std::fs::canonicalize(self.0)?).as_os_str()
        )))
    }
}
#[derive(Debug)]
#[repr(transparent)]
pub struct Cursor(pub PathBuf);
impl Cursor {
    pub fn pre_compile(self, id: ResourceID) -> Result<PreCompilePruduct> {
        Ok(PreCompilePruduct::from(format!(
            "{} CURSOR {:?}",
            pre_compile_resource_id(id)?.get(),
            (std::fs::canonicalize(self.0)?).as_os_str()
        )))
    }
}
