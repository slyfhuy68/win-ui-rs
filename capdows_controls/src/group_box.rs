use super::*;
pub struct GroupBox(HWND);
pub struct GroupBoxMsg(pub usize, HWND);
impl Control for GroupBox{
	type MsgType = GroupBoxMsg;
	fn from_window(wnd:Window) -> Self{
		Self(wnd.handle)
	}
	fn to_window(self) -> Window {
		Window{handle:self.0}
	}
}
impl ControlMsg for GroupBoxMsg{ 
	type ControlType = GroupBox;
	unsafe fn from_msg(ptr:usize) -> Option<Box<Self>>{
		let nmhdr = *(ptr as *mut NMHDR);
		let code = nmhdr.code;
		let w = nmhdr.hwndFrom.clone();
		let _ = nmhdr;
		let bmtype = match code {
			NM_CUSTOMDRAW => {
				ptr
			}
			_ => return None, 
		};
		Some(Box::new(Self (bmtype, w)))
	}
	fn get_control(&self) -> Self::ControlType{
		GroupBox(self.1)
	}
}
impl GroupBox {
	pub fn new(wnd:&mut Window,name:&str, 
		pos: Option<RectangleWH>, 
		identifier: WindowID, 
		style:ChildWindowStyles, 
		style_ex: NormalWindowExStyles, 
		font:bool, parent_draw: bool) -> Result<Self> {
			let control_style_ms = if parent_draw {
				WINDOW_STYLE(BS_OWNERDRAW as u32)
			} else {
				WINDOW_STYLE(0)
			} | WINDOW_STYLE(BS_GROUPBOX as u32);
			let hwnd = new_button(wnd, name, pos, identifier, style, style_ex, control_style_ms, font, !parent_draw, None)?;
			Ok(GroupBox(hwnd))
		}
}
