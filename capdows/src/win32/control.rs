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
///对于此trait的from_msg：给你一个指向 NMHDR 结构或将 NMHDR 结构作为其第一个成员的较大结构的指针
pub trait ControlMsg: CustomMessage {
    ///关联的控件类型
    type ControlType: Control;
    ///bool:是否为WM_COMMAND, *mut NMHDR:指向 NMHDR 结构或将 NMHDR 结构作为其第一个成员的较大结构的指针
    unsafe fn into_raw_control(&mut self) -> (bool, *mut NMHDR);
    fn get_control(&self) -> Self::ControlType;
    unsafe fn into_raw(&mut self) -> (u32, usize) {
        unsafe{
        let (mtype, ptr) = self.into_raw_control();
        (if mtype {WM_COMMAND} else {WM_NOTIFY}, ptr as usize)}
    }
    unsafe fn is_self(ptr: usize) -> Result<bool> {
        unsafe{
        Self::ControlType::is_self(&(*(ptr as *mut NMHDR)).hwndFrom)}
    }
}
