//! 警告：由于此crate为build.rs在编译器嵌入资源使用, 遇到任何错误都会直接panic（也就是编译期错误）
use capdows::prelude::*;
use capdows::ui::utility::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::path::Path;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use windows_sys::Win32::Storage::FileSystem::*;

mod compile_resource;
pub use compile_resource::LinkFor;
use compile_resource::compile_win32_res;
// extern crate embed_resource;
// use embed_resource::*;
// use windows_sys::Win32::Foundation::SetLastError; //{
// HMODULE, HWND, LPARAM, LRESULT, WPARAM,
// POINT, POINTS, RECT, SIZE, WIN32_ERROR, HINSTANCE,
// };
pub struct PreCompilePruduct(String);
impl PreCompilePruduct {
    pub fn from(s: String) -> Self {
        Self(s)
    }
    pub fn get(self) -> String {
        self.0
    }
    pub fn compile(self) {
        self.compile_for(Default::default())
    }
    pub fn compile_for(self, link_for: LinkFor) {
        let out_dir = env::var("OUT_DIR").expect("No OUT_DIR env var");
        let dest_path = Path::new(&out_dir).join(format!(
            "resource_{}_{}.rc",
            (self.0).as_bytes().len(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_nanos()
        ));
        let mut f = File::create(&dest_path).expect("无法创建文件");

        f.write_all(b"\xEF\xBB\xBF").expect("无法写入文件头");
        f.write_all((self.0).as_bytes()).expect("无法写入文件");

        compile_win32_res(dest_path.to_str().unwrap(), link_for);
    }
}
impl Add for PreCompilePruduct {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        PreCompilePruduct(format!("{}\n{}", self.0, other.0))
    }
}
pub use capdows::ui::core::{NumberId, ResourceID, StringId};
pub mod dialog;
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
#[macro_export]
macro_rules! compile_all_for {//ai宏
    ($for:expr, $first:expr, $($rest:expr),+ $(,)?) => {
        ($first $(+ $rest)+).compile_for($for)
    };
}
fn pre_compile_resource_id(id: ResourceID) -> PreCompilePruduct {
    PreCompilePruduct::from(match id {
        StringId(y) => {
            let result = y.to_string();
            if result.parse::<f32>().is_ok() {
                panic!("无效的资源ID，StringId不能由纯数字组成（包括小数）")
            };
            result
        }
        NumberId(x) => x.to_string(),
    })
}
fn pre_compile_lang_id(id: Option<LangID>) -> PreCompilePruduct {
    PreCompilePruduct::from(match id {
        None => String::from("\nLANGUAGE 0x000, 0x00\n"), //LANG_NEUTRAL, SUBLANG_NEUTRAL
        Some(id) => {
            let (lang_id, sub_lang_id) = id.split();
            format!("\nLANGUAGE 0x{:03x}, 0x{:02x}\n", lang_id, sub_lang_id)
        }
    })
}
pub use capdows::i18n::LangID;
