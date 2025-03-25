use super::*;
pub trait Control {
    type MsgType: ControlMsg;
    ///应该调用unsafe {Self::is_self(wnd.handle)}检查是否为自身类型的窗口
    fn from_window(wnd: Window) -> Result<Self>
    where
        Self: Sized,
    {
        unsafe {
            if Self::is_self(&wnd.handle)? {
                Ok(Self::force_from_window(wnd))
            } else {
                Err(Error::new(ERROR_INVALID_WINDOW_HANDLE.into(), ""))
            }
        }
    }
    ///不应检查是否为自身类型的窗口
    unsafe fn force_from_window(wnd: Window) -> Self
    where
        Self: Sized;
    fn to_window(self) -> Window;
    unsafe fn is_self(wnd: &HWND) -> Result<bool>;
}
pub trait ControlMsg: CustomMessage {
    ///关联的控件类型
    type ControlType: Control;
    ///Left:WM_COMMAND的WPARAM的HIWORD, Right:指向 NMHDR 结构或将 NMHDR 结构作为其第一个成员的较大结构的指针, WM_NOTIFY
    ///self如果存在则 *mut NMHDR（如果是WM_NOTIFY）指向的内容一定存在，self被Drop时，应释放指针内容避免内存泄漏
    unsafe fn into_raw(&mut self) -> Result<Either<u16, *mut NMHDR>>;
    ///获取发送消息的控件
    fn get_control(&self) -> Self::ControlType;
    ///给你一个指向 NMHDR 结构或将 NMHDR 结构作为其第一个成员的较大结构的指针。返回一个自身实例(不检查)
    unsafe fn from_msg(ptr: usize) -> Result<Self>
    where
        Self: Sized;
}
