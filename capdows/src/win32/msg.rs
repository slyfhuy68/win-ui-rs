use super::*;
pub type CallBackObj = Box<dyn MessageReceiver + std::marker::Send + std::marker::Sync + 'static>;
#[derive(Clone, Eq, PartialEq)]
pub struct WindowSizeCalcType {
    pub top_align: Option<bool>,  //None NULL true WVR_ALIGNTOP false WVR_ALIGNBOTTOM
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
        msg: RawMassage,
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
    } //默认返回true
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
        xy:Option<Point>,
        wh:Option<Size>,
        ptype: WindowPosType
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn pos_changing(
        &mut self,
        window: &mut Window,
        z_pos: Option<WindowZpos>,
        xy:Option<Point>,
        wh:Option<Size>,
        ptype: WindowPosType
    ) -> MessageReceiverResult<()> {
        Err(NoProcessed)
    }
    fn class_messages(&mut self,window: &mut Window, id:u16, msg: RawMassage) -> MessageReceiverResult<isize>{
        Err(NoProcessed)
    }
    fn applications_messages(&mut self,window: &mut Window, id:u16, msg: RawMassage) -> MessageReceiverResult<isize>{
        Err(NoProcessed)
    }
    fn share_messages(&mut self,window: &mut Window, id:&str, msg: RawMassage) -> MessageReceiverResult<isize>{
        Err(NoProcessed)
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
pub struct RawMassage(usize);
impl RawMassage {
    pub unsafe fn new(ptr:usize) -> Self {
        Self(ptr)
    }
    pub unsafe fn as_ptr(self) -> usize {
        let RawMassage(result) = self;
        result
    }
    pub fn get_msg<T:CustomMessage>(self) -> MessageReceiverResult<T> {
        match unsafe { T::is_self(self.0) } {
            Ok(false) => panic!("The type provided does not match the actual message!"),
            Ok(true) => {
                if let Some(x) = unsafe { T::from_msg(self.0) } {
                    Ok(*x)
                } else {
                    Err(NoProcessed)
                }
            }
            Err(_) => Err(NoProcessed),
        }
    }
    pub fn get_control_msg<C:Control>(self) -> MessageReceiverResult<C::MsgType> {
        self.get_msg::<C::MsgType>()
    }

}
#[repr(C)]
pub struct CustomMessageHead {
        id:u32
}
pub trait CustomMessage {
    ///给你一个指针，判断是否为自身类型消息
    unsafe fn is_self(ptr: usize) -> Result<bool>;
    ///给你一个原始指针（lParam）返回一个自身实例(***不检查***)
    unsafe fn from_msg(ptr: usize) -> Option<Box<Self>>;
    ///转换成原始指针, 原始指针推荐指向以CustomMessageHead作为第一个成员的指针
    unsafe fn into_raw(&mut self) -> usize;
}
pub trait ShareMessage: CustomMessage {
    fn get_string(&self) -> &str;
}
pub trait ClassMessage: CustomMessage {
    fn get_class(&self) -> WindowClass;
}