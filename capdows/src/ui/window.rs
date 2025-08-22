use super::*;
use std::fmt;
//不实现Debug已手动实现
#[repr(transparent)]
///已明确由Rust拥有的窗口
///如果一个窗口不一定是rust拥有的而又要调用这上面的方法，请使用Window::from_ref直接创建引用
pub struct Window {
    handle: HWND,
}
unsafe impl WindowLike for Window {
    #[inline]
    fn from_hwnd_ref(handle: &HWND) -> &Self {
        unsafe { std::mem::transmute(handle) }
    }
    #[inline]
    fn from_hwnd_mut(handle: &mut HWND) -> &mut Self {
        unsafe { std::mem::transmute(handle) }
    }
}
impl AsRef<Window> for Window {
    #[inline]
    fn as_ref(&self) -> &Window {
        self
    }
}
impl AsMut<Window> for Window {
    #[inline]
    fn as_mut(&mut self) -> &mut Window {
        self
    }
}
impl Default for Window {
    fn default() -> Self {
        Window { handle: 0 as HWND }
    }
}
// #[cfg(debug_assertions)]
// impl Drop for Window {
//     fn drop(&mut self) {
//         if !(std::thread::panicking() || self.handle.is_null()) {
//             println!("debug, {:?}", self);
//             println!("Backtrace:\n{}", std::backtrace::Backtrace::capture());
//             println!(
//                 "note: run with `RUST_BACKTRACE=1` or `RUST_BACKTRACE=full` for a verbose backtrace."
//             );
//         }
//     }
// }
unsafe impl Send for Window {}
unsafe impl Sync for Window {}
impl fmt::Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Window").field(&self.handle).finish()
    }
}
impl Into<HWND> for Window {
    #[inline]
    fn into(self) -> HWND {
        self.handle
    }
}
impl From<HWND> for Window {
    #[inline]
    fn from(handle: HWND) -> Self {
        Self { handle }
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
impl Into<SET_WINDOW_POS_FLAGS> for WindowPosType {
    fn into(self) -> SET_WINDOW_POS_FLAGS {
        let mut flags = 0u32;
        set_style(&mut flags, SWP_DRAWFRAME, self.draw_frame);
        set_style(&mut flags, SWP_FRAMECHANGED, self.frame_changed);
        set_style(&mut flags, SWP_HIDEWINDOW, self.hide);
        set_style(&mut flags, SWP_NOACTIVATE, self.no_active);
        set_style(&mut flags, SWP_NOCOPYBITS, self.no_copy_bytes);
        set_style(&mut flags, SWP_NOREDRAW, self.no_redraw);
        set_style(
            &mut flags,
            SWP_NOSENDCHANGING,
            self.no_send_changing_message,
        );
        set_style(&mut flags, SWP_SHOWWINDOW, self.show_window);
        flags
    }
}
impl From<SET_WINDOW_POS_FLAGS> for WindowPosType {
    fn from(flags: SET_WINDOW_POS_FLAGS) -> Self {
        WindowPosType {
            draw_frame: ucontain(flags, SWP_DRAWFRAME),
            frame_changed: ucontain(flags, SWP_FRAMECHANGED),
            hide: ucontain(flags, SWP_HIDEWINDOW),
            no_active: ucontain(flags, SWP_NOACTIVATE),
            no_copy_bytes: ucontain(flags, SWP_NOCOPYBITS),
            no_redraw: ucontain(flags, SWP_NOREDRAW),
            no_send_changing_message: ucontain(flags, SWP_NOSENDCHANGING),
            show_window: ucontain(flags, SWP_SHOWWINDOW),
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
#[repr(i32)]
pub enum ShowWindowType {
    /// 隐藏窗口并激活另一个窗口。
    Hide = SW_HIDE,
    /// 激活并显示窗口。如果窗口最小化、最大化或排列，系统会将其还原到其原始大小和位置。应用程序应在首次显示窗口时指定此标志。
    Normal = SW_SHOWNORMAL,
    /// 激活窗口并将其显示为最小化窗口。
    ShowMinimized = SW_SHOWMINIMIZED,
    /// 激活窗口并显示最大化的窗口。
    ShowMaximized = SW_SHOWMAXIMIZED,
    /// 以最近的大小和位置显示窗口。此值类似于Normal，只是窗口未激活。
    ShowWithoutActivating = SW_SHOWNOACTIVATE,
    /// 激活窗口并以当前大小和位置显示窗口。
    Show = SW_SHOW,
    /// 最小化指定的窗口，并按Z序激活下一个顶级窗口。
    Minimize = SW_MINIMIZE,
    /// 将窗口显示为最小化窗口。此值类似于ShowMinimized，但窗口未激活。
    ShowMinNoActivate = SW_SHOWMINNOACTIVE,
    /// 以当前大小和位置显示窗口。此值类似于Show，只是不激活窗口。
    ShowNoActivate = SW_SHOWNA,
    /// 激活并显示窗口。如果窗口最小化、最大化或排列，系统会将其还原到其原始大小和位置。还原最小化窗口时，应用程序应指定此标志。
    Restore = SW_RESTORE,
    /// 根据启动应用程序的程序传递给程序值设置显示状态。
    ShowDefault = SW_SHOWDEFAULT,
    /// 最小化窗口，即使拥有窗口的线程没有响应。仅当最小化不同线程的窗口时，才应使用此标志。
    ForceMinimize = SW_FORCEMINIMIZE,
}
impl ShowWindowType {
    #[inline]
    pub fn get_show_window_type() -> Result<Self> {
        let mut info = STARTUPINFOW::default();
        unsafe {
            GetStartupInfoW(&mut info);
        }
        Ok((info.wShowWindow as SHOW_WINDOW_CMD).try_into()?)
    }
}
impl Into<SHOW_WINDOW_CMD> for ShowWindowType {
    fn into(self) -> SHOW_WINDOW_CMD {
        self as SHOW_WINDOW_CMD
    }
}
impl TryFrom<SHOW_WINDOW_CMD> for ShowWindowType {
    type Error = Error;
    fn try_from(value: SHOW_WINDOW_CMD) -> Result<Self> {
        Ok(match value {
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
            _ => return Err(ERROR_INVALID_DATA),
        })
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
            WindowZpos::TopMost => -1isize as HWND,   // (HWND)-1
            WindowZpos::Top => 0isize as HWND,        // (HWND)0
            WindowZpos::NoTopMost => -2isize as HWND, // (HWND)-2
            WindowZpos::Bottom => 1isize as HWND,     // (HWND)1
            WindowZpos::PriorWindow(hwnd) => hwnd.into(),
        }
    }
}

impl WindowZpos {
    ///如果HWND是一个窗口，确保它是Rust拥有的
    pub unsafe fn from_handle(hwnd: HWND) -> Self {
        let ptr_value = hwnd as isize;
        match ptr_value {
            -1 => WindowZpos::TopMost,
            0 => WindowZpos::Top,
            -2 => WindowZpos::NoTopMost,
            1 => WindowZpos::Bottom,
            _ => WindowZpos::PriorWindow(unsafe { Window::from_handle(hwnd) }),
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
    #[inline]
    pub fn handle_eq(&self, other: &Window) -> bool {
        self.handle == other.handle
    }
    #[inline]
    pub fn from_mut_ref(handle: &mut HWND) -> &mut Window {
        unsafe { std::mem::transmute(handle) }
    }
    #[inline]
    pub fn from_ref(handle: &HWND) -> &Window {
        unsafe { std::mem::transmute(handle) }
    }
    #[inline]
    pub unsafe fn from_handle(handle: HWND) -> Self {
        Window { handle }
    }
    #[inline]
    pub fn parent(&self) -> Option<Self> {
        unsafe {
            let hwnd = GetAncestor(self.handle, GA_PARENT);
            if hwnd.is_null() {
                None
            } else {
                Some(Window::from_handle(hwnd))
            }
        }
    }
    #[inline]
    pub fn is_child(&self) -> bool {
        unsafe { GetWindowLongPtrW(self.handle, GWL_STYLE) & WS_CHILD as isize != 0 }
    }
    #[inline]
    pub fn with_menu<F, T>(&mut self, f: F) -> Result<T>
    where
        F: FnOnce(&mut Menu) -> T,
    {
        unsafe {
            if self.is_child() {
                return Err(ERROR_MUSTNOT_CHILD);
            };
            let mut menu = GetMenu(self.handle);
            if menu.is_null() {
                return Err(ERROR_NOT_FOUND_MENU);
            };
            Ok(f(Menu::from_mut_ref(&mut menu)))
        }
    }
    #[inline]
    pub fn with_child<F, T>(&self, id: WindowID, f: F) -> Result<T>
    where
        F: FnOnce(&Window) -> T,
    {
        let hwnd = error_from_win32!(GetDlgItem(self.handle, id as i32))?;
        Ok(f(Self::from_ref(&hwnd)))
    }
    #[inline]
    pub fn as_ctl_mut<C: RawHwndControl>(&mut self) -> Result<&mut C> {
        C::from_hwnd_ref_mut(&mut self.handle)
    }
    #[inline]
    pub fn as_ctl<C: RawHwndControl>(&self) -> Result<&C> {
        C::from_hwnd_ref(&self.handle)
    }
    #[inline]
    pub fn with_child_mut<F, T>(&mut self, id: WindowID, f: F) -> Result<T>
    where
        F: FnOnce(&mut Window) -> T,
    {
        let mut hwnd = error_from_win32!(GetDlgItem(self.handle, id as i32))?;
        Ok(f(Self::from_mut_ref(&mut hwnd)))
    }
    #[inline]
    pub fn root_parent(&self) -> Option<Self> {
        unsafe {
            let hwnd = GetAncestor(self.handle, GA_ROOT);
            if hwnd.is_null() {
                None
            } else {
                Some(Window::from_handle(hwnd))
            }
        }
    }
    #[inline]
    pub fn copy_handle(&self) -> Self {
        Self {
            handle: self.handle,
        }
    }
    #[inline]
    pub unsafe fn handle(&self) -> HWND {
        self.handle
    }
    #[inline]
    pub unsafe fn move_out(&mut self) -> Window {
        let wnd = self.copy_handle();
        self.nullify();
        wnd
    }
    #[inline]
    pub fn nullify(&mut self) {
        self.handle = 0 as HWND;
    }
    #[inline]
    pub fn redraw_menu_bar(&mut self) -> Result<()> {
        error_from_win32_bool!(DrawMenuBar(self.handle))
    }
    #[inline]
    pub fn show(&mut self, stype: ShowWindowType) -> bool {
        unsafe { ShowWindow(self.handle, stype.into()) != 0 }
    }
    #[inline]
    pub fn set_menu(&mut self, menu: Option<MenuBar>) -> Result<()> {
        error_from_win32_bool!(SetMenu(
            self.handle,
            menu.map(|menu: MenuBar| menu.handle())
                .unwrap_or(0 as *mut c_void),
        ))
    }
    #[inline]
    pub fn with_caption_menu<F, T>(&mut self, f: F) -> Result<T>
    where
        F: FnOnce(&mut Menu) -> T,
    {
        unsafe {
            let mut menu = GetSystemMenu(self.handle, 0);
            if menu as usize == 0 {
                return Err(ERROR_NOT_FOUND_MENU);
            };
            Ok(f(Menu::from_mut_ref(&mut menu)))
        }
    }
    #[inline]
    pub fn reset_caption_menu(&mut self) {
        unsafe {
            let _ = GetSystemMenu(self.handle, 1);
        }
    }
    #[inline]
    pub fn with_class<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&mut WindowClass) -> T,
    {
        // https://learn.microsoft.com/zh-cn/windows/win32/api/winuser/ns-winuser-wndclassexw
        // 文档：lpszClassName 的最大长度为 256。 如果 lpszClassName 大于最大长度，则 RegisterClassEx 函数将失败。
        let mut buffer = [0u16; 256];
        let len =
            error_from_win32!(GetClassNameW(self.handle, buffer.as_mut_ptr(), 256) as *mut c_void)?
                as usize;
        let mut vec = buffer[..len].to_vec();
        let _ = buffer;
        vec.push(0);
        Ok(f(&mut WindowClass::from_raw(vec.as_ptr() as PCWSTR)))
    }
    #[inline]
    pub fn get_context_help_id(&self) -> Option<HelpId> {
        HelpId::try_from(unsafe { GetWindowContextHelpId(self.handle) } as i32)
    }
    #[inline]
    pub fn set_context_help_id(&mut self, help_id: Option<HelpId>) -> Result<()> {
        let help = match help_id {
            None => 0,
            Some(x) => x.get(),
        } as u32;
        error_from_win32_bool!(SetWindowContextHelpId(self.handle, help))
    }
    #[inline]
    pub fn destroy(&mut self) -> Result<()> {
        error_from_win32_bool!(DestroyWindow(self.handle))?;
        self.handle = 0 as HWND;
        Ok(())
    }
    #[inline]
    pub fn with_screen_point<T, F: FnOnce(&mut Window) -> T>(point: Point, f: F) -> Option<T> {
        unsafe {
            let mut hwnd = WindowFromPoint(point.to_win32_point());
            if hwnd as usize == 0 {
                None
            } else {
                Some(f(Window::from_mut_ref(&mut hwnd)))
            }
        }
    }
    ///移除id为0的默认项会返回ERROR_NOT_SUPPORTED
    ///不支持能操作其它线程创建的窗口的接收器，会返回Err
    #[inline]
    pub fn remove_msg_receiver<
        const PORC_ID: usize,
        C: RawMessageHandler<SubPorc<PORC_ID>> + Sync + 'static,
    >(
        &mut self,
        _msg_receiver: PhantomData<C>,
    ) -> Result<()> {
        error_from_win32_bool!(RemoveWindowSubclass(
            self.handle,
            Some(subclass_porc::<PORC_ID, C>),
            PORC_ID,
        ))?;
        Ok(())
    }
    #[inline]
    pub fn add_msg_receiver<
        const PORC_ID: usize,
        C: RawMessageHandler<SubPorc<PORC_ID>> + Sync + 'static,
    >(
        &mut self,
        _msg_receiver: PhantomData<C>,
    ) -> Result<()> {
        error_from_win32_bool!(SetWindowSubclass(
            self.handle,
            Some(subclass_porc::<PORC_ID, C>),
            PORC_ID,
            0 as usize,
        ))
    }
    #[inline]
    pub fn has_receiver_for<
        const PORC_ID: usize,
        C: RawMessageHandler<SubPorc<PORC_ID>> + Sync + 'static,
    >(
        &mut self,
        _msg_receiver: PhantomData<C>,
    ) -> bool {
        unsafe {
            GetWindowSubclass(
                self.handle,
                Some(subclass_porc::<PORC_ID, C>),
                PORC_ID,
                0 as *mut usize,
            ) != 0
        }
    }
    #[inline]
    pub fn force_redraw(&mut self) -> Result<()> {
        error_from_win32_bool!(SetWindowPos(
            self.handle,
            0 as HWND,
            0,
            0,
            0,
            0,
            SWP_NOSIZE | SWP_NOMOVE | SWP_NOZORDER,
        ))
    }
    #[inline]
    pub fn have_any_popup_window() -> bool {
        unsafe { AnyPopup() != 0 }
    }
    //未实现区--------------------------------------
    #[inline]
    pub fn set_z_group(&mut self, _pos: WindowZpos, _group: WindowZposGroup) -> Result<()> {
        todo!() //SetWindowBand windows未公开api
    }
    #[inline]
    pub fn get_z_group(&self) -> Result<WindowZposGroup> {
        todo!() //GetWindowBand windows未公开api
    }
    #[inline]
    pub fn adjust_window_rect(_rect: Rect, _wtype: WindowType, _have_menu: bool) -> Result<Rect> {
        todo!() //AdjustWindowRectEx
    }
    #[inline]
    pub fn arrange_iconic(&mut self) -> Result<u32> {
        todo!() //ArrangeIconicWindows
    }
    #[inline]
    pub fn to_top(&mut self) -> Result<()> {
        todo!() //BringWindowToTop
    }
    #[inline]
    pub fn minimize(&mut self) -> Result<()> {
        todo!() //CloseWindow
    }
    #[inline]
    pub fn set_animate(
        &mut self,
        _time: std::time::Duration,
        _atype: WindowAnimateShowType,
    ) -> Result<()> {
        todo!() //AnimateWindow
    }
    #[inline]
    pub fn cascade_child(
        &mut self,
        _skip_mdi_disabled: bool,
        _area: Option<Rect>,
        _wnd: Option<&[Window]>,
    ) -> Result<u16> {
        todo!() //CascadeWindows
    }
    #[inline]
    pub fn with_child_from_point<T, F: FnOnce(&mut Window) -> T>(
        &mut self,
        _pos: Point,
        _f: F,
        _skip_disabled: bool,
        _skip_visible: bool,
        _skip_transparent: bool,
    ) -> Option<T> {
        todo!() //ChildWindowFromPointEx
    }
    #[inline]
    pub fn force_end(&self) {
        todo!()
        // unsafe {EndTask(self.handle, false, true)};
    }
    #[inline]
    pub fn find_window(&mut self, _class: Option<WindowClass>, _title: Option<&str>) {
        todo!()
    }
    #[inline]
    pub fn cascade_window(
        _skip_mdi_disabled: bool,
        _area: Option<Rect>,
        _wnd: Option<&[&Window]>,
    ) -> Result<u16> {
        todo!() //CascadeWindows
    }
    #[inline]
    pub fn allow_set_foreground_window(_pid: Option<u32>) -> Result<()> {
        todo!() //AllowSetForegroundWindow
    }
    #[inline]
    pub fn set_foreground(&self) -> Result<()> {
        todo!() //SetForegroundWindow
    }
}
// use std::fmt;
// pub struct WindowRawMsgFuture {
//     join_handle: tokio::task::JoinHandle<Result<isize>>,
// }
// impl fmt::Debug for WindowRawMsgFuture {
//     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
//         self.join_handle.fmt(fmt)
//     }
// }
// use std::pin::Pin;
// use std::task::Context;
// use std::task::Poll;
// impl Future for WindowRawMsgFuture {
//     type Output = Result<isize>;
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         match self.join_handle.poll(self, cx) {
//             Poll::Pending => Poll::Pending,
//             Poll::Ready(Err(_)) => Poll::Ready(Err(win_error!(ERROR_INTERNAL_ERROR))),
//             Poll::Ready(Ok(x)) => Poll::Ready(x),
//         }
//     }
// }
// impl std::panic::RefUnwindSafe for WindowRawMsgFuture {}
// impl std::panic::UnwindSafe for WindowRawMsgFuture {}

impl Window {
    pub unsafe fn add_msg_to_queue<C: UnsafeMessage + 'static>(&self, msg: C) -> Result<()> {
        unsafe {
            let ptr = msg.into_raw_msg()?;
            let RawMessage(code, wparam, lparam) = ptr.as_msg();
            error_from_win32_bool!(PostMessageW(self.handle, code, wparam, lparam))
        }
    }
    // pub async unsafe fn send_msg_unsafe_async<C: UnsafeMessage + 'static>(&self, msg: C) -> Result<isize> {
    //     use tokio::task;
    //     let hwnd = self.handle() as usize;
    //     let ptr = msg.into_raw_msg()?;
    //
    //     /* WindowRawMsgFuture {*/
    //         let data: StdResult<Result<isize>, tokio::task::JoinError> = /*join_handle: */task::spawn_blocking(move || unsafe {
    //             let RawMessage(code, wparam, lparam) = ptr.ptr;
    //             last_error!(
    //                 SendMessageW(
    //                     hwnd as HWND,
    //                     code,
    //                     wparam,
    //                     lparam
    //                 )
    //             )
    //         })/*,*/.await;
    //     // }
    //     match data {
    //         Err(_) => Err(win_error!(ERROR_INTERNAL_ERROR)),
    //         Ok(x) => x,
    //     }
    // }
    ///当向自己线程发消息时，自己调用处理函数，否则添加到队列并等待
    pub unsafe fn send_msg_unsafe<C: UnsafeMessage>(&self, msg: C) -> Result<isize> {
        unsafe {
            let ptr = msg.into_raw_msg()?;
            let RawMessage(code, wparam, lparam) = ptr.as_msg();
            error_from_win32_zero_num!(SendMessageW(self.handle, code, wparam, lparam))
        }
    }
}
impl Window {
    ///向自己的父窗口发送控件消息（在编写控件时使用，阻塞）
    pub fn send_control_msg<M: UnsafeControlMsg>(&self, msg: M) -> Result<isize> {
        unsafe {
            self.parent()
                .ok_or(ERROR_WINDOW_TYPE_NOT_SUPPORT)?
                .send_msg_unsafe(msg)
        }
    }
    // ///向自己的父窗口发送控件消息（异步版）
    // pub async fn send_control_msg_async<M: UnsafeControlMsg>(&self, msg: M) -> Result<isize> {
    //     unsafe {
    //         self.parent()
    //             .ok_or(ERROR_WINDOW_TYPE_NOT_SUPPORT)?
    //             .send_msg_unsafe_async(msg)
    //             .await
    //     }
    // }
    ///向自己的父窗口发送控件消息（不获取返回值）
    pub fn send_control_nofiy<M: UnsafeControlMsg + 'static>(&self, msg: M) -> Result<()> {
        unsafe {
            self.parent()
                .ok_or(ERROR_WINDOW_TYPE_NOT_SUPPORT)?
                .add_msg_to_queue(msg)
        }
    }
}
