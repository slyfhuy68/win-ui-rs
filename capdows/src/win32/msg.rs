use super::*;
pub type CallBackObj = dyn MessageReceiver + Sync + 'static;
#[derive(Clone, Eq, PartialEq)]
pub struct WindowSizeCalcType {
    pub top_align: Option<bool>, //None NULL true WVR_ALIGNTOP false WVR_ALIGNBOTTOM
    pub left_align: Option<bool>, //None NULL true WVR_ALIGNLEFT false WVR_ALIGNRIGHT
    pub her_draw: bool,
    pub ver_draw: bool,
}
#[derive(Clone, Eq, PartialEq)]
pub enum ShowStateChangeState {
    OtherUnzoom,
    OtherZoom,
    ParentCloseing,
    ParentOpening,
}
#[derive(Clone, Eq, PartialEq)]
pub enum SizedMsgType {
    MaxOther,    //SIZE_MAXHIDE
    Maximized,   //SIZE_MAXIMIZED
    RestOther,   //SIZE_MAXSHOW
    Minimized,   //SIZE_MINIMIZED
    WindowSized, //SIZE_RESTORED
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum MessageReceiverError {
    NoProcessed,
    WinErr(Error),
}
impl From<Error> for MessageReceiverError {
    fn from(err: Error) -> Self {
        Self::WinErr(err)
    }
}
impl MessageReceiverError {
    pub fn code(&self) -> i32 {
        match self {
            WinErr(x) => x.code(),
            NoProcessed => -255i32,
        }
    }
}
// impl Into<Error> for MessageReceiverError {
//      fn from(self) -> Self {
//              panic!()
//      }
// }
pub use MessageReceiverError::*;
pub type MessageReceiverResult<T> = std::result::Result<T, MessageReceiverError>;
pub enum SizingMsgType {
    Bottom,      //WMSZ_BOTTOM
    BottomLeft,  //WMSZ_BOTTOMLEFT
    BottomRight, //WMSZ_BOTTOMRIGHT
    Left,        //WMSZ_LEFT
    Right,       //WMSZ_RIGHT
    Top,         //WMSZ_TOP
    TopLeft,     //WMSZ_TOPLEFT
    TopRight,    //WMSZ_TOPRIGHT
}
//ai开始----
pub enum MouseMsgMoveType {
    Move(Point),
    Hover(Point),

    Leave,
}
pub enum MouseMsg {
    Move {
        mtype: MouseMsgMoveType,
        is_nc: bool,
    },

    Button {
        button_type: MouseButton,
        state: ButtonState,
        pos: Point,
        is_nc: bool,
    },

    Wheel {
        delta: i16,
        is_horizontal: bool,
        pos: Point,
    },
    CaptureLost(Window),
    // Activate {
    //     activation: MouseActivateState,
    // },
    //
    // HitTest {
    //     hit_test_code: HitTestCode,
    // },//自动响应
}

#[derive(Debug, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    X1,
    X2,
}

#[derive(Debug, PartialEq)]
pub enum ButtonState {
    Down,
    Up,
    DoubleClick,
}

#[derive(Debug, PartialEq)]
pub enum MouseActivateState {
    Activate,
    Inactive,
    NoActivate,
    NoActivateClick,
}
//ai结束-----
pub enum WindowNotify {
    Null, //WM_NULL
}
#[derive(Debug)]
pub enum MenuCommandMsgItemPos<'a> {
    CostomId(MenuItemID),
    Position(&'a mut Menu, u16),
}
///每个回调的id表示一个窗口的接收器id，如果这是一个子类化接收器，NoProcessed表示调用子类链上一个接收器，id为子类化id，如果不是，那么id为0，NoProcessed表示进行默认处理
#[allow(unused_variables)]
pub trait MessageReceiver {
    // fn activating()包含WM_MOUSEACTIVATE
    fn mouse_msg(
        &mut self,
        id: usize,
        window: &mut Window,
        msg: MouseMsg,
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn menu_command(
        &mut self,
        id: usize,
        window: &mut Window,
        item: MenuCommandMsgItemPos,
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn error_handler(&mut self, err: MessageReceiverError) -> MessageReceiverResult<isize> {
        Ok(err.code() as isize)
    }
    ///不常用的wParam与lParam都未使用、处理消息返回零的消息与WM_NULL
    fn notifications(
        &mut self,
        id: usize,
        window: &mut Window,
        notification_type: WindowNotify,
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn control_message(
        &mut self,
        id: usize,
        window: &mut Window,
        msg: &mut RawMessage,
        wnd_id: WindowID,
    ) -> MessageReceiverResult<isize> {
        Err(NoProcessed)
    }
    ///itype参数：这只是[`crate::win32::class::WindowClass`]的crate_window方法的参数的一个副本，但是你可以调用所有者/父窗口和菜单上面的方法，因为它们本质是指针
    fn create(
        &mut self,
        id: usize,
        window: &mut Window,
        name: &str,
        class: WindowClass,
        file: ExecutableFile,
        pos: Rectangle,
        itype: &mut WindowType,
        //ex_data: usize,
    ) -> MessageReceiverResult<bool> {
        Err(NoProcessed)
    } //true 0 false -1
    fn destroy(&mut self, id: usize, window: &mut Window) -> MessageReceiverResult<()> {
        stop_msg_loop();
        Ok(())
    }
    fn class_messages(
        &mut self,
        id: usize,
        window: &mut Window,
        code: u16,
        msg: RawMessage,
    ) -> MessageReceiverResult<usize> {
        Err(NoProcessed) //code = raw_code(WM_USER 到 0x7FFF) - WM_USER + 1,WM_USER = 0x0400
    }
    fn applications_messages(
        &mut self,
        id: usize,
        window: &mut Window,
        code: u16,
        msg: RawMessage,
    ) -> MessageReceiverResult<usize> {
        Err(NoProcessed) //code = raw_code(WM_APP 到 0xBFFF) - WM_APP + 1,WM_APP = 0x8000
    }
    fn share_messages(
        &mut self,
        id: usize,
        window: &mut Window,
        code: &str,
        msg: RawMessage,
    ) -> MessageReceiverResult<usize> {
        Err(NoProcessed) //code = raw_code(0xC000到0xFFFF) - 0xC000 + 1,字符串消息
    }
    fn system_reserved_messages(
        &mut self,
        id: usize,
        window: &mut Window,
        code: u32,
        msg: RawMessage,
    ) -> MessageReceiverResult<usize> {
        Err(NoProcessed) //code = raw_code(大于 0xFFFF) - 0xFFFF，由系统保留
    }
}
pub fn msg_loop() {
    let mut msg = MSG::default();
    unsafe {
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            let _ = DispatchMessageW(&msg);
        }
    }
}
pub fn stop_msg_loop() {
    unsafe { PostQuitMessage(0) };
}
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct RawMessage(
    ///消息代码
    pub u32,
    ///wparam
    pub usize,
    ///lparam
    pub isize,
);
impl RawMessage {
    pub fn get_msg<T: UnsafeMessage>(&mut self) -> Result<T> {
        unsafe {
            match T::is_self_msg(&self)? {
                false => panic!("The type provided does not match the actual message!"),
                _ => (),
            };
            T::from_raw_msg(*self)
        }
    }
    pub fn get_control_msg<T: Control>(&mut self) -> Result<T::MsgType> {
        self.get_msg::<T::MsgType>()
    }
}
pub trait AsMsg {
    fn as_msg(&self) -> RawMessage; //RawMessage已实现Copy
}
///注意为此类型实现Clone时，也要克隆指针指向的数据
pub unsafe trait UnsafeMessage: Send + Sync {
    type OwnerType: AsMsg;
    ///给你一个RawMessage,判断是否为自身类型消息
    unsafe fn is_self_msg(ptr: &RawMessage) -> Result<bool>;
    ///给你一个RawMessage, 返回一个自身实例(***不检查***)
    unsafe fn from_raw_msg(ptr: RawMessage) -> Result<Self>
    where
        Self: Sized;
    ///转换成RawMessage,self如果存在则RawMessage里的指针（如有）指向的内容一定存在，self被Drop时，应释放指针内容避免内存泄漏
    unsafe fn into_raw_msg(self) -> Result<Self::OwnerType>;
}
pub trait CustomMessage: /*UnsafeMessage*/Send + Sync {
    type DataType;
    fn into_raw_parts(self) -> Result<(u32, Self::DataType)>;
    fn from_raw_parts(code: u32, data: Self::DataType) -> Result<Self>
    where
        Self: Sized;
}
// pub(crate) struct AsteriskMutDynAny(pub *mut dyn Any);
// impl<T: CustomMessage + 'static> UnsafeMessage for T {
//     unsafe fn is_self_msg(ptr: &RawMessage) -> Result<bool> {
//         unsafe {
//             if ptr.1 == 0 {
//                 return Err(win_error!(ERROR_NO_DATA));
//             };
//             Ok((*(ptr.1 as *const TypeId)) == (TypeId::of::<T>()))
//         }
//     }
//     unsafe fn from_raw_msg(ptr: RawMessage) -> Result<Self>
//     where
//         Self: Sized,
//     {
//         unsafe {
//             let RawMessage(code, _, lparam) = ptr;
//             let data = match lparam {
//                 0 => None,
//                 x => Some(&mut *((*(x as *const AsteriskMutDynAny)).0)),
//             };
//             Self::from_raw_parts(code, data)
//         }
//     }
//     unsafe fn into_raw_msg(&mut self) -> Result<Self::OwnerType> {
//         let (code, data) = self.into_raw_parts()?;
//         let (wapper, dataa) = match data {
//             Some(x) => {
//                 let data = AsteriskMutDynAny(x as &mut _);
//                 (&data as *const _ as isize, Some(data))
//             }
//             None => (0isize, None),
//         };
//         let id = TypeId::of::<T>();
//         let datas = (id, dataa);
//         Ok(PtrWapper {
//             ptr: RawMessage(code, &id as *const _ as usize, wapper),
//             owner: Some(Box::new(datas)),
//         })
//     }
// }
// 注释掉了，原因见https://internals.rust-lang.org/t/priorities-for-trait-implementations
// 和https://github.com/rust-lang/rust/issues/37653#issuecomment-749178040
pub trait ShareMessage: CustomMessage {
    ///同一个结构体/枚举表示同一个字符串，注意: 最多同时存在16384（0xFFFF-0xC000+1）个不同的字符串，超出时RegisterWindowMessage将返回0
    fn get_string(&self) -> &str;
}

pub trait ClassMessage: CustomMessage {
    fn get_class(&self) -> WindowClass;
}
#[derive(Eq, PartialEq, Debug)]
pub struct UnsafeControlMsgDefaultOwnerType<D: NotifyMessage> {
    pub msg: RawMessage,
    pub data: Option<D>,
}
impl<D: NotifyMessage> AsMsg for UnsafeControlMsgDefaultOwnerType<D> {
    fn as_msg(&self) -> RawMessage {
        use std::ptr::addr_of;
        match &self.data {
            None => self.msg,
            Some(d) => {
                let result = RawMessage(self.msg.0, self.msg.1, addr_of!(d) as isize);
                result
            }
        }
    }
}
unsafe impl<T: UnsafeControlMsg> UnsafeMessage for T {
    type OwnerType = UnsafeControlMsgDefaultOwnerType<T::NotifyType>;
    unsafe fn into_raw_msg(self) -> Result<Self::OwnerType> {
        unsafe {
            let handle = self.get_control().get_window().handle();
            let ptr = self.into_raw()?;
            Ok(match ptr {
                Left(l) => UnsafeControlMsgDefaultOwnerType {
                    msg: {
                        let id: WindowID = match GetDlgCtrlID(handle){
                            0 => 0,
                            a => a.try_into().expect("The control ID exceeds the WindowID::MAX, the GetDlgCtrlID returned an invalid value."),
                        };
                        RawMessage(
                            WM_COMMAND,
                            ((l as usize) << 16) | (id as usize),
                            handle.0 as isize,
                        )
                    },
                    data: None,
                },
                Right(r) => UnsafeControlMsgDefaultOwnerType {
                    msg: RawMessage(WM_NOTIFY, 0, 0isize),
                    data: Some(r),
                },
            })
        }
    }
    unsafe fn is_self_msg(ptr: &RawMessage) -> Result<bool> {
        unsafe {
            let RawMessage(msg, _, lparam) = ptr;
            match *msg {
                WM_COMMAND => {
                    let param2e = HWND((*lparam) as *mut c_void);
                    T::ControlType::is_self(&(Window::from_handle(param2e)))
                }
                WM_NOTIFY => {
                    if *lparam == 0 {
                        return Err(ERROR_NULL_POINTER);
                    }
                    let ptr = (*((*lparam) as *mut NMHDR)).hwndFrom;
                    T::ControlType::is_self(&(Window::from_handle(ptr)))
                }
                _ => Ok(false),
            }
        }
    }
    unsafe fn from_raw_msg(ptr: RawMessage) -> Result<Self>
    where
        Self: Sized,
    {
        unsafe {
            let RawMessage(msg, wparam, lparam) = ptr;
            match msg {
                WM_COMMAND => {
                    let mut nmhdr = NMHDR {
                        hwndFrom: HWND(lparam as *mut c_void),
                        idFrom: (wparam & 0xffff) as usize,
                        code: ((wparam >> 16) & 0xffff) as u32,
                    };
                    Self::from_msg(&mut nmhdr as *mut _ as usize, true)
                }
                WM_NOTIFY => Self::from_msg(lparam as usize, false),
                _ => Err(ERROR_MSG_CODE_NOT_SUPPORT),
            }
        }
    }
}
