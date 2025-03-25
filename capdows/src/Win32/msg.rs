use super::*;
pub type CallBackObj = Box<dyn MessageReceiver + std::marker::Send + std::marker::Sync + 'static>;
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
            WinErr(x) => x.code().0,
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
pub trait MessageReceiver {
    fn control_message(
        &mut self,
        window: &mut Window,
        msg: RawMessage,
        id: WindowID,
    ) -> MessageReceiverResult<isize> {
        Err(NoProcessed)
    }
    fn activate_app(&mut self, window: &mut Window, state: bool) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    } //WM_ACTIVATEAPP
    fn cancel_mode(&mut self, window: &mut Window) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn child_activate(&mut self, window: &mut Window) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn close(&mut self, window: &mut Window) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn insufficient_memory(
        &mut self,
        window: &mut Window,
        percent: u16,
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn create(
        &mut self,
        window: &mut Window,
        name: &str,
        class: WindowClass,
        file: ExecutableFile,
        pos: Rectangle,
        itype: WindowType,
        //ex_data: usize,
    ) -> MessageReceiverResult<bool> {
        Err(NoProcessed)
    } //true 0 false -1
    fn destroy(&mut self, window: &mut Window) -> MessageReceiverResult<()> {
        unsafe { PostQuitMessage(0) };
        Ok(())
    }
    fn enable(&mut self, window: &mut Window, state: bool) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn enter_size_move(&mut self, window: &mut Window) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn exit_size_move(&mut self, window: &mut Window) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn get_icon(
        &mut self,
        window: &mut Window,
        itype: GetIconMsgiType,
        dpi: i64,
    ) -> MessageReceiverResult<Icon> {
        Err(NoProcessed)
    }
    fn get_min_max_info(
        &mut self,
        window: &mut Window,
        info: &mut MinMaxInfo,
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    // fn change_input_lang(&mut self,window: &mut Window) -> () {todo!()}
    // fn change_input_lang_reques(&mut self,window: &mut Window) -> () {todo!()}
    fn moved(&mut self, window: &mut Window, x: i32, y: i32) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn moveing(
        &mut self,
        window: &mut Window,
        pos: (i32, i32, i32, i32),
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    } //未处理返false
    fn nc_activate(
        &mut self,
        window: &mut Window,
        draw: bool,
        handle: Option<Option<HANDLE>>,
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    } //未处理返true
    fn nc_calc_size_client_area(
        &mut self,
        window: &mut Window,
        new_window_coordinates: Rectangle,
        original_window_coordinates: Rectangle,
        original_work_area_coordinates: Rectangle,
        z_pos: WindowZpos,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        ptype: WindowPosType,
    ) -> MessageReceiverResult<Option<WindowSizeCalcType>> /*None:WVR_VALIDRECTS*/ {
        Err(NoProcessed)
    }
    fn nc_crate(
        &mut self,
        window: &mut Window,
        windowname: &str,
        classname: &str,
        windows_style: u32,
        windows_ex_style: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: u32,
        menu: usize,
        additional_application_data: usize,
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn nc_destroy(&mut self, window: &mut Window) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn null_message(&mut self, window: &mut Window) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn query_drag_icon(&mut self, window: &mut Window) -> MessageReceiverResult<Option<Cursor>> {
        Err(NoProcessed)
    }
    fn query_open(&mut self, window: &mut Window) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn show_state_change(
        &mut self,
        window: &mut Window,
        showing: bool,
        state: Option<ShowStateChangeState>,
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn sized(
        &mut self,
        window: &mut Window,
        stype: SizedMsgType,
        new_hight: i32,
        new_width: i32,
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn sizeing(
        &mut self,
        window: &mut Window,
        stype: SizingMsgType,
        coordinates: &mut Rectangle,
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn style_changed(
        &mut self,
        window: &mut Window,
        old: WindowType,
        new: WindowType,
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn style_changing(
        &mut self,
        window: &mut Window,
        old: WindowType,
        new: &mut WindowType,
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn theme_changed(&mut self, window: &mut Window) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    //fn user_changed(&mut self,window: Window) -> MessageReceiverResult<()> {Err(NoProcessed)}
    fn pos_changed(
        &mut self,
        window: &mut Window,
        z_pos: Option<WindowZpos>,
        xy: Option<Point>,
        wh: Option<Size>,
        ptype: WindowPosType,
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn pos_changing(
        &mut self,
        window: &mut Window,
        z_pos: Option<WindowZpos>,
        xy: Option<Point>,
        wh: Option<Size>,
        ptype: WindowPosType,
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn class_messages(
        &mut self,
        window: &mut Window,
        code: u16,
        msg: RawMessage,
    ) -> MessageReceiverResult<usize> {
        Err(NoProcessed) //code = raw_code(WM_USER 到 0x7FFF) - WM_USER + 1,WM_USER = 0x0400
    }
    fn applications_messages(
        &mut self,
        window: &mut Window,
        code: u16,
        msg: RawMessage,
    ) -> MessageReceiverResult<usize> {
        Err(NoProcessed) //code = raw_code(WM_APP 到 0xBFFF) - WM_APP + 1,WM_APP = 0x8000
    }
    fn share_messages(
        &mut self,
        window: &mut Window,
        code: &str,
        msg: RawMessage,
    ) -> MessageReceiverResult<usize> {
        Err(NoProcessed) //code = raw_code(0xC000到0xFFFF) - 0xC000 + 1,字符串消息
    }
    fn system_reserved_messages(
        &mut self,
        window: &mut Window,
        code: u32,
        msg: RawMessage,
    ) -> MessageReceiverResult<usize> {
        Err(NoProcessed) //code = raw_code(大于 0xFFFF) - 0xFFFF，由系统保留
    }
}
pub fn msg_loop() -> () {
    let mut msg = MSG::default();
    unsafe {
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            let _ = DispatchMessageW(&msg);
        }
    }
}
#[derive(Copy, Clone)]
pub struct RawMessage(pub u32, pub usize, pub isize);
impl RawMessage{
    fn get_msg<T: CustomMessage>(self) -> Result<T> {
        unsafe {
        match T::is_self_msg(&self)? {
            false => panic!("The type provided does not match the actual message!"), 
            _ => ()
        };
        match T::from_raw_msg(self) {
            Ok(x) => Ok(*x), 
            Err(e) => Err(e)
        }
        }
    }
}
pub trait CustomMessage {
    ///给你一个RawMessage,判断是否为自身类型消息
    unsafe fn is_self_msg(ptr: &RawMessage) -> Result<bool>;
    ///给你一个RawMessage,返回一个自身实例(***不检查***)
    unsafe fn from_raw_msg(ptr: RawMessage) -> Result<Box<Self>>;
    ///转换成RawMessage,self如果存在则RawMessage里的指针（如有）指向的内容一定存在，self被Drop时，应释放指针内容避免内存泄漏
    unsafe fn into_raw_msg(&mut self) -> Result<RawMessage>;
}
pub trait ShareMessage: CustomMessage {
    ///同一个结构体/枚举表示同一个字符串，注意: 最多同时存在16384（0xFFFF-0xC000+1）个不同的字符串，超出时RegisterWindowMessage将返回0
    fn get_string(&self) -> String;
}

pub trait ClassMessage: CustomMessage {
    fn get_class(&self) -> WindowClass;
}
impl<T: ControlMsg> CustomMessage for T {
    unsafe fn into_raw_msg(&mut self) -> Result<RawMessage> {
        unsafe {
            let ptr = self.into_raw()?;
            Ok(match ptr {
                Left(l) => {
                    let handle = self.get_control().to_window().handle;
                    let id: WindowID = match GetDlgCtrlID(handle){
                    0 => 0,
                    a => a.try_into().expect("The control ID exceeds the WindowID::MAX, the GetDlgCtrlID returned an invalid value."),
                };
                    RawMessage(
                        WM_COMMAND,
                        ((l as usize) << 16) | (id as usize),
                        handle.0 as isize,
                    )
                }
                Right(r) => RawMessage(WM_NOTIFY, 0, r as isize),
            })
        }
    }
    unsafe fn is_self_msg(ptr: &RawMessage) -> Result<bool> {
        unsafe {
            let RawMessage(msg, wparam, lparam) = ptr;
            match *msg {
                WM_COMMAND => {
                    let param2e = HWND((*lparam) as *mut c_void);
                    T::ControlType::is_self(&param2e)
                }
                WM_NOTIFY => {
                    if *lparam == 0{
                        return Err(win_error!(ERROR_BAD_ARGUMENTS));
                    }
                    let ptr = (*((*lparam) as *mut NMHDR)).hwndFrom;
                    T::ControlType::is_self(&ptr)
                }
                _ => Ok(false),
            }
        }
    }
    unsafe fn from_raw_msg(ptr: RawMessage) -> Result<Box<Self>> {
        unsafe {
            let RawMessage(msg, wparam, lparam) = ptr;
            match msg {
                WM_COMMAND => {
                    let mut nmhdr = NMHDR {
                        hwndFrom: HWND(lparam as *mut c_void),
                        idFrom: (wparam & 0xffff) as usize,
                        code: ((wparam >> 16) & 0xffff) as u32,
                    };
                    Self::from_msg(&mut nmhdr as *mut _ as usize)
                }
                WM_NOTIFY => Self::from_msg(lparam as usize),
                _ => Err(Error::new(ERROR_INVALID_DATA.into(), "")),
            }
        }
    }
}
