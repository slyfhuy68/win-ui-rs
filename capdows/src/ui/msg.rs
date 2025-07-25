use super::*;
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
// impl MessageReceiverError {
//     pub fn code(&self) -> i32 {
//         match self {
//             WinErr(x) => x.code(),
//             NoProcessed => -255i32,
//         }
//     }
// }
// impl Into<Error> for MessageReceiverError {
//      fn from(self) -> Self {
//              panic!()
//      }
// }
pub unsafe trait RawMessageHandler {
    unsafe fn handle_msg(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        callback_id: usize,
    ) -> Option<isize>;
    #[inline]
    unsafe fn handle_normal_msg(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<isize> {
        unsafe { Self::handle_msg(hwnd, msg, wparam, lparam, 0) }
    }
}
#[repr(C)]
#[allow(non_snake_case)]
struct NMHDRSTATIC {
    #[allow(non_snake_case)]
    nmhdr: NMHDR,
    #[allow(non_snake_case)]
    DC: HANDLE,
}
macro_rules! do_msg {
    ($cb:expr) => {
        match $cb {
            Ok(x) => Some(x),
            Err(NoProcessed) => None,
            Err(_) => Some(0isize),
        }
    };
}
macro_rules! do_nofity {
    ($cb:expr) => {
        match $cb {
            Ok(()) => Some(0isize),
            Err(NoProcessed) => None,
            Err(_) => Some(0isize),
        }
    };
}
unsafe impl<C: MessageReceiver + Sync + 'static> RawMessageHandler for C {
    unsafe fn handle_msg(
        hwnd: HWND,
        msg: u32,
        param1: WPARAM,
        param2: LPARAM,
        callback_id: usize,
    ) -> Option<isize> {
        unsafe {
            let mut w = Window::from_handle(hwnd);
            use MessageReceiverError::*;
            let result = match msg {
                WM_CREATE => {
                    let s = *(param2 as *mut CREATESTRUCTW);
                    let (mut wc, _buffer) = {
                        let mut buffer = [0u16; 256];
                        let len = GetClassNameW(hwnd, buffer.as_mut_ptr(), 256) as usize;
                        if len == 0 {
                            (WindowClass::from_raw(s.lpszClass), None)
                        } else {
                            let mut vec = buffer[..len].to_vec();
                            vec.push(0);
                            (WindowClass::from_raw(vec.as_ptr() as PCWSTR), Some(vec))
                        }
                    };
                    let mut wtype = WindowType::from_data(
                        s.style as WINDOW_STYLE,
                        s.dwExStyle,
                        s.hMenu,
                        s.hwndParent,
                    );
                    unsafe extern "C" {
                        unsafe fn wcslen(s: *const u16) -> usize;
                    }
                    let len = wcslen(s.lpszName);
                    let result = match C::create(
                        callback_id,
                        &mut w,
                        &(String::from_utf16(std::slice::from_raw_parts(s.lpszName, len))
                            .unwrap_or(String::from(""))),
                        &mut wc,
                        &s.hInstance.into(),
                        rect(s.x, s.y, s.cx, s.cy),
                        &mut wtype,
                    ) {
                        Ok(x) => Some(match x {
                            true => 0isize,
                            false => -1isize,
                        }),
                        Err(NoProcessed) => None,
                        Err(_) => Some(-1isize),
                    };
                    use std::mem::ManuallyDrop;
                    let _ = ManuallyDrop::new((wtype, wc));
                    result
                }
                WM_DESTROY => do_nofity! {C::destroy(callback_id, &mut w)
                },
                WM_COMMAND => {
                    if param2 != 0 {
                        let param2e = param2;
                        let param1e = param1;
                        do_msg! {C::control_message(
                            callback_id,
                            &mut w,
                            &mut RawMessage(WM_COMMAND, param1e, param2e),
                            (param1e & 0xffff) as WindowID,
                        ) }
                    } else {
                        let high = ((param1 >> 16) & 0xffff) as u8;
                        let low = (param1 & 0xffff) as u16;
                        match high {
                            0 => {
                                do_nofity! {C::menu_command(
                                    callback_id,
                                    &mut w,
                                    MenuCommandMsgItemPos::CostomId(low as MenuItemID),
                                )
                                }
                            }
                            // 1 => ,//加速器
                            _ => None,
                        }
                    }
                }
                WM_NOTIFYFORMAT => {
                    Some(2isize) //此crate只能创建Unicode窗口NFR_UNICODE
                }
                WM_MENUCOMMAND => {
                    let mut hmenu = param2 as HMENU;
                    do_nofity! { C::menu_command(
                        callback_id,
                        &mut w,
                        MenuCommandMsgItemPos::Position(Menu::from_mut_ref(&mut hmenu), param1 as u16),
                    )
                    }
                }
                WM_NOTIFY => {
                    let nmhdr_ptr = param2 as *mut NMHDR;
                    do_msg! {C::control_message(
                        callback_id,
                        &mut w,
                        &mut RawMessage(WM_NOTIFY, 0, nmhdr_ptr as isize),
                        (*nmhdr_ptr).idFrom as WindowID,
                    )}
                }
                WM_CTLCOLORSTATIC => {
                    let mut nmhdr = NMHDRSTATIC {
                        nmhdr: NMHDR {
                            hwndFrom: param2 as HWND,
                            idFrom: GetWindowLongW(param2 as HWND, GWL_ID) as usize,
                            code: WM_CTLCOLORSTATIC,
                        },
                        DC: param1 as HANDLE,
                    };
                    let nmhdr_ptr: *mut NMHDRSTATIC = &mut nmhdr;
                    do_msg! {C::control_message(
                        callback_id,
                        &mut w,
                        &mut RawMessage(WM_NOTIFY, 0, nmhdr_ptr as isize),
                        nmhdr.nmhdr.idFrom as WindowID,
                    )
                    }
                }
                WM_NULL => do_nofity! {C::alive_test(callback_id, &mut w)
                },
                WM_LBUTTONDOWN => {
                    do_nofity! {C::mouse_msg(
                        callback_id,
                        &mut w,
                        MouseMsg::Button {
                            button_type: MouseButton::Left,
                            state: ButtonState::Down,
                            is_nc: false,
                            pos: point2(
                                (param2 & 0xFFFF) as u16 as i32,
                                (param2 >> 16) as u16 as i32,
                            ),
                        },
                    )
                    }
                }
                WM_LBUTTONUP => {
                    do_nofity! {C::mouse_msg(
                        callback_id,
                        &mut w,
                        MouseMsg::Button {
                            button_type: MouseButton::Left,
                            state: ButtonState::Up,
                            is_nc: false,
                            pos: point2(
                                (param2 & 0xFFFF) as u16 as i32,
                                (param2 >> 16) as u16 as i32,
                            ),
                        },
                    )
                    }
                }
                _ => None,
            };
            w.nullify();
            result
        }
    }
}
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
#[derive(Debug)]
pub enum MenuCommandMsgItemPos<'a> {
    CostomId(MenuItemID),
    Position(&'a mut Menu, u16),
}
///每个回调的id表示一个窗口的接收器id，如果这是一个子类化接收器，NoProcessed表示调用子类链上一个接收器，id为子类化id，如果不是，那么id为0，NoProcessed表示进行默认处理
#[allow(unused_variables)]
pub trait MessageReceiver: Default {
    // fn activating()包含WM_MOUSEACTIVATE
    fn mouse_msg(id: usize, window: &mut Window, msg: MouseMsg) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn menu_command(
        id: usize,
        window: &mut Window,
        item: MenuCommandMsgItemPos,
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    ///WM_NULL, 用于系统测试程序是否响应，一般不处理
    fn alive_test(id: usize, window: &mut Window) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn control_message(
        id: usize,
        window: &mut Window,
        msg: &mut RawMessage,
        wnd_id: WindowID,
    ) -> MessageReceiverResult<isize> {
        Err(NoProcessed)
    }
    ///itype参数：这只是[`crate::ui::class::WindowClass`]的crate_window方法的参数的一个副本，但是你可以调用所有者/父窗口和菜单上面的方法，因为它们本质是指针
    fn create(
        id: usize,
        window: &mut Window,
        name: &str,
        class: &mut WindowClass,
        file: &ExecutableFile,
        pos: Rect,
        itype: &mut WindowType,
        //ex_data: usize,
    ) -> MessageReceiverResult<bool> {
        Err(NoProcessed)
    } //true 0 false -1
    fn destroy(id: usize, window: &mut Window) -> MessageReceiverResult<()> {
        stop_msg_loop();
        Ok(())
    }
    fn class_messages(
        id: usize,
        window: &mut Window,
        code: u16,
        msg: RawMessage,
    ) -> MessageReceiverResult<usize> {
        Err(NoProcessed) //code = raw_code(WM_USER 到 0x7FFF) - WM_USER + 1,WM_USER = 0x0400
    }
    fn applications_messages(
        id: usize,
        window: &mut Window,
        code: u16,
        msg: RawMessage,
    ) -> MessageReceiverResult<usize> {
        Err(NoProcessed) //code = raw_code(WM_APP 到 0xBFFF) - WM_APP + 1,WM_APP = 0x8000
    }
    fn share_messages(
        id: usize,
        window: &mut Window,
        code: &str,
        msg: RawMessage,
    ) -> MessageReceiverResult<usize> {
        Err(NoProcessed) //code = raw_code(0xC000到0xFFFF) - 0xC000 + 1,字符串消息
    }
    fn system_reserved_messages(
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
        while GetMessageW(&mut msg, 0 as HWND, 0, 0) != 0 {
            let _ = TranslateMessage(&msg);
            let _ = DispatchMessageW(&msg);
        }
    }
}
pub fn stop_msg_loop() {
    unsafe { PostQuitMessage(0) };
}
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct RawMessage(
    ///消息代码
    pub u32,
    ///wparam
    pub WPARAM,
    ///lparam
    pub LPARAM,
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
                            handle as isize,
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
                    let param2e = (*lparam) as HWND;
                    T::ControlType::is_self(Window::from_ref(&param2e))
                }
                WM_NOTIFY => {
                    if *lparam == 0 {
                        return Err(ERROR_NULL_POINTER);
                    }
                    let ptr = (*((*lparam) as *mut NMHDR)).hwndFrom;
                    T::ControlType::is_self(Window::from_ref(&ptr))
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
                        hwndFrom: lparam as HWND,
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
