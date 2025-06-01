// use capdows::win32::allmods::*;
use capdows::win32::font::*;
use capdows::win32::*;
capdows::import_foundation!();
use std::ffi::c_void;
use windows::Win32::Graphics::Gdi::*;
// use windows::Win32::Graphics::Gdi::GetStockObject;
use class::*;
use window::*;
use windows::Win32::{UI::Controls::*, UI::WindowsAndMessaging::*};
use windows::core::PCWSTR;
// use msg::*;
use capdows::win32::core::*;
use control::*;
use image::*;
use style::*;
// use windows::core::Result as wResult;
pub mod button;
pub mod check_box;
pub mod edit;
pub mod group_box;
pub mod combo_box;
pub mod radio;
pub mod view;
use either::*;
fn style_of(wnd: &Window) -> WINDOW_STYLE {
    WINDOW_STYLE(style_of_raw(wnd) as u32)
}
fn style_of_raw(wnd: &Window) -> i32 {
    unsafe { GetWindowLongW(wnd.handle(), GWL_STYLE) as i32 }
}
fn new_control(
    wnd: &mut Window,
    control_name: &'static str,
    name: &str,
    pos: Option<Rectangle>,
    id: u16,
    style: ChildWindowStyles,
    style_ex: NormalWindowExStyles,
    control_style_ms: WINDOW_STYLE,
    font: Option<ControlFont>,
    no_notify: bool,
) -> Result<Window> {
    unsafe {
        let mut xx: WINDOW_EX_STYLE = style_ex.into();
        let (mut yy, zz) = style.into();
        if no_notify {
            xx |= WS_EX_NOPARENTNOTIFY;
        };
        yy |= WS_CHILD | control_style_ms;
        xx |= zz;
        let ex_style = xx;
        let w_style = yy;
        let id = Some(HMENU(id as *mut c_void));
        let parent = Some(wnd.handle());
        let (ptr, _ptr_raw) = str_to_pcwstr(name);
        let (cptr, _cptr_raw) = str_to_pcwstr(control_name);
        let (Point(x, y), Size(width, height)) = match pos {
            None => (
                Point(CW_USEDEFAULT, CW_USEDEFAULT),
                Size(CW_USEDEFAULT, CW_USEDEFAULT),
            ),
            Some(x) => x.get_size(),
        };
        let hinstance = HINSTANCE(GetWindowLongW(wnd.handle(), GWL_HINSTANCE) as *mut c_void);
        let hwnd = CreateWindowExW(
            ex_style,
            cptr,
            ptr,
            w_style,
            x,
            y,
            width,
            height,
            parent,
            id,
            Some(hinstance),
            None,
        )?;
        if let Some(font) = font {
            PostMessageW(
                Some(hwnd),
                WM_SETFONT,
                WPARAM(font.into_handle()?.0 as usize),
                LPARAM(1),
            )?;
        };
        Ok(hwnd.into())
    }
}
fn new_button(
    wnd: &mut Window,
    name: &str,
    pos: Option<Rectangle>,
    id: u16,
    style: ChildWindowStyles,
    style_ex: NormalWindowExStyles,
    control_style_ms: WINDOW_STYLE,
    font: Option<ControlFont>,
    no_notify: bool,
    draw: Option<Either<Bitmap, Icon>>,
) -> Result<Window> {
    let wnd = new_control(
        wnd,
        "BUTTON",
        name,
        pos,
        id,
        style,
        style_ex,
        control_style_ms,
        font,
        no_notify,
    )?;
    match draw {
        Some(x) => unsafe {
            let _ = match x {
                Left(b) => PostMessageW(
                    Some(wnd.handle()),
                    BM_SETIMAGE,
                    WPARAM(IMAGE_BITMAP.0 as usize),
                    LPARAM(b.handle.0 as isize),
                ),
                Right(c) => PostMessageW(
                    Some(wnd.handle()),
                    BM_SETIMAGE,
                    WPARAM(IMAGE_ICON.0 as usize),
                    LPARAM(c.handle.0 as isize),
                ),
            };
        },
        None => {}
    };
    Ok(wnd)
}
fn is_some_window(wnd: &Window, class: &'static str) -> Result<bool> {
    let mut array1 = vec![0u16; 8];
    if unsafe { GetClassNameW(wnd.copy_handle().into(), &mut array1[..]) } == 0 {
        return Err(Error::correct_error());
    }
    let meunasfe = unsafe { PCWSTR(array1.as_ptr()).to_string()? };
    //println!("{}", meunasfe);
    return Ok(meunasfe.to_lowercase() == class.to_lowercase().to_string());
}
fn is_button_window(wnd: &Window) -> Result<bool> {
    is_some_window(wnd, "Button")
}
use capdows_macros::define_control;
