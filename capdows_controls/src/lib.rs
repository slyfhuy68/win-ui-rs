use capdows::Win32::allmods::*;
use capdows::Win32::*;
use capdows::Win32::control::*;
use windows::Win32::Graphics::Gdi::GetStockObject;
use windows::Win32::Graphics::Gdi::DEFAULT_GUI_FONT;
use windows::core::*;
use windows::Win32::{Foundation::*, UI::WindowsAndMessaging::*, UI::Controls::*};
use std::ffi::c_void;
pub mod button;
pub mod radio;
pub mod group_box;
pub enum BitmapOrIcon {
	Bitmap(Bitmap), 
	Icon(Icon)
}
fn new_control(wnd:&mut Window, 
		control_name:&'static str, 
		name:&str, 
		pos:Option<RectangleWH>, 
		id:u16,  
		style:ChildWindowStyles, 
		style_ex: NormalWindowExStyles, 
		control_style_ms:WINDOW_STYLE, 
		font:bool, no_notify: bool, 
		) -> Result<HWND> {
	let mut xx: WINDOW_EX_STYLE = style_ex.into();
	let (mut yy, zz) = style.into();
	if no_notify {
		xx |= WS_EX_NOPARENTNOTIFY;
	};
	//---------------------draw功能未实现！[todo]
	yy |= WS_CHILD | control_style_ms;
	xx |= zz;
	let ex_style = xx;
	let w_style = yy;
	let id = Some(HMENU(id as *mut c_void));
	let parent = Some(wnd.handle);
	let (ptr, _ptr_raw) = str_to_pcwstr(name);
	let (cptr, _cptr_raw) = str_to_pcwstr(control_name);
	let ((x, y), width, height) = match pos {
		None => ((CW_USEDEFAULT, CW_USEDEFAULT), CW_USEDEFAULT, CW_USEDEFAULT),
		Some(x) => x,
	};
	let hinstance = HINSTANCE(unsafe { GetWindowLongW(wnd.handle, GWL_HINSTANCE) as *mut c_void});
	let hwnd = unsafe {CreateWindowExW(
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
		None
	)}?;
	if font { unsafe {
		PostMessageW(Some(hwnd), WM_SETFONT, WPARAM(GetStockObject(DEFAULT_GUI_FONT).0 as usize), LPARAM(1))?;
	};};
	Ok(hwnd)
}
fn new_button(wnd:&mut Window, 
		name:&str, 
		pos:Option<RectangleWH>, 
		id:u16,  
		style:ChildWindowStyles, 
		style_ex: NormalWindowExStyles, 
		control_style_ms:WINDOW_STYLE, 
		font:bool, no_notify: bool, 
		draw:Option<BitmapOrIcon>
		) -> Result<HWND> {
	let hwnd = new_control(wnd, "BUTTON", name, pos, id, style, style_ex, control_style_ms, font, no_notify)?;
	match draw {
		Some(x) => unsafe {let _ = match x {
			BitmapOrIcon::Bitmap(b) => PostMessageW(Some(hwnd), BM_SETIMAGE, WPARAM(IMAGE_BITMAP.0 as usize) , LPARAM(b.handle as isize)), 
			BitmapOrIcon::Icon(c) => PostMessageW(Some(hwnd), BM_SETIMAGE, WPARAM(IMAGE_ICON.0 as usize), LPARAM(c.handle as isize)), 
		};}, 
		None => {}
	};
	Ok(hwnd)
}