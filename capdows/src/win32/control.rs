use super::*;
pub trait Control {
    type MsgType: ControlMsg;
    ///应先调用unsafe {Self::is_self(wnd.handle)}检查是否为自身类型的窗口
    fn from_window(wnd: Window) -> Result<Box<Self>>;
    ///不应检查是否为自身类型的窗口
    unsafe fn force_from_window(wnd: Window) -> Self;
    fn to_window(self) -> Window;
    unsafe fn is_self(wnd: &HWND) -> Result<bool>;
}
pub trait ControlMsg: CustomMessage {
    ///关联的控件类型
    type ControlType: Control;
    ///Left:WM_COMMAND的WPARAM值, Right:指向 NMHDR 结构或将 NMHDR 结构作为其第一个成员的较大结构的指针, WM_NOTIFY
    unsafe fn into_raw(&mut self) -> Either<WPARAM, *mut NMHDR>;
    ///获取发送消息的控件
    fn get_control(&self) -> Self::ControlType;
    ///给你一个指向 NMHDR 结构或将 NMHDR 结构作为其第一个成员的较大结构的指针。返回一个自身实例(不检查)
    unsafe fn from_msg(ptr: usize) -> Option<Box<Self>>;
}
impl<T: ControlMsg> CustomMessage for T {
    unsafe fn into_raw_msg(&mut self) -> RawMassage {
        let ptr = self.into_raw();
        match ptr {
            Left(l) => RawMassage(WM_COMMAND, l.0, self.get_control().to_window().handle.0 as isize), 
            Right(r) => RawMassage(WM_NOTIFY, 0, r as isize)
        }
    }
    unsafe fn is_self(ptr: RawMassage) -> Result<bool> {
        let RawMassage(msg, wparam, lparam) = ptr;
        match msg {
            WM_COMMAND => {
                let param2e = HWND(lparam as *mut c_void);
                T::ControlType::is_self(&param2e)
            }, 
            WM_NOTIFY => {
                let ptr = (*(lparam as *mut NMHDR)).hwndFrom;
                T::ControlType::is_self(&ptr)
            }, 
            _ => Ok(false)
        }
    }
    unsafe fn from_raw_msg(ptr: RawMassage) -> Option<Box<Self>> {
        let RawMassage(msg, wparam, lparam) = ptr;
        match msg {
            WM_COMMAND => {
                let mut nmhdr = NMHDR {
                    hwndFrom: HWND(lparam as *mut c_void),
                    idFrom: (wparam & 0xffff) as usize,
                    code: ((wparam >> 16) & 0xffff) as u32,
                };
                Self::from_msg(&mut nmhdr as *mut _ as usize)
            }, 
            WM_NOTIFY => {
                Self::from_msg(lparam as usize)
            }, 
            _ => None
        }
    }
}