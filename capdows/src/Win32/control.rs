use super::*;
/// #Panic
///此trait中的函数通常不应以任何理由Panic
pub trait Control {
	type MsgType:ControlMsg;
	fn from_window(wnd:Window) -> Self;
	fn to_window(self) -> Window;
	unsafe fn is_self(wnd: HWND) -> Result<bool>;
}
/// #Panic
///此trait中的函数通常不应以任何理由Panic
pub trait ControlMsg { 
	type ControlType: Control;
	unsafe fn from_msg(ptr:usize) -> Option<Box<Self>>;
	fn get_control(&self) -> Self::ControlType;
}