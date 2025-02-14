use super::*;
pub trait Control {
	type MsgType:ControlMsg;
	fn from_window(wnd:Window) -> Self;
	fn to_window(self) -> Window;
}
pub trait ControlMsg { 
	type ControlType: Control;
	unsafe fn from_msg(ptr:usize) -> Option<Box<Self>>;
	fn get_control(&self) -> Self::ControlType;
}