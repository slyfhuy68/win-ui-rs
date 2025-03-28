//                     _     _                                _            __   _                        __      ___      ____            _   _     _               _
//      __ _   _   _  | |_  | |__     ___    _ __   _   ___  | |  _   _   / _| | |__    _   _   _   _   / /_    ( _ )    / __ \    __ _  (_) | |_  | |__    _   _  | |__
//     / _` | | | | | | __| | '_ \   / _ \  | '__| (_) / __| | | | | | | | |_  | '_ \  | | | | | | | | | '_ \   / _ \   / / _` |  / _` | | | | __| | '_ \  | | | | | '_ \
//    | (_| | | |_| | | |_  | | | | | (_) | | |     _  \__ \ | | | |_| | |  _| | | | | | |_| | | |_| | | (_) | | (_) | | | (_| | | (_| | | | | |_  | | | | | |_| | | |_) |
//     \____|  \____|  \__| |_| |_|  \___/  |_|    (_) |___/ |_|  \____| |_|   |_| |_|  \____|  \____|  \___/   \___/   \ \____|  \____| |_|  \__| |_| |_|  \____| |____/
//                                                                |___/                         |___/                    \____/   |___/
// #![allow(dead_code)]
#![allow(unused_variables)]
// #![allow(unused_mut)]
// #![allow(unused_imports)]
// #![allow(non_upper_case_globals)]
// #![allow(unused_unsafe)]
// #![allow(non_snake_case)]
// #![allow(unused_must_use)]
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
mod proc;
use proc::*;
pub mod prop;
pub mod style;
use style::*;
pub mod sys_prop;
pub mod timer;
pub mod window;
use window::*;
pub mod core {
    pub use crate::win_error;
    #[derive(Debug, Clone)]
    pub struct Point(pub i32, pub i32);
    impl Copy for Point {}
    #[derive(Debug, Clone)]
    pub struct Size(pub i32, pub i32);
    impl Copy for Size {}
    #[derive(Debug, Clone)]
    pub enum Rectangle {
        ///通过对角线两点定义矩形
        Points(Point, Point),
        ///通过左上角一点和宽高定义矩形
        PointSize(Point, Size),
    }
    impl Copy for Rectangle {}
    impl Rectangle {
        pub fn is_points(&self) -> bool {
            matches!(self, Rectangle::Points(_, _))
        }
        pub fn is_size(&self) -> bool {
            matches!(self, Rectangle::PointSize(_, _))
        }
        pub fn to_size(self) -> Self {
            match self {
                Rectangle::Points(w, Point(x, y)) => {
                    Rectangle::PointSize(w, Size(x - w.0, y - w.1))
                }
                x => x,
            }
        }
        pub fn to_point(self) -> Self {
            match self {
                Rectangle::PointSize(w, Size(x, y)) => {
                    Rectangle::Points(w, Point(w.0 + x, w.1 + y))
                }
                x => x,
            }
        }
        pub fn get_points(self) -> (Point, Point) {
            match self {
                Rectangle::PointSize(w, Size(x, y)) => (w, Point(w.0 + x, w.1 + y)),
                Rectangle::Points(x, y) => (x, y),
            }
        }
        pub fn get_size(self) -> (Point, Size) {
            match self {
                Rectangle::Points(w, Point(x, y)) => (w, Size(x - w.0, y - w.1)),
                Rectangle::PointSize(x, y) => (x, y),
            }
        }
    }
}
use self::core::*;
pub mod allmods {
    pub use super::brush::*;
    pub use super::class::*;
    pub use super::core::*;
    pub use super::help::*;
    pub use super::image::*;
    pub use super::menu::*;
    pub use super::module::*;
    pub use super::msg::*;
    pub use super::prop::*;
    pub use super::style::*;
    pub use super::window::*;

    pub use super::{Error, Result};
}
//----------------------------------------------------------------------------------
pub use either::*;
pub use windows::core::{Error, Result, w};
//----------------------------------------------------------------------------------
use std::ffi::c_void;
use std::{os::windows::raw::HANDLE, ptr::null_mut as NULL_PTR, string::*};
use windows::Win32::Foundation::HANDLE as wHANDLE;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::{GetStartupInfoW, STARTUPINFOW};
use windows::Win32::{
    Foundation::*, Graphics::Gdi::*, UI::Controls::*, UI::Shell::*, UI::WindowsAndMessaging::*,
};
use windows::core::*;
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
//                              工具函数
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
fn _po_to_pcwstr(sels: Option<Either<&str, usize>>) -> (PCWSTR, Option<Vec<u16>>) {
    match sels {
        None => (PCWSTR::null(), None),
        Some(x) => _p_to_pcwstr(x),
    }
}
fn _p_to_pcwstr(sels: Either<&str, usize>) -> (PCWSTR, Option<Vec<u16>>) {
    match sels {
        Right(x) => (make_int_resource(x), None),
        Left(y) => {
            let (pcw, pwc) = str_to_pcwstr(y);
            (pcw, Some(pwc))
        }
    }
}
pub fn make_int_resource(i: usize) -> PCWSTR {
    PCWSTR(i as *mut u16)
}
// pub fn usize_to_hmenu(inta: usize) -> HMENU {
//     let hwnd1: *mut c_void = inta as *mut c_void;
//     HMENU(hwnd1)
// }
// pub fn usize_to_hinstance(inta: usize) -> HINSTANCE {
//     let hwnd1: *mut c_void = inta as *mut c_void;
//     HINSTANCE(hwnd1)
// }
// pub fn usize_to_hwnd(inta: usize) -> HWND {
//     let hwnd1: *mut c_void = inta as *mut c_void;
//     HWND(hwnd1)
// }
pub fn str_to_pcwstr(s: &str) -> (PCWSTR, Vec<u16>) {
    let wide_str: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_str_ptr = wide_str.as_ptr();
    return (PCWSTR(wide_str_ptr), wide_str);
}
// pub fn str_to_vecu16(s: &str) -> Vec<u16> {
//     return s
//         .to_string()
//         .encode_utf16()
//         .chain(std::iter::once(0))
//         .collect();
// }
// pub fn result_to_win_result<T, E>(ierror: std::result::Result<T, E>) -> Result<T> {
//     match ierror {
//         std::result::Result::Ok(x) => Ok(x),
//         std::result::Result::Err(_) => Err(Error::empty()), //[todo]
//     }
// }
// pub fn str_to_pcwstr(s: & str) -> (PCWSTR, RawPointerData) {
//     let wide_str: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
//     let wide_str_ptr = wide_str.as_ptr();
//     let ptr_data = RawPointerData(Box::new(wide_str));
//     return (PCWSTR(wide_str_ptr), ptr_data);
// }
#[macro_export]
macro_rules! win_error {
    ($const:expr) => {
        Error::new($const.into(), "")
    };
}
