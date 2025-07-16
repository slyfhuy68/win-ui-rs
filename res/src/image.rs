use super::*;
use std::fs::canonicalize;
#[derive(Debug)]
pub struct Bitmap(pub PathBuf, pub Option<LangID>);
impl Bitmap {
    pub fn pre_compile(self, id: ResourceID) -> PreCompilePruduct {
        PreCompilePruduct::from(format!(
            "{}{} BITMAP {}",
            pre_compile_lang_id(self.1).get(),
            pre_compile_resource_id(id).get(),
            (canonicalize(self.0).expect("无法获取文件路径"))
                .into_os_string()
                .into_string()
                .expect("完整文件路径包含非Unicode字符")
        ))
    }
}
#[derive(Debug)]
pub struct Icon(pub PathBuf, pub Option<LangID>);
impl Icon {
    pub fn pre_compile(self, id: ResourceID) -> PreCompilePruduct {
        PreCompilePruduct::from(format!(
            "{}{} ICON {}",
            pre_compile_lang_id(self.1).get(),
            pre_compile_resource_id(id).get(),
            (canonicalize(self.0).expect("无法获取文件路径"))
                .into_os_string()
                .into_string()
                .expect("完整文件路径包含非Unicode字符")
        ))
    }
}
#[derive(Debug)]
pub struct Cursor(pub PathBuf, pub Option<LangID>);
impl Cursor {
    pub fn pre_compile(self, id: ResourceID) -> PreCompilePruduct {
        PreCompilePruduct::from(format!(
            "{}{} CURSOR {}",
            pre_compile_lang_id(self.1).get(),
            pre_compile_resource_id(id).get(),
            (canonicalize(self.0).expect("无法获取文件路径"))
                .into_os_string()
                .into_string()
                .expect("完整文件路径包含非Unicode字符")
        ))
    }
}
