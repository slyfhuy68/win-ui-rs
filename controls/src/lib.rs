use capdows::win32::allmods::*;
use capdows::win32::control::*;
use capdows::win32::*;
use std::ffi::c_void;
use windows::Win32::Graphics::Gdi::DEFAULT_GUI_FONT;
use windows::Win32::Graphics::Gdi::GetStockObject;
use windows::Win32::{Foundation::*, UI::Controls::*, UI::WindowsAndMessaging::*};
use windows::core::*;
pub mod button;
pub mod check_box;
pub mod edit;
pub mod group_box;
pub mod radio;
pub mod view;
fn new_control(
    wnd: &mut Window,
    control_name: &'static str,
    name: &str,
    pos: Option<Rectangle>,
    id: u16,
    style: ChildWindowStyles,
    style_ex: NormalWindowExStyles,
    control_style_ms: WINDOW_STYLE,
    font: bool,
    no_notify: bool,
) -> Result<HWND> {
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
    let parent = Some(wnd.handle);
    let (ptr, _ptr_raw) = str_to_pcwstr(name);
    let (cptr, _cptr_raw) = str_to_pcwstr(control_name);
    let (Point(x, y), Size(width, height)) = match pos {
        None => (
            Point(CW_USEDEFAULT, CW_USEDEFAULT),
            Size(CW_USEDEFAULT, CW_USEDEFAULT),
        ),
        Some(x) => x.get_size(),
    };
    let hinstance = HINSTANCE(unsafe { GetWindowLongW(wnd.handle, GWL_HINSTANCE) as *mut c_void });
    let hwnd = unsafe {
        CreateWindowExW(
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
        )
    }?;
    if font {
        unsafe {
            PostMessageW(
                Some(hwnd),
                WM_SETFONT,
                WPARAM(GetStockObject(DEFAULT_GUI_FONT).0 as usize),
                LPARAM(1),
            )?;
        };
    };
    Ok(hwnd)
}
fn new_button(
    wnd: &mut Window,
    name: &str,
    pos: Option<Rectangle>,
    id: u16,
    style: ChildWindowStyles,
    style_ex: NormalWindowExStyles,
    control_style_ms: WINDOW_STYLE,
    font: bool,
    no_notify: bool,
    draw: Option<Either<Bitmap, Icon>>,
) -> Result<HWND> {
    let hwnd = new_control(
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
                    Some(hwnd),
                    BM_SETIMAGE,
                    WPARAM(IMAGE_BITMAP.0 as usize),
                    LPARAM(b.handle.0 as isize),
                ),
                Right(c) => PostMessageW(
                    Some(hwnd),
                    BM_SETIMAGE,
                    WPARAM(IMAGE_ICON.0 as usize),
                    LPARAM(c.handle.0 as isize),
                ),
            };
        },
        None => {}
    };
    Ok(hwnd)
}
fn is_some_window(wnd: &Window, class: &'static str) -> Result<bool> {
    let mut array1 = vec![0u16; 8];
    if unsafe { GetClassNameW(wnd.clone().into(), &mut array1[..]) } == 0 {
        return Err(Error::from_win32());
    }
    let meunasfe = unsafe { PCWSTR(array1.as_ptr()).to_string()? };
    //println!("{}", meunasfe);
    return Ok(meunasfe.to_lowercase() == class.to_lowercase().to_string());
}
fn is_button_window(wnd: &HWND) -> Result<bool> {
    is_some_window(Window { handle: wnd }, "Button")
}
use capdows_macros::define_control;
