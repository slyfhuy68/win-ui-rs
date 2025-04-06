use super::*;
use std::any::Any;
pub trait Control {
    type MsgType: UnsafeControlMsg;
    ///应该调用unsafe {Self::is_self(wnd.handle)}检查是否为自身类型的窗口
    fn from_window(wnd: Window) -> Result<Self>
    where
        Self: Sized,
    {
        unsafe {
            if Self::is_self(&wnd)? {
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
    fn get_window(&self) -> &Window;
    fn get_window_mut(&mut self) -> &mut Window;
    fn is_self(wnd: &Window) -> Result<bool>;
    fn get_id(&self) -> WindowID {
        match unsafe {GetDlgCtrlID(self.get_window().handle)}{
            0 => 0,
            a => a.try_into().expect("The control ID exceeds the WindowID::MAX, the GetDlgCtrlID returned an invalid value."),
        }
    }
    fn get_class(&self) -> WindowClass;
}
impl<T: Control> From<T> for Window {
    fn from(ctl: T) -> Window {
        ctl.to_window()
    }
}
pub trait ControlMsgType {
    type ControlType: Control;
}
///表示lParam不为零的WM_COMMAND消息和表示lParam不为零的WM_NOTIFY消息
pub trait UnsafeControlMsg: UnsafeMessage + ControlMsgType {
    ///Left:WM_COMMAND的WPARAM的HIWORD, Right:指向 NMHDR 结构或将 NMHDR 结构作为其第一个成员的较大的未实现Unpin的结构的指针, WM_NOTIFY
    unsafe fn into_raw(&mut self) -> Result<Either<u16, PtrWapper<*mut NMHDR>>>;
    ///获取发送消息的控件
    unsafe fn get_control_unsafe(&self) -> &Self::ControlType;
    unsafe fn get_control_mut_unsafe(&mut self) -> &mut Self::ControlType;
    ///给你一个指向 NMHDR 结构或将 NMHDR 结构作为其第一个成员的较大结构的指针。返回一个自身实例(不检查)
    unsafe fn from_msg(ptr: usize, command: bool) -> Result<Self>
    where
        Self: Sized;
    unsafe fn is_self(ptr: usize) -> Result<bool> {
        unsafe {
            Self::ControlType::is_self(&Window {
                handle: (*(ptr as *mut NMHDR)).hwndFrom,
            })
        }
    }
}
pub trait ControlMsg: UnsafeControlMsg {
    fn into_raw_control_msg(&mut self) -> Result<(u16, Option<&mut dyn Any>)>;
    ///获取发送消息的控件
    fn get_control(&self) -> &Self::ControlType;
    fn get_control_mut(&mut self) -> &mut Self::ControlType;
    ///建议使用std::borrow::Cow
    fn from_raw_control_msg(code: u16, data: Option<&mut dyn Any>) -> Result<Self>
    where
        Self: Sized;
}
#[repr(C)]
pub struct DefaultNMHDR {
    pub nmhdr: NMHDR,
    pub data: *mut dyn Any,
}
impl<T: ControlMsg> UnsafeControlMsg for T {
    unsafe fn into_raw(&mut self) -> Result<Either<u16, PtrWapper<*mut NMHDR>>> {
        let handle = self.get_control().get_window().handle;
        let id = self.get_control().get_id() as usize;
        let (code, data) = self.into_raw_control_msg()?;
        if code > 0x7FFF || code < (WM_USER as u16) {
            panic!(
                "ControlMsg::into_raw_control_msg returned an invalid Msg-code! The msg-code must between 0x0400 and 0x7FFF"
            )
        }
        match data {
            None => Ok(Left(code)),
            Some(data) => Ok(Right({
                let mut nmhdr = Box::new(DefaultNMHDR {
                    nmhdr: NMHDR {
                        hwndFrom: handle,
                        idFrom: id,
                        code: (code as u32) + WM_USER - 1,
                    },
                    data: data as *mut dyn Any,
                });
                PtrWapper {
                    ptr: nmhdr.as_mut() as *mut DefaultNMHDR as *mut NMHDR,
                    owner: nmhdr,
                }
            })),
        }
    }
    unsafe fn get_control_unsafe(&self) -> &T::ControlType {
        self.get_control() as &T::ControlType
    }
    unsafe fn get_control_mut_unsafe(&mut self) -> &mut T::ControlType {
        self.get_control_mut() as &mut T::ControlType
    }
    //ptr:指向DefaultNMHDR的指针
    unsafe fn from_msg(ptr: usize, command: bool) -> Result<Self>
    where
        Self: Sized,
    {
        unsafe {
            if command {
                let msg_ptr = ptr as *const NMHDR;
                T::from_raw_control_msg(((*(msg_ptr)).code - WM_USER + 1) as u16, None)
            } else {
                let msg_ptr = ptr as *const DefaultNMHDR;
                T::from_raw_control_msg(
                    ((*(msg_ptr)).nmhdr.code - WM_USER + 1) as u16,
                    Some(&mut *(*(msg_ptr)).data),
                )
            }
        }
    }
}
