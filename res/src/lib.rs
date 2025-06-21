use capdows::win32::*;
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
capdows::import_foundation!();
use windows::Win32::Storage::FileSystem::*;
pub struct PreCompilePruduct(String);
use std::ops::Add;
impl PreCompilePruduct {
    pub fn from(s: String) -> Self {
        Self(s)
    }
    pub fn get(self) -> String {
        self.0
    }
    pub fn compile(self) -> Result<()> {
        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("resource.rc");
        let mut f = File::create(&dest_path).expect("无法创建文件");
        f.write_all(b"\xEF\xBB\xBF")?;
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
pub use capdows::win32::core::{NumberId, ResourceID, StringId};
pub mod image;
pub mod menu;
pub mod string_table;
pub mod version;
#[macro_export]
macro_rules! compile_all {//ai宏
    ($first:expr, $($rest:expr),+ $(,)?) => {
        ($first $(+ $rest)+).compile()
    };
}

fn pre_compile_resource_id(id: ResourceID) -> Result<PreCompilePruduct> {
    Ok(PreCompilePruduct::from(match id {
        StringId(y) => {
            if y.parse::<f32>().is_ok() {
                return Err(ERROR_INVALID_STRING_ID);
            };
            y
        }
        NumberId(x) => x.to_string(),
    }))
}
fn pre_compile_lang_id(id: Option<LangID>) -> PreCompilePruduct {
    PreCompilePruduct::from(match id {
        None => String::from("\nLANGUAGE 0x000, 0x00\n"),//LANG_NEUTRAL, SUBLANG_NEUTRAL
        Some(id) => {
            let (lang_id, sub_lang_id) = id.split();
            format!("\nLANGUAGE 0x{:03x}, 0x{:02x}\n", lang_id, sub_lang_id)
        }
    })
}
pub use capdows::i18n::LangID;
