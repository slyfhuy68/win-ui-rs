use super::*;
use std::fs::canonicalize;
fn pre_compile_image(
    pathbuf: PathBuf,
    lang: Option<LangID>,
    id: ResourceID,
    name: &'static str,
) -> PreCompilePruduct {
    PreCompilePruduct::from(format!(
        "{}{} {name} {}",
        pre_compile_lang_id(lang).get(),
        pre_compile_resource_id(id).get(),
        {
            let mut path = canonicalize(pathbuf)
                .expect("无法获取文件路径")
                .into_os_string()
                .into_string()
                .expect("完整文件路径包含非Unicode字符");
            path.remove(0);
            path.remove(0);
            path.remove(0);
            path.remove(0);
            path
        }
    ))
}
#[derive(Debug)]
pub struct Bitmap(pub PathBuf, pub Option<LangID>);
impl Bitmap {
    #[inline]
    pub fn pre_compile(self, id: ResourceID) -> PreCompilePruduct {
        pre_compile_image(self.0, self.1, id, "BITMAP")
    }
}
#[derive(Debug)]
pub struct Icon(pub PathBuf, pub Option<LangID>);
impl Icon {
    #[inline]
    pub fn pre_compile(self, id: ResourceID) -> PreCompilePruduct {
        pre_compile_image(self.0, self.1, id, "ICON")
    }
}
#[derive(Debug)]
pub struct Cursor(pub PathBuf, pub Option<LangID>);
impl Cursor {
    #[inline]
    pub fn pre_compile(self, id: ResourceID) -> PreCompilePruduct {
        pre_compile_image(self.0, self.1, id, "CURSOR")
    }
}
