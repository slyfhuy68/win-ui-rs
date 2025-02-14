use capdows::Win32::allmods::*;
use capdows::Win32::*;
use capdows::Win32::control::*;
use windows::Win32::Graphics::Gdi::GetStockObject;
use windows::Win32::Graphics::Gdi::DEFAULT_GUI_FONT;
use windows::Win32::{Foundation::*, UI::WindowsAndMessaging::*, UI::Controls::*};
use std::ffi::c_void;
use windows::core::*;
pub struct Button (HWND);//PUSHBUTTON
#[derive(Default)]
pub enum BottonContentPos {
	#[default] Center, //BS_CENTER | BS_VCENTER
	Left, //BS_LEFT | BS_VCENTER
	Right, //BS_RIGHT | BS_VCENTER
	Top, //BS_TOP | BS_CENTER
	Bottom, //BS_BOTTOM | BS_CENTER
	TopLeft, //BS_TOP | BS_LEFT
	TopRight, //BS_TOP | BS_RIGHT
	BottomLeft, //BS_BOTTOM | BS_LEFT
	BottomRight, //BS_BOTTOM | BS_RIGHT
}
pub struct ButtonStyle{
	pub extra_msg:bool, //BS_NOTIFY
	pub light:bool, //if light BS_DEFPUSHBUTTON else BS_PUSHBUTTON
	pub flat:bool, //BS_FLAT
}
impl Into<WINDOW_STYLE> for ButtonStyle {
	fn into(self) -> WINDOW_STYLE {
		let mut ms_style = WINDOW_STYLE(0u32);
		if self.extra_msg {ms_style |= WINDOW_STYLE(BS_NOTIFY as u32);};
		if self.flat {ms_style |=  WINDOW_STYLE(BS_FLAT as u32);};
		if self.light {ms_style |=  WINDOW_STYLE(BS_DEFPUSHBUTTON as u32);} else {ms_style |=  WINDOW_STYLE(BS_PUSHBUTTON as u32);};
		ms_style
	}
}
impl Default for ButtonStyle {
	fn default() -> Self {
		Self {
			extra_msg:false,
			light:false,
			flat:true, 
		} 
	} 
}
pub enum BitmapOrIcon {
	Bitmap(Bitmap), 
	Icon(Icon)
}
pub enum ButtonAutoDrawType {
	IconOnly(BitmapOrIcon), //BS_ICON
	TextOnly(bool), //bool:multiple_lines BS_TEXT
	IconAndText(BitmapOrIcon, bool)//bool:BS_MULTILINE, BS_TEXT BS_ICON
}
pub enum ButtonMsgType {
	MouseEntering, 
	MouseLaveing, 
	Clicked, 
	DoubleClicked, 
	LoseKeyboardFocus, 
	GetKeyboardFocus, 
	Draw(usize),
}
pub struct ButtonMsg{
	hwnd:HWND, 
	pub bm_type:ButtonMsgType, 
}
impl Control for Button{
	type MsgType = ButtonMsg;
	fn from_window(wnd:Window) -> Self{
		Self(wnd.handle)
	}
	fn to_window(self) -> Window {
		Window{handle:self.0}
	}
}
impl ControlMsg for ButtonMsg{ 
	type ControlType = Button;
	unsafe fn from_msg(ptr:usize) -> Option<Box<Self>>{
		let nmhdr = *(ptr as *mut NMHDR);
		let code = nmhdr.code;
		let w = nmhdr.hwndFrom.clone();
		drop(nmhdr);
		use ButtonMsgType::*;
		let bmtype = match code {
			BCN_HOTITEMCHANGE => {
				let data = *(ptr as *mut NMBCHOTITEM);
				if data.dwFlags == HICF_MOUSE | HICF_ENTERING {
					MouseEntering
				} else if data.dwFlags == HICF_MOUSE | HICF_LEAVING {
					MouseLaveing
				} else {
					return None;
				}
			}, 
			BN_CLICKED => {
				Clicked
			}, 
			BN_DBLCLK | BN_DOUBLECLICKED => {
				DoubleClicked
			}, 
			BN_KILLFOCUS => {
				LoseKeyboardFocus
			}, 
			BN_SETFOCUS => {
				GetKeyboardFocus
			}, 
			NM_CUSTOMDRAW => {
				Draw(ptr)
			}
			_ => return None, 
		};
		Some(Box::new(Self {
			hwnd:w, 
			bm_type:bmtype
		}))
	}
	fn get_control(&self) -> Self::ControlType{
		Button(self.hwnd)
	}
}
pub enum ButtonDrawType {
	ParentDraw, //BS_OWNERDRAW
	AutoDraw(ButtonAutoDrawType, ButtonStyle), //NULL
}
impl Default for ButtonDrawType {
	fn default() -> Self {
		Self::AutoDraw(ButtonAutoDrawType::TextOnly(false), Default::default())
	} 
}
impl Into<(WINDOW_STYLE, Option<BitmapOrIcon>)> for ButtonDrawType {
	fn into(self) -> (WINDOW_STYLE, Option<BitmapOrIcon>) {
		match self {
			ButtonDrawType::ParentDraw => (WINDOW_STYLE(BS_OWNERDRAW as u32), None), 
			ButtonDrawType::AutoDraw(dtype, bstyle) => {
				let mut wstyle = WINDOW_STYLE(0);
				(bstyle.into(), match dtype {
					ButtonAutoDrawType::IconOnly(boi) => Some(boi), 
					ButtonAutoDrawType::TextOnly(a) => {if a {wstyle |= WINDOW_STYLE(BS_MULTILINE as u32);}; None}, 
					ButtonAutoDrawType::IconAndText(boi, a) => {
						if a {wstyle |= WINDOW_STYLE(BS_MULTILINE as u32)};
						Some(boi)
					}
				})
			}
		}
	}
}
// impl From(WINDOW_STYLE, Option<BitmapOrIcon>) for ButtonDrawType {
// 	fn from(data: (WINDOW_STYLE, Option<BitmapOrIcon>)) -> Self {
// 		
// 	}
// }
impl Button {
	pub fn new(wnd:&mut Window,name:&str, 
		pos: Option<RectangleWH>, 
		identifier: WindowID, 
		control_style:ButtonDrawType, 
		style:ChildWindowStyles, 
		style_ex: NormalWindowExStyles, 
		font:bool, no_notify: bool) -> Result<Self> {
			let mut xx: WINDOW_EX_STYLE = style_ex.into();
			let (mut yy, mut zz) = style.into();
			if no_notify {
				xx |= WS_EX_NOPARENTNOTIFY;
			};
			yy |= WS_CHILD;
			xx |= zz;
			let id = Some(HMENU(identifier as *mut c_void));
			let parent = Some(wnd.handle);
			let (ptr, ptr_raw) = str_to_pcwstr(name);
			let ((x, y), width, height) = match pos {
				None => ((CW_USEDEFAULT, CW_USEDEFAULT), CW_USEDEFAULT, CW_USEDEFAULT),
				Some(x) => x,
			};
			let hInstance = HINSTANCE(unsafe { GetWindowLongW(wnd.handle, GWL_HINSTANCE) as *mut c_void});
			let hwnd = unsafe {CreateWindowExW(
				xx, 
				w!("BUTTON"), 
				ptr, 
				yy, 
				x,
				y,
				width,
				height,
				parent, 
				id, 
				Some(hInstance),
				None
			)}?;
			if font { unsafe {
				PostMessageW(Some(hwnd), WM_SETFONT, WPARAM(GetStockObject(DEFAULT_GUI_FONT).0 as usize), LPARAM(1))?;
			};};
			Ok(Button(hwnd))
		}
}