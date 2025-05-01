use super::*;
///表示 NMHDR 或将 NMHDR 作为其第一个成员的、#[repr(C)]的较大结构
pub unsafe trait NotifyMessage {}
unsafe impl NotifyMessage for NMHDR {}
///Windows控件
pub trait Control {
    type MsgType: UnsafeControlMsg;
    ///应该调用unsafe {Self::is_self(wnd.handle)}检查是否为自身类型的窗口
    fn from_window(wnd: &Window) -> Result<Self>
    where
        Self: Sized,
    {
        unsafe {
            if Self::is_self(&wnd)? {
                Ok(Self::force_from_window(wnd.copy_handle()))
            } else {
                Err(ERROR_INVALID_WINDOW_HANDLE)
            }
        }
    }
    ///不应检查是否为自身类型的窗口
    unsafe fn force_from_window(wnd: Window) -> Self
    where
        Self: Sized;
    fn to_window(self) -> Window;
    fn get_window(&self) -> &Window;
    fn get_window_mut(&mut self) -> &mut Window;
    fn is_self(wnd: &Window) -> Result<bool>;
    fn get_id(&self) -> WindowID {
        match unsafe {GetDlgCtrlID(self.get_window().handle())}{
            0 => 0,
            a => a.try_into().expect("The control ID exceeds the WindowID::MAX, the GetDlgCtrlID returned an invalid value."),
        }
    }
    fn get_class() -> WindowClass;
}
impl<T: Control> From<T> for Window {
    fn from(ctl: T) -> Window {
        ctl.to_window()
    }
}

pub trait ControlMsgType: Send + Sync {
    type ControlType: Control;
    fn get_control(&self) -> &Self::ControlType;
    fn get_control_mut(&mut self) -> &mut Self::ControlType;
}
///控件消息
///表示lParam不为零的WM_COMMAND消息和表示lParam不为零的WM_NOTIFY消息
///对于此trait来说，它的OwnerType必须是NMHDR或将 NMHDR 结构体作为其第一个成员的、#[repr(C)]的较大结构体，否则会导致未定义行为
pub unsafe trait UnsafeControlMsg: /*UnsafeMessage + */ControlMsgType {
    type NotifyType: NotifyMessage;
    ///Left:WM_COMMAND的WPARAM的HIWORD, Right:指向 NMHDR 结构或将 NMHDR 结构作为其第一个成员的较大的未实现Unpin的结构的指针, WM_NOTIFY
    unsafe fn into_raw(self) -> Result<Either<u16, Self::NotifyType>>;
    ///给你一个指向 NMHDR 结构或将 NMHDR 结构作为其第一个成员的较大结构的指针。返回一个自身实例(不检查)
    unsafe fn from_msg(ptr: usize, command: bool) -> Result<Self>
    where
        Self: Sized;
    unsafe fn is_self(ptr: usize) -> Result<bool> {
        unsafe { Self::ControlType::is_self(&((*(ptr as *mut NMHDR)).hwndFrom.into())) }
    }
}
pub trait ControlMsg: /*UnsafeControlMsg + */ ControlMsgType{
    type ControlMsgDataType;
    fn into_raw_control_msg(self) -> Result<(u32, Option<Self::ControlMsgDataType>)>;
    fn from_raw_control_msg(code: u32, data: Option<&mut Self::ControlMsgDataType>, wnd: Window) -> Result<Self>
    where
        Self: Sized;
}
#[repr(C)]
pub struct DefaultNMHDR<T> {
    pub nmhdr: NMHDR,
    pub data: Option<T>,
}
unsafe impl<T> NotifyMessage for DefaultNMHDR<T> {}
unsafe impl<T> UnsafeControlMsg for T
where
    T: ControlMsg,
{
    type NotifyType = DefaultNMHDR<T::ControlMsgDataType>;
    unsafe fn into_raw(self) -> Result<Either<u16, Self::NotifyType>> {
        let handle = unsafe { self.get_control().get_window().handle() };
        let id = self.get_control().get_id() as usize;
        let (code, data) = self.into_raw_control_msg()?;
        let mdata = match data {
            None => {
                let ucode: std::result::Result<u16, _> = code.try_into();
                match ucode {
                    Ok(code) => return Ok(Left(code)),
                    Err(_) => None,
                }
            }
            Some(data) => Some(data),
        };
        Ok(Right(DefaultNMHDR {
            nmhdr: NMHDR {
                hwndFrom: handle,
                idFrom: id,
                code: code,
            },
            data: mdata,
        }))
    }
    //ptr:指向DefaultNMHDR的指针
    unsafe fn from_msg(ptr: usize, command: bool) -> Result<Self>
    where
        Self: Sized,
    {
        unsafe {
            if command {
                let msg_ptr = ptr as *const NMHDR;
                T::from_raw_control_msg(
                    ((*(msg_ptr)).code) as u32,
                    None,
                    (*(msg_ptr)).hwndFrom.into(),
                )
            } else {
                let msg_ptr = ptr as *mut DefaultNMHDR<T::ControlMsgDataType>;
                T::from_raw_control_msg(
                    (*(msg_ptr)).nmhdr.code,
                    (&mut (*(msg_ptr)).data).into(),
                    (*(msg_ptr)).nmhdr.hwndFrom.into(),
                )
            }
        }
    }
}
