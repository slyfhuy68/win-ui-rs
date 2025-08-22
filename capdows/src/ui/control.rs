use super::*;
///表示 NMHDR 或将 NMHDR 作为其第一个成员的、#[repr(C)]的较大结构
///
/// # SAFTY
/// 确保实现此trait的类型都是将 NMHDR 作为其第一个成员的、#[repr(C)]的结构体
pub unsafe trait NotifyMessage {
    fn code(&self) -> u32;
    ///返回的窗口可能不是Rust拥有的
    unsafe fn wnd_from(&self) -> Window;
    fn id_from(&self) -> WindowID;
}
unsafe impl NotifyMessage for NMHDR {
    fn code(&self) -> u32 {
        self.code
    }
    unsafe fn wnd_from(&self) -> Window {
        unsafe { Window::from_handle(self.hwndFrom) }
    }
    fn id_from(&self) -> WindowID {
        self.idFrom as u16
    }
}
///Windows控件
pub trait Control: AsMut<Window> + AsRef<Window> {
    type MsgType: UnsafeControlMsg;
    const CLASS_NAME: &'static str;
    const CLASS_NAME_WIDE: &'static widestr;
    fn from_window(wnd: Window) -> Result<Self>
    where
        Self: Sized,
    {
        unsafe {
            if Self::is_self(&wnd)? {
                Ok(Self::force_from_window(wnd))
            } else {
                Err(ERROR_INVALID_WINDOW_HANDLE)
            }
        }
    }
    ///不检查是否为自身类型的窗口
    unsafe fn force_from_window(wnd: Window) -> Self
    where
        Self: Sized;
    fn to_window(self) -> Window;
    // fn get_window(&self) -> &Window;
    // fn get_window_mut(&mut self) -> &mut Window;
    fn is_self(wnd: &Window) -> Result<bool>;
    fn get_id(&self) -> WindowID {
        let wnd: &Window = self.as_ref();
        match unsafe {GetDlgCtrlID(wnd.handle())}{
            0 => 0,
            a => a.try_into().expect("The control ID exceeds the WindowID::MAX, the GetDlgCtrlID returned an invalid value."),
        }
    }
    // fn get_class() -> WindowClass {
    //     unsafe { WindowClass::from_str(Self::CLASS_NAME) }
    // }
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
///
///表示lParam不为零的WM_COMMAND消息和表示lParam不为零的WM_NOTIFY消息
///
///对于此trait来说，它的OwnerType必须是NMHDR或将 NMHDR 结构体作为其第一个成员的、#[repr(C)]的较大结构体，否则会导致未定义行为
pub unsafe trait UnsafeControlMsg: /*UnsafeMessage + */ControlMsgType {
    type NotifyType: NotifyMessage;
    ///Left:WM_COMMAND的WPARAM的HIWORD, Right:指向 NMHDR 结构或将 NMHDR 结构作为其第一个成员的较大的未实现Unpin的结构的指针, WM_NOTIFY
    unsafe fn into_raw(self) -> Result<Either<u16, Self::NotifyType>>;
    ///给你一个指向 NMHDR 结构或将 NMHDR 结构作为其第一个成员的较大结构的指针。返回一个自身实例(不检查)
    unsafe fn from_msg(ptr: usize, command: bool) -> Result<Self>
    where
        Self: Sized;
    #[inline]
    unsafe fn is_self(ptr: usize) -> Result<bool> {
        unsafe {
            Self::ControlType::is_self(&(
                Window::from_handle((*(ptr as *mut NMHDR)).hwndFrom)
            ))
        }
    }
}
pub trait ControlMsg: /*UnsafeControlMsg + */ ControlMsgType{
    type ControlMsgDataType;
    fn into_raw_control_msg(self) -> Result<(u32, Option<Self::ControlMsgDataType>)>;
    fn from_raw_control_msg(code: u32, data: Option<&mut Self::ControlMsgDataType>, wnd: Window) -> Result<Self>
    where
        Self: Sized;
}

unsafe impl<T: ControlMsg> StaticMsg for T where T::ControlMsgDataType: 'static {}

#[repr(C)]
pub struct DefaultNMHDR<T> {
    pub nmhdr: NMHDR,
    pub data: Option<T>,
}
unsafe impl<T> NotifyMessage for DefaultNMHDR<T> {
    fn code(&self) -> u32 {
        self.nmhdr.code
    }
    unsafe fn wnd_from(&self) -> Window {
        unsafe { Window::from_handle(self.nmhdr.hwndFrom) }
    }
    fn id_from(&self) -> WindowID {
        self.nmhdr.idFrom as u16
    }
}
unsafe impl<T> UnsafeControlMsg for T
where
    T: ControlMsg,
{
    type NotifyType = DefaultNMHDR<T::ControlMsgDataType>;
    unsafe fn into_raw(self) -> Result<Either<u16, Self::NotifyType>> {
        let wnd: &Window = self.get_control().as_ref();
        let handle = unsafe { wnd.handle() };
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
                    Window::from_handle((*(msg_ptr)).hwndFrom),
                )
            } else {
                let msg_ptr = ptr as *mut DefaultNMHDR<T::ControlMsgDataType>;
                T::from_raw_control_msg(
                    (*(msg_ptr)).nmhdr.code,
                    (&mut (*(msg_ptr)).data).into(),
                    Window::from_handle((*(msg_ptr)).nmhdr.hwndFrom),
                )
            }
        }
    }
}
pub unsafe trait RawHwndControl: Control + Sized {
    #[inline]
    fn from_hwnd_ref(wnd: &HWND) -> Result<&Self> {
        unsafe {
            if Self::is_self(Window::from_ref(wnd))? {
                Ok(Self::from_hwnd_ref_unchecked(wnd))
            } else {
                Err(ERROR_INVALID_WINDOW_HANDLE)
            }
        }
    }
    #[inline]
    fn from_hwnd_ref_mut(wnd: &mut HWND) -> Result<&mut Self> {
        unsafe {
            if Self::is_self(Window::from_ref(wnd))? {
                Ok(Self::from_hwnd_ref_mut_unchecked(wnd))
            } else {
                Err(ERROR_INVALID_WINDOW_HANDLE)
            }
        }
    }
    unsafe fn from_hwnd_ref_unchecked(wnd: &HWND) -> &Self;
    unsafe fn from_hwnd_ref_mut_unchecked(wnd: &mut HWND) -> &mut Self;
}
