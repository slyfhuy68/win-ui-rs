use super::*;
#[derive(Clone, Eq, PartialEq)]
pub struct Window {
    pub handle: HWND,
}
impl From<HWND> for Window {
    fn from(handle: HWND) -> Self {
        Window { handle }
    }
}
impl Into<HWND> for Window {
    fn into(self) -> HWND {
        self.handle
    }
}
pub type WindowID = u16;
pub enum GetIconMsgiType {
    Big,          //ICON_BIG
    Small,        //ICON_SMALL
    ProgramSmall, //ICON_SMALL2
}
pub struct WindowPosType {
    pub draw_frame: bool,    //SWP_DRAWFRAME
    pub frame_changed: bool, //SWP_FRAMECHANGED
    pub hide: bool,          //SWP_HIDEWINDOW
    pub no_active: bool,     //SWP_NOACTIVATE
    pub no_copy_bytes: bool, //SWP_NOCOPYBITS
    //pub no_move:bool,
    //pub no_owner_z_order: bool, //SWP_ NOOWNERZORDER / SWP_NOREPOSITION
    pub no_redraw: bool,                //SWP_NOREDRAW
    pub no_send_changing_message: bool, //SWP_NOSENDCHANGING
    //pub no_sizing:bool,
    pub show_window: bool, //SWP_SHOWWINDOW
}
impl From<WindowPosType> for SET_WINDOW_POS_FLAGS {
    fn from(wpt: WindowPosType) -> Self {
        let mut flags = SET_WINDOW_POS_FLAGS(0);
        if wpt.draw_frame {
            flags.0 |= SWP_DRAWFRAME.0;
        }
        if wpt.frame_changed {
            flags.0 |= SWP_FRAMECHANGED.0;
        }
        if wpt.hide {
            flags.0 |= SWP_HIDEWINDOW.0;
        }
        if wpt.no_active {
            flags.0 |= SWP_NOACTIVATE.0;
        }
        if wpt.no_copy_bytes {
            flags.0 |= SWP_NOCOPYBITS.0;
        }
        if wpt.no_redraw {
            flags.0 |= SWP_NOREDRAW.0;
        }
        if wpt.no_send_changing_message {
            flags.0 |= SWP_NOSENDCHANGING.0;
        }
        if wpt.show_window {
            flags.0 |= SWP_SHOWWINDOW.0;
        }
        flags
    }
}

// 如果需要从 SET_WINDOW_POS_FLAGS 转换回 WindowPosType，可以这样实现：
impl From<SET_WINDOW_POS_FLAGS> for WindowPosType {
    fn from(flags: SET_WINDOW_POS_FLAGS) -> Self {
        WindowPosType {
            draw_frame: flags.contains(SWP_DRAWFRAME),
            frame_changed: flags.contains(SWP_FRAMECHANGED),
            hide: flags.contains(SWP_HIDEWINDOW),
            no_active: flags.contains(SWP_NOACTIVATE),
            no_copy_bytes: flags.contains(SWP_NOCOPYBITS),
            no_redraw: flags.contains(SWP_NOREDRAW),
            no_send_changing_message: flags.contains(SWP_NOSENDCHANGING),
            show_window: flags.contains(SWP_SHOWWINDOW),
        }
    }
}
pub struct MinMaxInfo {
    pub max_size_x: i32,
    pub max_size_y: i32,
    pub max_position_x: i32,
    pub max_position_y: i32,
    pub min_track_x: i32,
    pub min_track_y: i32,
    pub max_track_x: i32,
    pub max_track_y: i32,
}
/// 定义窗口显示的状态类型。
pub enum ShowWindowType {
    /// 隐藏窗口并激活另一个窗口。
    Hide, // SW_HIDE
    /// 激活并显示窗口。如果窗口最小化、最大化或排列，系统会将其还原到其原始大小和位置。应用程序应在首次显示窗口时指定此标志。
    Normal, // SW_SHOWNORMAL
    /// 激活窗口并将其显示为最小化窗口。
    ShowMinimized, // SW_SHOWMINIMIZED
    /// 激活窗口并显示最大化的窗口。
    ShowMaximized, // SW_SHOWMAXIMIZED
    /// 以最近的大小和位置显示窗口。此值类似于Normal，只是窗口未激活。
    ShowWithoutActivating, // SW_SHOWNOACTIVATE
    /// 激活窗口并以当前大小和位置显示窗口。
    Show, // SW_SHOW
    /// 最小化指定的窗口，并按Z顺序激活下一个顶级窗口。
    Minimize, // SW_MINIMIZE
    /// 将窗口显示为最小化窗口。此值类似于ShowMinimized，但窗口未激活。
    ShowMinNoActivate, // SW_SHOWMINNOACTIVE
    /// 以当前大小和位置显示窗口。此值类似于Show，只是窗口未激活。
    ShowNoActivate, // SW_SHOWNA
    /// 激活并显示窗口。如果窗口最小化、最大化或排列，系统会将其还原到其原始大小和位置。还原最小化窗口时，应用程序应指定此标志。
    Restore, // SW_RESTORE
    /// 根据启动应用程序的程序传递给程序值设置显示状态。
    ShowDefault, // SW_SHOWDEFAULT
    /// 最小化窗口，即使拥有窗口的线程没有响应。仅当最小化不同线程的窗口时，才应使用此标志。
    ForceMinimize, // SW_FORCEMINIMIZE
}
impl Into<SHOW_WINDOW_CMD> for ShowWindowType {
    fn into(self) -> SHOW_WINDOW_CMD {
        match self {
            ShowWindowType::Hide => SW_HIDE,
            ShowWindowType::Normal => SW_SHOWNORMAL,
            ShowWindowType::ShowMinimized => SW_SHOWMINIMIZED,
            ShowWindowType::ShowMaximized => SW_SHOWMAXIMIZED,
            ShowWindowType::ShowWithoutActivating => SW_SHOWNOACTIVATE,
            ShowWindowType::Show => SW_SHOW,
            ShowWindowType::Minimize => SW_MINIMIZE,
            ShowWindowType::ShowMinNoActivate => SW_SHOWMINNOACTIVE,
            ShowWindowType::ShowNoActivate => SW_SHOWNA,
            ShowWindowType::Restore => SW_RESTORE,
            ShowWindowType::ShowDefault => SW_SHOWDEFAULT,
            ShowWindowType::ForceMinimize => SW_FORCEMINIMIZE,
        }
    }
}
impl From<SHOW_WINDOW_CMD> for ShowWindowType {
    fn from(value: SHOW_WINDOW_CMD) -> Self {
        match value {
            SW_HIDE => ShowWindowType::Hide,
            SW_SHOWNORMAL => ShowWindowType::Normal,
            SW_SHOWMINIMIZED => ShowWindowType::ShowMinimized,
            SW_SHOWMAXIMIZED => ShowWindowType::ShowMaximized,
            SW_SHOWNOACTIVATE => ShowWindowType::ShowWithoutActivating,
            SW_SHOW => ShowWindowType::Show,
            SW_MINIMIZE => ShowWindowType::Minimize,
            SW_SHOWMINNOACTIVE => ShowWindowType::ShowMinNoActivate,
            SW_SHOWNA => ShowWindowType::ShowNoActivate,
            SW_RESTORE => ShowWindowType::Restore,
            SW_SHOWDEFAULT => ShowWindowType::ShowDefault,
            SW_FORCEMINIMIZE => ShowWindowType::ForceMinimize,
            _ => ShowWindowType::Normal,
        }
    }
}
pub enum WindowZpos {
    TopMost,
    Top,
    NoTopMost,
    Bottom,
    PriorWindow(Window),
}
impl From<WindowZpos> for HWND {
    fn from(zpos: WindowZpos) -> Self {
        match zpos {
            WindowZpos::TopMost => HWND((-1isize) as *mut c_void), // (HWND)-1
            WindowZpos::Top => HWND(0isize as *mut c_void),        // (HWND)0
            WindowZpos::NoTopMost => HWND((-2isize) as *mut c_void), // (HWND)-2
            WindowZpos::Bottom => HWND(1isize as *mut c_void),     // (HWND)1
            WindowZpos::PriorWindow(hwnd) => hwnd.into(),
        }
    }
}

impl From<HWND> for WindowZpos {
    fn from(hwnd: HWND) -> Self {
        let ptr_value = hwnd.0 as isize;
        match ptr_value {
            -1 => WindowZpos::TopMost,
            0 => WindowZpos::Top,
            -2 => WindowZpos::NoTopMost,
            1 => WindowZpos::Bottom,
            _ => WindowZpos::PriorWindow(hwnd.into()),
        }
    }
}
///windows未公开函数
pub enum WindowZposGroup {
    Default,                 //ZBID_DEFAULT = 0,
    Desktop,                 //ZBID_DESKTOP = 1,
    UIaccess,                //ZBID_UIACCESS = 2,
    ImmersiveIHM,            //ZBID_IMMERSIVE_IHM = 3,
    ImmersiveNotification,   //ZBID_IMMERSIVE_NOTIFICATION = 4,
    ImmersiveAppchrome,      //ZBID_IMMERSIVE_APPCHROME = 5,
    ImmersiveMogo,           //ZBID_IMMERSIVE_MOGO = 6,
    ImmersiveImmersiveEdgy,  //ZBID_IMMERSIVE_EDGY = 7,
    ImmersiveInactiveMobody, //ZBID_IMMERSIVE_INACTIVEMOBODY = 8,
    ImmersiveInactiveDock,   //ZBID_IMMERSIVE_INACTIVEDOCK = 9,
    ImmersiveActiveMobody,   //ZBID_IMMERSIVE_ACTIVEMOBODY = 10,
    ImmersiveActiveDock,     //ZBID_IMMERSIVE_ACTIVEDOCK = 11,
    ImmersiveBackround,      //ZBID_IMMERSIVE_BACKGROUND = 12,
    ImmersiveSearch,         //ZBID_IMMERSIVE_SEARCH = 13,
    GenuineWindows,          //ZBID_GENUINE_WINDOWS = 14,
    ImmersiveRestricted,     //ZBID_IMMERSIVE_RESTRICTED = 15,
    SystemTools,             //ZBID_SYSTEM_TOOLS = 16,
    Lock,                    //ZBID_LOCK = 17,
    AbovelockUx,             //ZBID_ABOVELOCK_UX = 18,
}
impl Default for Window {
    fn default() -> Self {
        Self {
            handle: HWND(NULL_PTR()),
        }
    }
}
pub enum WindowAnimateType {
    Roll {
        hor_positive: bool, //true AW_HOR_POSITIVE false AW_HOR_NEGATIVE
        ver_negative: bool, //true AW_VER_NEGATIVE false AW_VER_POSITIVE
    }, //NULL
    Side {
        hor_positive: bool, //true AW_HOR_POSITIVE false AW_HOR_NEGATIVE
        ver_negative: bool, //true AW_VER_NEGATIVE false AW_VER_POSITIVE
    }, //AW_SLIDE
    Center, //AW_CENTER
    Blend,  //AW_BLEND
}
pub enum WindowAnimateShowType {
    Hide(WindowAnimateType),
    Activate(WindowAnimateType),
    NoActivate(WindowAnimateType),
}
impl Window {
    pub fn adjust_window_rect(
        rect: Rectangle,
        wtype: WindowType,
        have_menu: bool,
    ) -> Result<Rectangle> {
        todo!() //AdjustWindowRectEx
    }
    pub fn redraw_menu_bar(&mut self) -> Result<()> {
        todo!() //DrawMenuBar
    }
    pub fn get_menu(&mut self) -> Result<Menu> {
        todo!() //GetMenu
    }
    pub fn show(&mut self, stype: ShowWindowType) -> Result<bool> {
        Ok(unsafe { ShowWindow(self.handle, stype.into()) }.into())
    }
    pub fn set_menu(&mut self, menu: Option<Menu>) -> Result<()> {
        todo!() //SetMenu
    }
    pub fn get_system_menu(&mut self) -> Menu {
        todo!() //getSystemMenu(__,false)
    }
    pub fn reset_system_menu(&mut self) {
        todo!() //getSystemMenu(__,true)
    }
    pub fn get_class(&self) -> Result<WindowClass> {
        let mut array1 = vec![0u16; 255];
        if unsafe { GetClassNameW(self.handle, &mut array1[..]) } == 0 {
            return Err(Error::empty());
        }
        let result1 = PCWSTR(array1.as_ptr());
        Ok(WindowClass {
            name: result1,
            owner: Some(array1),
        })
    }
    pub fn get_context_help_id(&self) -> Option<u32> {
        match unsafe { GetWindowContextHelpId(self.handle) } {
            0 => None,
            x => Some(x),
        }
    }
    pub fn set_context_help_id(&mut self, help_id: Option<u32>) -> Result<()> {
        let help = match help_id {
            None => 0,
            Some(x) => x,
        };
        unsafe { SetWindowContextHelpId(self.handle, help) }
    }
    pub fn set_z_group(&mut self, pos: WindowZpos, group: WindowZposGroup) -> Result<()> {
        todo!() //SetWindowBand windows未公开api
    }
    pub fn get_z_group(&self) -> Result<WindowZposGroup> {
        todo!() //SetWindowBand windows未公开api
    }
    pub fn arrange_iconic(&mut self) -> Result<u32> {
        todo!() //ArrangeIconicWindows
    }
    pub fn to_top(&mut self) -> Result<()> {
        todo!() //BringWindowToTop
    }
    pub fn minimize(&mut self) -> Result<()> {
        todo!() //CloseWindow
    }
    pub fn destroy(&mut self) -> Result<()> {
        todo!() //DestroyWindow
    }
    pub fn set_animate(
        &mut self,
        time: std::time::Duration,
        atype: WindowAnimateShowType,
    ) -> Result<()> {
        todo!() //AnimateWindow
    }
    pub fn cascade_child(
        &mut self,
        skip_mdi_disabled: bool,
        area: Option<Rectangle>,
        wnd: Option<&[Window]>,
    ) -> Result<u16> {
        todo!() //CascadeWindows
    }
    pub fn get_child_from_point(
        &mut self,
        pos: Point,
        skip_disabled: bool,
        skip_visible: bool,
        skip_transparent: bool,
    ) -> Option<Window> {
        todo!() //ChildWindowFromPointEx
    }
    ///移除id为0的默认项会返回ERROR_NOT_SUPPORTED
    ///# 警告
    ///不能操作其它线程创建的窗口的接收器
    pub fn remove_msg_receiver(&mut self, id: usize) -> Result<()> {
        if id == 0 {
            return Err(win_error!(ERROR_NOT_SUPPORTED));
        }
        unsafe {
            let ptr = self.get_msg_receiver_mut(id)? as *mut Box<CallBackObj>;
            if RemoveWindowSubclass(self.handle, Some(subclass_porc), id).into() {
                let _ = Box::from_raw(ptr);
                Ok(())
            } else {
                return Err(Error::from_win32());
            }
        }
    }
    pub fn add_msg_receiver(&mut self, id: usize, mssc: Box<CallBackObj>) -> Result<()> {
        if id == 0 || self.has_receiver_for(id) {
            return Err(win_error!(ERROR_OBJECT_ALREADY_EXISTS));
        }
        unsafe {
            if SetWindowSubclass(
                self.handle,
                Some(subclass_porc),
                id,
                Box::into_raw(Box::new(mssc)) as usize,
            )
            .into()
            {
                Ok(())
            } else {
                Err(win_error!(ERROR_INVALID_FUNCTION))
            }
        }
    }
    pub fn get_msg_receiver(&self, id: usize) -> Result<&Box<CallBackObj>> {
        unsafe {
            if id == 0 {
                return Ok(&*(get_proc(&self)?));
            };
            let mut data: usize = 0usize;
            if GetWindowSubclass(
                self.handle,
                Some(subclass_porc),
                id,
                Some(&mut data as *mut _),
            )
            .into()
            {
                Ok(&*(data as *const Box<CallBackObj>))
            } else {
                Err(win_error!(ERROR_INVALID_PARAMETER))
            }
        }
    }
    pub fn get_msg_receiver_mut(&mut self, id: usize) -> Result<&mut Box<CallBackObj>> {
        unsafe {
            if id == 0 {
                return Ok(&mut *(get_proc(&self)?));
            };
            let mut data: usize = 0usize;
            if GetWindowSubclass(self.handle, Some(subclass_porc), id, Some(&mut data)).into() {
                Ok(&mut *(data as *mut Box<CallBackObj>))
            } else {
                Err(win_error!(ERROR_INVALID_PARAMETER))
            }
        }
    }
    pub fn has_receiver_for(&mut self, id: usize) -> bool {
        unsafe {
            if id == 0 {
                return true;
            }
            GetWindowSubclass(self.handle, Some(subclass_porc), id, None).into()
        }
    }
    // pub fn force_end(&self) {
    //     unsafe {EndTask(self.handle,false,true)};
    // }

    //pub fn find_window(&mut self,class:Option<WindowClass>,title: Option<&str>){}
    pub fn force_redraw(&mut self) -> Result<()> {
        unsafe {
            SetWindowPos(
                self.handle,
                None,
                0,
                0,
                0,
                0,
                SWP_NOSIZE | SWP_NOMOVE | SWP_NOZORDER,
            )
        }
    }
}
pub fn cascade_window(
    skip_mdi_disabled: bool,
    area: Option<Rectangle>,
    wnd: Option<&[Window]>,
) -> Result<u16> {
    todo!() //CascadeWindows
}
pub fn allow_set_foreground_window(pid: Option<u32>) -> Result<()> {
    todo!() //AllowSetForegroundWindow
}
pub fn have_any_popup_window() -> bool {
    unsafe { AnyPopup() }.as_bool()
}
// pub struct SetWindowsPosCallbackResult(pub HDWP);
// pub fn set_windows_pos<F>(mut cb:F, mut num:u16) -> Result<()>//BeginDeferWindowPos/DeferWindowPos/EndDeferWindowPos
//     where
//         F: FnMut(Box<dyn FnOnce(Window, Option<WindowZpos>, Option<Point>, Option<Size>, WindowPosType) -> Result<SetWindowsPosCallbackResult>>) -> Result<SetWindowsPosCallbackResult>
// {
//     let mut dp1 = unsafe {
//         BeginDeferWindowPos(num as i32)?
//         };
//         while num > 0 {
//             let defer = move |wnd:Window, p:Option<WindowZpos>, xy:Option<Point>, wh:Option<Size>, t:WindowPosType| -> Result<SetWindowsPosCallbackResult> {
//                 let mut flag: SET_WINDOW_POS_FLAGS = t.into();
//                 let Point(x, y) = if let Some(z) = xy {z} else {flag|= SWP_NOREPOSITION;Point(0, 0)};
//                 let Size(w, h) = if let Some(z) = wh {z} else {flag|= SWP_NOSIZE;Size(0, 0)};
//                 unsafe{
//                     Ok(SetWindowsPosCallbackResult(DeferWindowPos(dp1,
//                         wnd.into(),
//                         Some(if let Some(x) = p {x.into()} else {flag|= SWP_NOZORDER;HWND(0isize as *mut c_void)}),
//                         x, y, w, h,
//                         flag)?))
//                 }
//             };
//             let SetWindowsPosCallbackResult(dp1) = cb(Box::new(defer))?;
//             num -= 1;
//         }
//         unsafe {
//         EndDeferWindowPos(dp1)}
//
// }
//
// pub fn set_windows_pos<F>(mut cb:F, mut num:u16) -> Result<()>
// where
//     F: FnMut(HDWP, Box<dyn FnOnce(Window, Option<WindowZpos>, Option<Point>, Option<Size>, WindowPosType) -> Result<SetWindowsPosCallbackResult>>) -> Result<SetWindowsPosCallbackResult>
// {
//     let mut dp1 = unsafe { BeginDeferWindowPos(num as i32)? };
//
//     while num > 0 {
//         // 创建一个闭包，该闭包接收 dp1 并在内部使用它。
//         let defer = move |wnd:Window, p:Option<WindowZpos>, xy:Option<Point>, wh:Option<Size>, t:WindowPosType| -> Result<SetWindowsPosCallbackResult> {
//             let mut flag: SET_WINDOW_POS_FLAGS = t.into();
//             let Point(x, y) = if let Some(z) = xy {z} else {flag|= SWP_NOREPOSITION;Point(0, 0)};
//             let Size(w, h) = if let Some(z) = wh {z} else {flag|= SWP_NOSIZE;Size(0, 0)};
//             unsafe {
//                 Ok(SetWindowsPosCallbackResult(DeferWindowPos(dp1,
//                     wnd.into(),
//                     Some(if let Some(x) = p {x.into()} else {flag|= SWP_NOZORDER;HWND(0isize as *mut c_void)}),
//                     x, y, w, h,
//                     flag)?))
//             }
//         };
//
//         // 更新 dp1 为 cb 返回的新 HDWP 值
//         let SetWindowsPosCallbackResult(dp1) = cb(dp1, Box::new(defer))?;
//         num -= 1;
//     }
//
//     unsafe {
//         EndDeferWindowPos(dp1)
//     }
// }
