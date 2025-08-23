pub mod brush;
// use brush::*;
use std::hash::Hasher;
pub mod mouse;
// use mouse::*;
pub mod tools;
// use tools::*;
pub mod class;
use class::*;
pub mod control;
use control::*;
pub mod dialog;
use dialog::*;
pub mod help;
use help::*;
pub mod image;
use image::*;
pub mod menu;
use menu::*;
pub mod module;
use module::*;
pub mod msg;
use msg::*;
pub mod font;
// use font::*;
mod proc;
use proc::*;
pub mod prop;
// use prop::*;
pub mod style;
use style::*;
pub mod sys_prop;
pub mod timer;
pub mod window;
use crate::error::*;
use crate::positioning::ext_methods::*;
use crate::positioning::*;
use crate::strings::*;
use capdows_utility::*;
use euclid::{point2, rect};
use window::*;
pub mod core {
    use super::*;
    pub use std::marker::PhantomData;
    pub type ResourceNumberId = u16;
    pub enum ResourceID {
        StringId(&'static CWideStr),
        NumberId(ResourceNumberId),
    }
    #[doc(no_inline)]
    pub use ResourceID::*;
    impl ResourceID {
        #[inline]
        pub fn to_pcwstr(self) -> PCWSTR {
            match self {
                NumberId(x) => x as PCWSTR,
                StringId(y) => y.to_pcwstr(),
            }
        }
    }
}
use self::core::*;
//----------------------------------------------------------------------------------
use super::i18n::*;
use either::*;
//----------------------------------------------------------------------------------
use crate::error::WinError as Error;
use std::ffi::c_void;
use std::num::NonZeroI32;
use std::num::NonZeroU32;
use std::os::windows::raw::HANDLE;
use std::{ptr::null_mut as NULL_PTR, string::*};
use windows_sys::Win32::Foundation::{
    HINSTANCE,
    HMODULE,
    HWND,
    LPARAM,
    LRESULT,
    WPARAM,
    // POINT, POINTS, RECT, SIZE, WIN32_ERROR,
};
use windows_sys::Win32::Globalization::lstrlenW;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::Threading::{GetStartupInfoW, STARTUPINFOW};
use windows_sys::Win32::{
    Graphics::Gdi::*, UI::Controls::*, UI::Shell::*, UI::WindowsAndMessaging::*,
};
use windows_sys::core::*;
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
//                              工具函数
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
pub fn str_to_pcwstr(s: &str) -> (PCWSTR, Vec<u16>) {
    let wide_str: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_str_ptr = wide_str.as_ptr();
    (wide_str_ptr as PCWSTR, wide_str)
}
pub fn str_to_pwstr(s: &str) -> (PWSTR, Vec<u16>) {
    let mut wide_str: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_str_ptr = wide_str.as_mut_ptr();
    (wide_str_ptr as PWSTR, wide_str)
}
use std::hash::DefaultHasher;
use std::hash::Hash;
pub fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
#[inline]
pub fn option_copy_handle(wnd: &Option<Window>) -> Option<Window> {
    wnd.as_ref().map(|wnd| wnd.copy_handle())
}
