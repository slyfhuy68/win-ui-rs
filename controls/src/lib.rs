// use capdows::win32::allmods::*;
use capdows::win32::font::*;
use capdows::win32::*;
capdows::import_foundation!();
use std::ffi::c_void;
use windows::Win32::Graphics::Gdi::*;
// use windows::Win32::Graphics::Gdi::GetStockObject;
// use class::*;
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
pub mod combo_box;
pub mod edit;
pub mod group_box;
pub mod radio;
pub mod view;
use either::*;
fn style_of(wnd: &Window) -> WINDOW_STYLE {
    WINDOW_STYLE(style_of_raw(wnd) as u32)
}
fn style_of_raw(wnd: &Window) -> i32 {
    unsafe { GetWindowLongW(wnd.handle(), GWL_STYLE) as i32 }
}
fn new_control<S: Into<(WINDOW_STYLE, ChildWindowStyles)>>(
    wnd: &mut Window,
    control_name: &'static str,
    name: &str,
    pos: Option<Rectangle>,
    id: u16,
    styles: S,
    font: Option<ControlFont>,
) -> Result<Window> {
    unsafe {
        let (control_style, stylea) = styles.into();
        let (mut style, style_ex) = stylea.into();
        style |= WS_CHILD | control_style;
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
            style_ex,
            cptr,
            ptr,
            style,
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
        Ok(Window::from_handle(hwnd))
    }
}
fn new_button<
    S: Into<(
        WINDOW_STYLE,
        Option<Either<Bitmap, Icon>>,
        ChildWindowStyles,
    )>,
>(
    wnd: &mut Window,
    name: &str,
    pos: Option<Rectangle>,
    id: u16,
    style: S,
    font: Option<ControlFont>,
) -> Result<Window> {
    let (cs, draw, cws) = style.into();
    let wnd = new_control(wnd, "BUTTON", name, pos, id, (cs, cws), font)?;
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
//这三个trait设计的不好，[todo]修改三个trait
pub trait CommonControl: Control + Sized {
    type Style: Into<(WINDOW_STYLE, ChildWindowStyles)> + Send + Sync;
    fn new(
        wnd: &mut Window,
        name: &str,
        pos: Option<Rectangle>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<Self> {
        let hwnd = new_control(
            wnd,
            Self::CLASS_NAME,
            name,
            pos,
            identifier,
            control_style,
            font,
        )?;
        unsafe { Ok(Self::force_from_window(hwnd.into())) }
    }
}
pub trait ButtonControl: Control + Sized {
    type Style: Into<(
            WINDOW_STYLE,
            Option<Either<Bitmap, Icon>>,
            ChildWindowStyles,
        )> + Send
        + Sync;
    fn new(
        wnd: &mut Window,
        name: &str,
        pos: Option<Rectangle>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<Self> {
        let hwnd = new_button(wnd, name, pos, identifier, control_style, font)?;
        unsafe { Ok(Self::force_from_window(hwnd.into())) }
    }
}
pub trait DataControl: Control + Sized {
    type Data;
    type Style: Into<(WINDOW_STYLE, Self::Data, ChildWindowStyles)> + Send + Sync;
    fn new(
        wnd: &mut Window,
        name: &str,
        pos: Option<Rectangle>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<Self> {
        let (cs, data, cws) = control_style.into();
        let hwnd = new_control(
            wnd,
            Self::CLASS_NAME,
            name,
            pos,
            identifier,
            (cs, cws),
            font,
        )?;
        Self::set_data(hwnd, data)
    }
    fn set_data(wnd: Window, data: Self::Data) -> Result<Self>;
}
