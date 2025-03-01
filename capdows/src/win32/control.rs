use super::*;
/// #Panic
///此trait中的函数通常不应以任何理由Panic
pub trait Control {
    type MsgType: ControlMsg;
    ///应先调用unsafe {Self::is_self(wnd.handle)}检查是否为自身类型的窗口
    fn from_window(wnd: Window) -> Result<Box<Self>>;
    ///不应检查是否为自身类型的窗口
    unsafe fn force_from_window(wnd: Window) -> Self;
    fn to_window(self) -> Window;
    unsafe fn is_self(wnd: &HWND) -> Result<bool>;
}
/// #Panic
///此trait中的函数通常不应以任何理由Panic
pub trait ControlMsg {
    type ControlType: Control;
    unsafe fn from_msg(ptr: usize) -> Option<Box<Self>>;
    fn get_control(&self) -> Self::ControlType;
}
