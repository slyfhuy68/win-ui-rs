//                     _     _                                _            __   _                        __      ___      ____            _   _     _               _
//      __ _   _   _  | |_  | |__     ___    _ __   _   ___  | |  _   _   / _| | |__    _   _   _   _   / /_    ( _ )    / __ \    __ _  (_) | |_  | |__    _   _  | |__
//     / _` | | | | | | __| | '_ \   / _ \  | '__| (_) / __| | | | | | | | |_  | '_ \  | | | | | | | | | '_ \   / _ \   / / _` |  / _` | | | | __| | '_ \  | | | | | '_ \
//    | (_| | | |_| | | |_  | | | | | (_) | | |     _  \__ \ | | | |_| | |  _| | | | | | |_| | | |_| | | (_) | | (_) | | | (_| | | (_| | | | | |_  | | | | | |_| | | |_) |
//     \____|  \____|  \__| |_| |_|  \___/  |_|    (_) |___/ |_|  \____| |_|   |_| |_|  \____|  \____|  \___/   \___/   \ \____|  \____| |_|  \__| |_| |_|  \____| |____/
//                                                                |___/                         |___/                    \____/   |___/
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(non_upper_case_globals)]
#![allow(unused_unsafe)]
#![allow(non_snake_case)]
#![allow(unused_must_use)]
// author:slyfhuy68@github
pub const PROC_KEY_NAME: &'static str = "MalibUserCallback";
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
pub mod brush;
use brush::*;
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
mod prop;
use prop::*;
mod style;
use style::*;
mod sys_prop;
use sys_prop::*;
mod timer;
use timer::*;
pub mod window;
use window::*;
pub mod core {
    use super::*;
    pub type Point = (i32, i32);
    pub type Rectangle = (Point, Point);
    pub type RectangleWH = (Point, i32, i32); 
    pub trait IntOrName {
        fn to_pcwstr(self) -> PCWSTR;
    }
    #[derive(Copy, Clone)]
    pub struct NullRaw {}
    pub static NULLn: NullRaw = NullRaw {};
    impl IntOrName for NullRaw {
        fn to_pcwstr(self) -> PCWSTR {
            PCWSTR::null()
        }
    }
    impl<T: IntOrName> IntOrName for Option<T> {
        fn to_pcwstr(self) -> PCWSTR {
            match self {
                None => PCWSTR::null(),
                Some(x) => x.to_pcwstr(),
            }
        }
    }
    impl IntOrName for usize {
        fn to_pcwstr(self) -> PCWSTR {
            make_int_resource(self)
        }
    }
    impl IntOrName for u16 {
        fn to_pcwstr(self) -> PCWSTR {
            make_int_resource(self as usize)
        }
    }
    impl IntOrName for u8 {
        fn to_pcwstr(self) -> PCWSTR {
            make_int_resource(self as usize)
        }
    }
    impl IntOrName for String {
        //#[deprecated(note = "临时方案")]
        fn to_pcwstr(self) -> PCWSTR {
            let (strp, stri) = str_to_pcwstr(&self);
            std::mem::forget(stri); //[todo] 临时方案
            strp
        }
    }
}
use self::core::*;
pub mod allmods {
    pub use super::core::*;
    pub use super::brush::*;
    pub use super::class::*;
    pub use super::help::*;
    pub use super::image::*;
    pub use super::menu::*;
    pub use super::module::*;
    pub use super::msg::*;
    pub use super::prop::*;
    pub use super::window::*;
    pub use super::style::*;
    pub use super::sys_prop::*;
    pub use super::timer::*;
    pub use super::Result;
}
//----------------------------------------------------------------------------------
pub use windows::core::{Result, w};
//----------------------------------------------------------------------------------
use std::ffi::c_void;
use std::{os::windows::raw::HANDLE, ptr::null_mut as NULL_PTR, string::*};
use windows::core::*;
use windows::Win32::Foundation::HANDLE as wHANDLE;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::{GetStartupInfoW, STARTUPINFOW};
use windows::Win32::{Foundation::*, Graphics::Gdi::*, UI::Shell::*, UI::WindowsAndMessaging::*, UI::Controls::*};
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
//                              工具函数
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
pub fn get_last_error<T>() -> Result<T> {
    Err(Error::from(unsafe { GetLastError() }))
}
pub fn make_int_resource(i: usize) -> PCWSTR {
    PCWSTR(i as *mut u16)
}
pub fn usize_to_hmenu(inta: usize) -> HMENU {
    let hwnd1: *mut c_void = inta as *mut c_void;
    HMENU(hwnd1)
}
pub fn usize_to_hinstance(inta: usize) -> HINSTANCE {
    let hwnd1: *mut c_void = inta as *mut c_void;
    HINSTANCE(hwnd1)
}
pub fn usize_to_hwnd(inta: usize) -> HWND {
    let hwnd1: *mut c_void = inta as *mut c_void;
    HWND(hwnd1)
}
pub fn str_to_pcwstr<'a>(s: &'a str) -> (PCWSTR, Vec<u16>) {
    let wide_str: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_str_ptr = wide_str.as_ptr();
    return (PCWSTR(wide_str_ptr), wide_str);
}
pub fn str_to_vecu16(s: &str) -> Vec<u16> {
    return s
        .to_string()
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
}
// fn str_to_usizeptr(line: &str) -> usize {
//     return str_to_vecu16(&line).as_ptr() as usize;
// }
pub fn result_to_win_result<T, E>(ierror: std::result::Result<T, E>) -> Result<T> {
    match ierror {
        std::result::Result::Ok(x) => Ok(x),
        std::result::Result::Err(_) => Err(Error::empty()), //[todo]
    }
}

// --------------------------------------------------------------------------------------------------------------------------------------------------------------
//                          win api 函数
// --------------------------------------------------------------------------------------------------------------------------------------------------------------
// fn send_window_msg(
//     hwnd: usize,
//     msg: u32,
//     wparam1: Option<usize>,
//     lparam1: Option<usize>,
// ) -> Result<isize> {
//      let hWnd = HWND(*hWnd2);
//     let hwnd1: *mut c_void = hwnd as *mut c_void;
//     let hwnd = HWND(hwnd1);
//     let wparam = match wparam1 {
//         Some(wparamold) => WPARAM(wparamold.try_into().unwrap()),
//         None => WPARAM(0.try_into().unwrap()),
//     };
//     let lparam = match lparam1 {
//         Some(lparamold) => LPARAM(lparamold.try_into().unwrap()),
//         None => LPARAM(0.try_into().unwrap()),
//     };
//     Ok(unsafe { SendMessageW(hwnd,msg,wparam,lparam) }.0)
// }
// --------------------------------------------------------------------------------------------------------------------------------------------------------------
// fn find_window(class_name: Option<&str>,window_name: Option<&str>) -> Result<HWND> {
//      match class_name {
//         Some(class_name1) => let (class_name,class_ptr) = str_to_pcwstr(class_name1),
//         None => PCWSTR::from_raw(std::ptr::null())
//     };
//     let window_name = match window_name {
//         Some(window_name1) => str_to_pcwstr(window_name1),
//         None => PCWSTR::from_raw(std::ptr::null()),
//     };
//     println!("{:?}",class_name);
//     println!("{:?}",window_name);
//     unsafe { FindWindowW(class_name,window_name) }
// }
// --------------------------------------------------------------------------------------------------------------------------------------------------------------
//
// --------------------------------------------------------------------------------------------------------------------------------------------------------------
