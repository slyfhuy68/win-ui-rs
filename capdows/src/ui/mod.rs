//                     _     _                                _            __   _                        __      ___      ____            _   _     _               _
//      __ _   _   _  | |_  | |__     ___    _ __   _   ___  | |  _   _   / _| | |__    _   _   _   _   / /_    ( _ )    / __ \    __ _  (_) | |_  | |__    _   _  | |__
//     / _` | | | | | | __| | '_ \   / _ \  | '__| (_) / __| | | | | | | | |_  | '_ \  | | | | | | | | | '_ \   / _ \   / / _` |  / _` | | | | __| | '_ \  | | | | | '_ \
//    | (_| | | |_| | | |_  | | | | | (_) | | |     _  \__ \ | | | |_| | |  _| | | | | | |_| | | |_| | | (_) | | (_) | | | (_| | | (_| | | | | |_  | | | | | |_| | | |_) |
//     \____|  \____|  \__| |_| |_|  \___/  |_|    (_) |___/ |_|  \____| |_|   |_| |_|  \____|  \____|  \___/   \___/   \ \____|  \____| |_|  \__| |_| |_|  \____| |____/
//                                                                |___/                         |___/                    \____/   |___/
// author:slyfhuy68@github
pub const PROC_KEY_NAME: &'static str = "MalibUserCallback";
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
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
use window::*;
pub mod utility {
    #[doc(no_inline)]
    pub use capdows_utility::*;
}

use crate::error::{Result, WinError as Error, WinError, errors::*};
use crate::positioning::*;
use crate::positioning::ext_methods::*;
use euclid::{point2, rect};
use crate::strings::*;
pub mod core {
    use super::*;
    pub type ResourceStringId = String;
    pub type ResourceNumberId = u16;
    pub enum ResourceID {
        StringId(&'static widestr),
        NumberId(ResourceNumberId),
    }
    #[doc(no_inline)]
    pub use ResourceID::*;
    impl ResourceID {
        #[inline]
        pub fn to_pcwstr(self) -> PCWSTR {
            match self {
                NumberId(x) => PCWSTR(x as *mut u16),
                StringId(y) => y.to_pcwstr()
            }
        }
    }
}
use self::core::*;
//----------------------------------------------------------------------------------
use super::i18n::*;
use either::*;
#[allow(unused_imports)]
use windows::core::{Error as wError, Result as wResult, w};
//----------------------------------------------------------------------------------
use std::ffi::c_void;
use std::num::NonZeroI32;
use std::num::NonZeroU32;
use std::{os::windows::raw::HANDLE, ptr::null_mut as NULL_PTR, string::*};
#[allow(unused_imports)]
use windows::Win32::Foundation::{
    APP_LOCAL_DEVICE_ID, COLORREF, CloseHandle, CompareObjectHandles, DECIMAL, DECIMAL_0_0,
    DECIMAL_1_0, DEVPROPKEY, DUPLICATE_HANDLE_OPTIONS, DuplicateHandle, FILETIME, FLOAT128,
    FreeLibrary, GENERIC_ACCESS_RIGHTS, GetHandleInformation, GetLastError, GlobalFree,
    HANDLE as wHANDLE, HANDLE_FLAGS, HANDLE_PTR, HGLOBAL, HINSTANCE, HLOCAL, HLSURF, HMODULE,
    HRSRC, HSPRITE, HSTR, HUMPD, HWND, LPARAM, LRESULT, LUID, LocalFree, NTSTATUS,
    NTSTATUS_FACILITY_CODE, NTSTATUS_SEVERITY_CODE, OBJECT_ATTRIBUTE_FLAGS, POINT, POINTL, POINTS,
    PROPERTYKEY, RECT, RECTL, RtlNtStatusToDosError, SHANDLE_PTR, SIZE, SYSTEMTIME,
    SetHandleInformation, SetLastError, SetLastErrorEx, SysAddRefString, SysAllocString,
    SysAllocStringByteLen, SysAllocStringLen, SysFreeString, SysReAllocString, SysReAllocStringLen,
    SysReleaseString, SysStringByteLen, SysStringLen, UNICODE_STRING, VARIANT_BOOL, WAIT_EVENT,
    WIN32_ERROR, WPARAM,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::{GetStartupInfoW, STARTUPINFOW};
use windows::Win32::{Graphics::Gdi::*, UI::Controls::*, UI::Shell::*, UI::WindowsAndMessaging::*};
use windows::core::*;
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
//                              工具函数
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
pub fn str_to_pcwstr(s: &str) -> (PCWSTR, Vec<u16>) {
    let wide_str: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_str_ptr = wide_str.as_ptr();
    return (PCWSTR(wide_str_ptr), wide_str);
}
pub fn str_to_pwstr(s: &str) -> (PWSTR, Vec<u16>) {
    let mut wide_str: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_str_ptr = wide_str.as_mut_ptr();
    return (PWSTR(wide_str_ptr), wide_str);
}
use std::hash::DefaultHasher;
use std::hash::Hash;
pub fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
// #[macro_export]
// macro_rules! win_error {
//     ($const:expr) => {
//         Error::from_win32($const)
//     };
// }
#[macro_export]
macro_rules! import_foundation {
    () => {
        #[allow(unused_imports)]
        use windows::Win32::Foundation::{
            APP_LOCAL_DEVICE_ID, COLORREF, CloseHandle, CompareObjectHandles, DECIMAL, DECIMAL_0_0,
            DECIMAL_1_0, DEVPROPKEY, DUPLICATE_HANDLE_OPTIONS, DuplicateHandle, FILETIME, FLOAT128,
            FreeLibrary, GENERIC_ACCESS_RIGHTS, GetHandleInformation, GetLastError, GlobalFree,
            HANDLE as wHANDLE, HANDLE_FLAGS, HANDLE_PTR, HGLOBAL, HINSTANCE, HLOCAL, HLSURF,
            HMODULE, HRSRC, HSPRITE, HSTR, HUMPD, HWND, LPARAM, LRESULT, LUID, LocalFree, NTSTATUS,
            NTSTATUS_FACILITY_CODE, NTSTATUS_SEVERITY_CODE, OBJECT_ATTRIBUTE_FLAGS, POINT, POINTL,
            POINTS, PROPERTYKEY, RECT, RECTL, RtlNtStatusToDosError, SHANDLE_PTR, SIZE, SYSTEMTIME,
            SetHandleInformation, SetLastError, SetLastErrorEx, SysAddRefString, SysAllocString,
            SysAllocStringByteLen, SysAllocStringLen, SysFreeString, SysReAllocString,
            SysReAllocStringLen, SysReleaseString, SysStringByteLen, SysStringLen, UNICODE_STRING,
            VARIANT_BOOL, WAIT_EVENT, WIN32_ERROR, WPARAM,
        };
    };
}
pub fn option_copy_handle(wnd: &Option<Window>) -> Option<Window> {
    match wnd {
        None => None,
        Some(wnd) => Some(wnd.copy_handle()),
    }
}
