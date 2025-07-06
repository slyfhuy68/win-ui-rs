use capdows::L;
use capdows::prelude::*;
use capdows::ui::{control::*, image::*, style::*, window::*, *};
capdows::import_foundation!();
use std::ffi::c_void;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::{UI::Controls::*, UI::WindowsAndMessaging::*};
use windows::core::{PCWSTR, w};
use capdows_resource::dialog::{DialogTempleControl, ControlPreCompilePruduct};
pub mod button;
pub mod check_box;
pub mod combo_box;
pub mod edit;
pub mod group_box;
// pub mod radio;
// pub mod view;
// pub mod prelude {
//     pub use crate::{
//         button::*, 
//         check_box::*, 
//         
//     }
// }
pub type ButtonImage = Either<Bitmap, Icon>;
use either::*;
// fn style_of(wnd: &Window) -> WINDOW_STYLE {
//     WINDOW_STYLE(style_of_raw(wnd) as u32)
// }
fn style_of_raw(wnd: &Window) -> i32 {
    unsafe { GetWindowLongW(wnd.handle(), GWL_STYLE) as i32 }
}
fn new_control(
    wnd: &mut Window,
    control_class: PCWSTR,
    name: String,
    pos: Option<Rect>,
    id: u16,
    style: WINDOW_STYLE,
    style_ex: WINDOW_EX_STYLE,
    font: Option<ControlFont>,
) -> Result<Window> {
    unsafe {
        let id = Some(HMENU(id as *mut c_void));
        let parent = Some(wnd.handle());
        let (ptr, _ptr_raw) = str_to_pcwstr(&name);
        let ((x, y), (width, height)) = match pos {
            None => (
                (CW_USEDEFAULT, CW_USEDEFAULT),
                (CW_USEDEFAULT, CW_USEDEFAULT),
            ),
            Some(euclid::Rect { origin, size }) => (origin.to_tuple(), size.to_tuple()),
        };
        let hinstance = HINSTANCE(GetWindowLongW(wnd.handle(), GWL_HINSTANCE) as *mut c_void);
        let hwnd = CreateWindowExW(
            style_ex,
            control_class,
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
fn new_button(
    wnd: &mut Window,
    name: String,
    pos: Option<Rect>,
    id: u16,
    style: WINDOW_STYLE,
    style_ex: WINDOW_EX_STYLE,
    font: Option<ControlFont>,
    draw: Option<ButtonImage>,
) -> Result<Window> {
    let wnd = new_control(wnd, w!("Button"), name, pos, id, style, style_ex, font)?;
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
fn is_some_window(wnd: &Window, class: &'static widestr) -> Result<bool> {
    let mut buffer = [0u16; 16]; //控件类名通常不超过16个字符
    let len = unsafe { GetClassNameW(wnd.handle(), &mut buffer) } as usize;
    if len == 0 {
        return Err(WinError::correct_error());
    };
    let new_buffer = &buffer[..len];
    Ok(unsafe { class.eq_ignore_ascii_case(widestr::from_utf16_unchecked(new_buffer)) })
}
use capdows_macros::define_control;
pub trait CommonControl: Control + Sized {
    type Style: Send + Sync;
    fn new(
        wnd: &mut Window,
        pos: Option<Rect>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<Self>;
}
pub trait TextControl: Control + Sized {
    const INSUFFICIENT_SPACE_RESULT: u32 = 0;
    const NOT_SUPPORT_RESULT: u32 = CB_ERR as u32;
    fn get_text(&self) -> Result<String> {
        let length = self.get_text_length()?;
        if length == 0 {
            return Ok(String::new());
        };
        let mut buffer: Vec<u16> = vec![0; length + 1];
        unsafe {
            Error::correct_error_result(
                SendMessageW(
                    self.get_window().handle(),
                    WM_GETTEXT,
                    Some(WPARAM(length)),
                    Some(LPARAM(buffer.as_mut_ptr() as isize)),
                )
                .0,
            )?;
        }
        Ok(String::from_utf16_lossy(&buffer[..length]))
    }
    fn get_text_length(&self) -> Result<usize> {
        Ok(unsafe {
            Error::correct_error_result(
                SendMessageW(self.get_window().handle(), WM_GETTEXTLENGTH, None, None).0,
            )? as usize
        })
    }
    fn set_text(&mut self, text: &str) -> Result<()> {
        let (text_ptr, _buffer) = str_to_pcwstr(text);
        let result = unsafe {
            Error::correct_error_result(SendMessageW(
                self.get_window().handle(),
                WM_SETTEXT,
                None,
                Some(LPARAM(text_ptr.0 as isize)),
            ))?
        }
        .0 as u32;
        if result == Self::INSUFFICIENT_SPACE_RESULT {
            Err(ERROR_INSUFFICIENT_SPACE)
        } else if result == Self::NOT_SUPPORT_RESULT {
            Err(ERROR_NOT_SUPPORTED)
        } else {
            Ok(())
        }
    }
}