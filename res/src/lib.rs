use capdows::win32::allmods::*;
use std::env;
// use either::Either;
// use either::Either::*;
//use windows::core::*;
use std::collections::HashMap;
use std::path::PathBuf;
extern crate embed_resource;
use embed_resource::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use windows::Win32::Foundation::*;
use windows::Win32::Storage::FileSystem::*;
pub struct PreCompilePruduct(String);
use std::ops::Add;
impl PreCompilePruduct {
    pub fn from(s: &str) -> Self {
        Self(s.to_string())
    }
    pub fn get(self) -> String {
        self.0
    }
    pub fn compile(self) -> Result<()> {
        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("resource.rc");
        let mut f = File::create(&dest_path).expect("无法创建文件");
        f.write_all((self.0).as_bytes()).expect("无法写入文件");
        compile(dest_path.to_str().unwrap(), NONE)
            .manifest_required()
            .unwrap();
        Ok(())
    }
}
impl Add for PreCompilePruduct {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        PreCompilePruduct(format!("{}\n{}", self.0, other.0))
    }
}
pub enum ResourceID {
    StringId(String),
    NumberId(u16),
}
pub use ResourceID::*;
pub mod image;
pub mod version;
#[macro_export]
macro_rules! compile_all {
    ($first:expr, $($rest:expr),+ $(,)?) => {
        ($first $(+ $rest)+).compile()
    };
}
//ai宏
