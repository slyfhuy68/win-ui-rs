//本文件几乎都是AI生成的
use super::*;
#[derive(Clone, PartialEq, Copy, Default, Debug)]
pub enum ClassBackgroundBrush {
    Brush(super::brush::Brush),
    ActiveBorder,
    ActiveCaption,
    AppWorkspace,
    Background,
    BtnFace,
    BtnShadow,
    BtnText,
    CaptionText,
    GrayText,
    Highlight,
    HighlightText,
    InactiveBorder,
    InactiveCaption,
    Menu,
    MenuText,
    Scrollbar,
    #[default]
    Window,
    WindowFrame,
    WindowText,
}
impl From<HBRUSH> for ClassBackgroundBrush {
    fn from(ush: HBRUSH) -> Self {
        match HBRUSH((ush.0 as usize - 1) as *mut c_void) {
            HBRUSH(val) if val == COLOR_ACTIVEBORDER.0 as *mut c_void => {
                ClassBackgroundBrush::ActiveBorder
            }
            HBRUSH(val) if val == COLOR_ACTIVECAPTION.0 as *mut c_void => {
                ClassBackgroundBrush::ActiveCaption
            }
            HBRUSH(val) if val == COLOR_APPWORKSPACE.0 as *mut c_void => {
                ClassBackgroundBrush::AppWorkspace
            }
            HBRUSH(val) if val == COLOR_BACKGROUND.0 as *mut c_void => {
                ClassBackgroundBrush::Background
            }
            HBRUSH(val) if val == COLOR_BTNFACE.0 as *mut c_void => ClassBackgroundBrush::BtnFace,
            HBRUSH(val) if val == COLOR_BTNSHADOW.0 as *mut c_void => {
                ClassBackgroundBrush::BtnShadow
            }
            HBRUSH(val) if val == COLOR_BTNTEXT.0 as *mut c_void => ClassBackgroundBrush::BtnText,
            HBRUSH(val) if val == COLOR_CAPTIONTEXT.0 as *mut c_void => {
                ClassBackgroundBrush::CaptionText
            }
            HBRUSH(val) if val == COLOR_GRAYTEXT.0 as *mut c_void => ClassBackgroundBrush::GrayText,
            HBRUSH(val) if val == COLOR_HIGHLIGHT.0 as *mut c_void => {
                ClassBackgroundBrush::Highlight
            }
            HBRUSH(val) if val == COLOR_HIGHLIGHTTEXT.0 as *mut c_void => {
                ClassBackgroundBrush::HighlightText
            }
            HBRUSH(val) if val == COLOR_INACTIVEBORDER.0 as *mut c_void => {
                ClassBackgroundBrush::InactiveBorder
            }
            HBRUSH(val) if val == COLOR_INACTIVECAPTION.0 as *mut c_void => {
                ClassBackgroundBrush::InactiveCaption
            }
            HBRUSH(val) if val == COLOR_MENU.0 as *mut c_void => ClassBackgroundBrush::Menu,
            HBRUSH(val) if val == COLOR_MENUTEXT.0 as *mut c_void => ClassBackgroundBrush::MenuText,
            HBRUSH(val) if val == COLOR_SCROLLBAR.0 as *mut c_void => {
                ClassBackgroundBrush::Scrollbar
            }
            HBRUSH(val) if val == COLOR_WINDOW.0 as *mut c_void => ClassBackgroundBrush::Window,
            HBRUSH(val) if val == COLOR_WINDOWFRAME.0 as *mut c_void => {
                ClassBackgroundBrush::WindowFrame
            }
            HBRUSH(val) if val == COLOR_WINDOWTEXT.0 as *mut c_void => {
                ClassBackgroundBrush::WindowText
            }
            HBRUSH(x) => {
                ClassBackgroundBrush::Brush(HBRUSH((x as usize + 1) as *mut c_void).into())
            }
        }
    }
}
impl Into<HBRUSH> for ClassBackgroundBrush {
    fn into(self) -> HBRUSH {
        use ClassBackgroundBrush::*;
        let result = match self {
            Brush(ush) => {
                return ush.into();
            }
            ActiveBorder => COLOR_ACTIVEBORDER,
            ActiveCaption => COLOR_ACTIVECAPTION,
            AppWorkspace => COLOR_APPWORKSPACE,
            Background => COLOR_BACKGROUND,
            BtnFace => COLOR_BTNFACE,
            BtnShadow => COLOR_BTNSHADOW,
            BtnText => COLOR_BTNTEXT,
            CaptionText => COLOR_CAPTIONTEXT,
            GrayText => COLOR_GRAYTEXT,
            Highlight => COLOR_HIGHLIGHT,
            HighlightText => COLOR_HIGHLIGHTTEXT,
            InactiveBorder => COLOR_INACTIVEBORDER,
            InactiveCaption => COLOR_INACTIVECAPTION,
            Menu => COLOR_MENU,
            MenuText => COLOR_MENUTEXT,
            Scrollbar => COLOR_SCROLLBAR,
            Window => COLOR_WINDOW,
            WindowFrame => COLOR_WINDOWFRAME,
            WindowText => COLOR_WINDOWTEXT,
        };
        HBRUSH((result.0 as usize + 1) as *mut c_void)
    }
}

#[derive(Clone, PartialEq, Copy, Default, Debug)]
#[repr(packed)]
pub struct WindowClassStyle {
    pub globa: bool,             //CS_GLOBALCLASS
    pub no_close_button: bool,   //CS_NOCLOSE
    pub ver_draw: bool,          //CS_VREDRAW
    pub her_draw: bool,          //CS_HREDRAW
    pub dbl_clk_msg: bool,       //CS_DBLCLKS
    pub parent_clipping: bool,   //CS_PARENTDC
    pub save_bits: bool,         //CS_SAVEBITS
    pub byte_ailgn_client: bool, //CS_BYTEALIGNCLIENT
    pub byte_ailgn_window: bool, //CS_BYTEALIGNWINDOW
    pub drop_shadrow: bool,      //CS_DROPSHADOW
    pub dc_type: DCtype,
}
impl From<WNDCLASS_STYLES> for WindowClassStyle {
    fn from(ms_style: WNDCLASS_STYLES) -> Self {
        Self {
            globa: ms_style.contains(CS_GLOBALCLASS),
            no_close_button: ms_style.contains(CS_NOCLOSE),
            ver_draw: ms_style.contains(CS_VREDRAW),
            her_draw: ms_style.contains(CS_HREDRAW),
            dbl_clk_msg: ms_style.contains(CS_DBLCLKS),
            parent_clipping: ms_style.contains(CS_PARENTDC),
            save_bits: ms_style.contains(CS_SAVEBITS),
            byte_ailgn_client: ms_style.contains(CS_BYTEALIGNCLIENT),
            byte_ailgn_window: ms_style.contains(CS_BYTEALIGNWINDOW),
            drop_shadrow: ms_style.contains(CS_DROPSHADOW),
            dc_type: ms_style.into(),
        }
    }
}
impl Into<WNDCLASS_STYLES> for WindowClassStyle {
    fn into(self) -> WNDCLASS_STYLES {
        let mut ms_style = WNDCLASS_STYLES(0u32);
        if self.globa {
            ms_style |= CS_GLOBALCLASS;
        };
        if self.no_close_button {
            ms_style |= CS_NOCLOSE;
        };
        if self.ver_draw {
            ms_style |= CS_VREDRAW;
        };
        if self.her_draw {
            ms_style |= CS_HREDRAW;
        };
        if self.dbl_clk_msg {
            ms_style |= CS_DBLCLKS;
        };
        if self.parent_clipping {
            ms_style |= CS_PARENTDC;
        };
        if self.save_bits {
            ms_style |= CS_SAVEBITS;
        };
        if self.byte_ailgn_client {
            ms_style |= CS_BYTEALIGNCLIENT;
        };
        if self.byte_ailgn_window {
            ms_style |= CS_BYTEALIGNWINDOW;
        };
        if self.drop_shadrow {
            ms_style |= CS_DROPSHADOW;
        };
        ms_style | self.dc_type.into()
    }
}
#[derive(Clone, PartialEq, Copy, Default, Debug)]
pub enum DCtype {
    #[default]
    DefaultDC, //NULL
    WindowDC, //CS_OWNDC
    ClassDC,  //CS_CLASSDC
}
impl Into<WNDCLASS_STYLES> for DCtype {
    fn into(self) -> WNDCLASS_STYLES {
        use DCtype::*;
        match self {
            WindowDC => CS_OWNDC,
            ClassDC => CS_CLASSDC,
            DefaultDC => WNDCLASS_STYLES(0u32),
        }
    }
}
impl From<WNDCLASS_STYLES> for DCtype {
    fn from(ms_style: WNDCLASS_STYLES) -> Self {
        use DCtype::*;
        if ms_style.contains(CS_OWNDC) {
            WindowDC
        } else if ms_style.contains(CS_CLASSDC) {
            ClassDC
        } else {
            DefaultDC
        }
    }
}
//-------------------------------------------------------------------------------
//const WS_ONLYCAPTION: WINDOW_STYLE = WINDOW_STYLE(4194304u32);
#[derive(Clone, PartialEq, Copy, Default, Debug)]
pub enum WindowSizeState {
    #[default]
    None, //NULL
    Min, //WS_MINIMIZE
    Max, //WS_MAXIMIZE
}
impl From<WINDOW_STYLE> for WindowSizeState {
    fn from(ms_style: WINDOW_STYLE) -> Self {
        if ms_style.contains(WS_MAXIMIZE) {
            Self::Max
        } else if ms_style.contains(WS_MINIMIZE) {
            Self::Min
        } else {
            Self::None
        }
    }
}
impl Into<WINDOW_STYLE> for WindowSizeState {
    fn into(self) -> WINDOW_STYLE {
        match self {
            Self::Max => WS_MAXIMIZE,
            Self::Min => WS_MINIMIZE,
            Self::None => WINDOW_STYLE(0u32),
        }
    }
}
#[derive(Clone, PartialEq, Copy, Default, Debug)]
pub enum WindowContextBarButton {
    NoButton, //NULL
    Minimize, //WS_MINIMIZEBOX
    Maximize, //WS_MAXIMIZEBOX
    #[default]
    MinimizeAndMaximize, //WS_MINIMIZEBOX | WS_MAXIMIZEBOX
    Help,     //WS_EX_CONTEXTHELP
}
impl From<(WINDOW_STYLE, WINDOW_EX_STYLE)> for WindowContextBarButton {
    fn from(rstyle: (WINDOW_STYLE, WINDOW_EX_STYLE)) -> Self {
        use WindowContextBarButton::*;
        let (ms_style, ms_style_ex) = rstyle;
        if ms_style_ex.contains(WS_EX_CONTEXTHELP) {
            Help
        } else if ms_style.contains(WS_MINIMIZEBOX | WS_MAXIMIZEBOX) {
            MinimizeAndMaximize
        } else if ms_style.contains(WS_MAXIMIZEBOX) {
            Maximize
        } else if ms_style.contains(WS_MINIMIZEBOX) {
            Minimize
        } else {
            NoButton
        }
    }
}
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE)> for WindowContextBarButton {
    fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE) {
        use WindowContextBarButton::*;
        match self {
            NoButton => (WINDOW_STYLE(0u32), WINDOW_EX_STYLE(0u32)),
            Minimize => (WS_MINIMIZEBOX, WINDOW_EX_STYLE(0u32)),
            Maximize => (WS_MAXIMIZEBOX, WINDOW_EX_STYLE(0u32)),
            MinimizeAndMaximize => (WS_MINIMIZEBOX | WS_MAXIMIZEBOX, WINDOW_EX_STYLE(0u32)),
            Help => (WINDOW_STYLE(0u32), WS_EX_CONTEXTHELP),
        }
    }
}
#[derive(Clone, PartialEq, Copy, Debug)]
pub enum WindowBorderType {
    NoBorder,                           //NULL
    DlgFame,                            //WS_DLGFRAME
    ThinLineDlgFame,                    //WS_DLGFRAME | WS_BORDER
    ThinLine,                           //WS_BORDER
    Caption,                            //WS_CAPTION
    SystemMenu(WindowContextBarButton), //WS_SYSMENU | WS_CAPTION
}
impl Default for WindowBorderType {
    fn default() -> Self {
        Self::SystemMenu(Default::default())
    }
}
impl From<(WINDOW_STYLE, WINDOW_EX_STYLE)> for WindowBorderType {
    fn from(rstyle: (WINDOW_STYLE, WINDOW_EX_STYLE)) -> Self {
        let (ms_style, ms_style_ex) = rstyle;
        use WindowBorderType::*;
        if ms_style.contains(WS_SYSMENU) {
            SystemMenu((ms_style, ms_style_ex).into())
        } else if ms_style.contains(WS_CAPTION) {
            Caption
        } else if ms_style.contains(WS_DLGFRAME | WS_BORDER) {
            ThinLineDlgFame
        } else if ms_style.contains(WS_BORDER) {
            ThinLine
        } else if ms_style.contains(WS_DLGFRAME) {
            DlgFame
        } else {
            NoBorder
        }
    }
}
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE)> for WindowBorderType {
    fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE) {
        use WindowBorderType::*;
        (
            match self {
                NoBorder => WINDOW_STYLE(0u32),
                WindowBorderType::DlgFame => WS_DLGFRAME,
                WindowBorderType::ThinLineDlgFame => WS_DLGFRAME | WS_BORDER,
                ThinLine => WS_BORDER,
                Caption => WS_CAPTION,
                SystemMenu(x) => {
                    let (z, y) = x.into();
                    return (WS_SYSMENU | WS_CAPTION | z, y);
                }
            },
            WINDOW_EX_STYLE(0u32),
        )
    }
}
#[derive(Clone, PartialEq, Copy, Default, Debug)]
pub enum WindowEdgeType {
    #[default]
    None,
    Raised, //WS_EX_WINDOWEDGE
    Sunken, //WS_EX_CLIENTEDGE
    ThreeD, //WS_EX_STATICEDGE
}
impl Into<WINDOW_EX_STYLE> for WindowEdgeType {
    fn into(self) -> WINDOW_EX_STYLE {
        use WindowEdgeType::*;
        match self {
            None => WINDOW_EX_STYLE(0u32),
            Raised => WS_EX_WINDOWEDGE,
            Sunken => WS_EX_CLIENTEDGE,
            ThreeD => WS_EX_STATICEDGE,
        }
    }
}
impl From<WINDOW_EX_STYLE> for WindowEdgeType {
    fn from(style: WINDOW_EX_STYLE) -> Self {
        use WindowEdgeType::*;
        if style.contains(WS_EX_WINDOWEDGE) {
            Raised
        } else if style.contains(WS_EX_CLIENTEDGE) {
            Sunken
        } else if style.contains(WS_EX_STATICEDGE) {
            ThreeD
        } else {
            None
        }
    }
}
#[derive(Clone, PartialEq, Copy, Default, Debug)]
#[repr(packed)]
pub struct NormalWindowStyles {
    //此部分使用正则表达式生成
    pub size_box: bool,              // WS_SIZEBOX
    pub horizontal_roll: bool,       // WS_HSCROLL
    pub vertical_roll: bool,         // WS_VSCROLL
    pub clip_children: bool,         // WS_CLIPCHILDREN
    pub disabled: bool,              // WS_DISABLED
    pub invisible: bool,             // !WS_VISIBLE
    pub dlg_modal_frame: bool,       // WS_EX_DLGMODALFRAME
    pub top_most: bool,              // WS_EX_TOPMOST
    pub accept_files: bool,          // WS_EX_ACCEPTFILES
    pub transparent: bool,           // WS_EX_TRANSPARENT
    pub tool_window: bool,           // WS_EX_TOOLWINDOW
    pub right_aligned: bool,         // WS_EX_RIGHT
    pub right_to_left_reading: bool, // WS_EX_RTLREADING
    pub left_scrroll_bar: bool,      // WS_EX_LEFTSCROLLBAR
    pub control_parent: bool,        // WS_EX_CONTROLPARENT
    pub app_window: bool,            // WS_EX_APPWINDOW
    pub no_inherit_layout: bool,     // WS_EX_NOINHERITLAYOUT
    pub right_layout: bool,          // WS_EX_LAYOUTRTL
    pub com_posited: bool,           // WS_EX_COMPOSITED
    pub no_auto_active: bool,        // WS_EX_NOACTIVATE
    pub no_redirection_bitmap: bool, // WS_EX_NOREDIRECTIONBITMAP
    pub edge_type: WindowEdgeType,
    pub size_state: WindowSizeState,
    pub border_type: WindowBorderType,
}
#[derive(Clone, PartialEq, Copy, Debug)]
#[repr(packed)]
pub struct ChildWindowStyles {
    pub style: NormalWindowStyles,
    pub tab_stop: bool,      //WS_TABSTOP
    pub group_leader: bool,  //WS_GROUP
    pub clip_isblings: bool, //WS_CLIPSIBLINGS
    pub no_parent_notify: bool, //WS_EX_NOPARENTNOTIFY
                             // pub mid_child: bool, //WS_EX_MDICHILD
}
impl Default for ChildWindowStyles {
    fn default() -> Self {
        Self {
            style: NormalWindowStyles {
                border_type: WindowBorderType::NoBorder,
                ..Default::default()
            },
            tab_stop: true,
            group_leader: false,
            clip_isblings: false,
            no_parent_notify: false,
        }
    }
}
impl From<(WINDOW_STYLE, WINDOW_EX_STYLE)> for NormalWindowStyles {
    fn from((style, style_ex): (WINDOW_STYLE, WINDOW_EX_STYLE)) -> Self {
        Self {
            size_box: style.contains(WS_SIZEBOX),
            horizontal_roll: style.contains(WS_HSCROLL),
            vertical_roll: style.contains(WS_VSCROLL),
            clip_children: style.contains(WS_CLIPCHILDREN),
            disabled: style.contains(WS_DISABLED),
            invisible: !style.contains(WS_VISIBLE),
            dlg_modal_frame: style_ex.contains(WS_EX_DLGMODALFRAME),
            top_most: style_ex.contains(WS_EX_TOPMOST),
            accept_files: style_ex.contains(WS_EX_ACCEPTFILES),
            transparent: style_ex.contains(WS_EX_TRANSPARENT),
            tool_window: style_ex.contains(WS_EX_TOOLWINDOW),
            right_aligned: style_ex.contains(WS_EX_RIGHT),
            right_to_left_reading: style_ex.contains(WS_EX_RTLREADING),
            left_scrroll_bar: style_ex.contains(WS_EX_LEFTSCROLLBAR),
            control_parent: style_ex.contains(WS_EX_CONTROLPARENT),
            app_window: style_ex.contains(WS_EX_APPWINDOW),
            no_inherit_layout: style_ex.contains(WS_EX_NOINHERITLAYOUT),
            right_layout: style_ex.contains(WS_EX_LAYOUTRTL),
            com_posited: style_ex.contains(WS_EX_COMPOSITED),
            no_auto_active: style_ex.contains(WS_EX_NOACTIVATE),
            no_redirection_bitmap: style_ex.contains(WS_EX_NOREDIRECTIONBITMAP),
            edge_type: WindowEdgeType::from(style_ex),
            size_state: WindowSizeState::from(style),
            border_type: WindowBorderType::from((style, style_ex)),
        }
    }
}
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE)> for NormalWindowStyles {
    fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE) {
        let (mut style, mut style_ex) = self.border_type.into();
        set_style(&mut style, WS_SIZEBOX, self.size_box);
        set_style(&mut style, WS_HSCROLL, self.horizontal_roll);
        set_style(&mut style, WS_VSCROLL, self.vertical_roll);
        set_style(&mut style, WS_CLIPCHILDREN, self.clip_children);
        set_style(&mut style, WS_DISABLED, self.disabled);
        set_style(&mut style, WS_VISIBLE, !self.invisible);
        set_style_ex(&mut style_ex, WS_EX_DLGMODALFRAME, self.dlg_modal_frame);
        set_style_ex(&mut style_ex, WS_EX_TOPMOST, self.top_most);
        set_style_ex(&mut style_ex, WS_EX_ACCEPTFILES, self.accept_files);
        set_style_ex(&mut style_ex, WS_EX_TRANSPARENT, self.transparent);
        set_style_ex(&mut style_ex, WS_EX_TOOLWINDOW, self.tool_window);
        set_style_ex(&mut style_ex, WS_EX_RIGHT, self.right_aligned);
        set_style_ex(&mut style_ex, WS_EX_RTLREADING, self.right_to_left_reading);
        set_style_ex(&mut style_ex, WS_EX_LEFTSCROLLBAR, self.left_scrroll_bar);
        set_style_ex(&mut style_ex, WS_EX_CONTROLPARENT, self.control_parent);
        set_style_ex(&mut style_ex, WS_EX_APPWINDOW, self.app_window);
        set_style_ex(&mut style_ex, WS_EX_NOINHERITLAYOUT, self.no_inherit_layout);
        set_style_ex(&mut style_ex, WS_EX_LAYOUTRTL, self.right_layout);
        set_style_ex(&mut style_ex, WS_EX_COMPOSITED, self.com_posited);
        set_style_ex(&mut style_ex, WS_EX_NOACTIVATE, self.no_auto_active);
        set_style_ex(
            &mut style_ex,
            WS_EX_NOREDIRECTIONBITMAP,
            self.no_redirection_bitmap,
        );
        style |= self.size_state.into();
        style_ex |= self.edge_type.into();
        (style, style_ex)
    }
}
#[inline]
fn set_style(style: &mut WINDOW_STYLE, flag: WINDOW_STYLE, condition: bool) {
    style.0 |= flag.0 * condition as u32;
}
#[inline]
fn set_style_ex(style_ex: &mut WINDOW_EX_STYLE, flag: WINDOW_EX_STYLE, condition: bool) {
    style_ex.0 |= flag.0 * condition as u32;
}
impl From<(WINDOW_STYLE, WINDOW_EX_STYLE)> for ChildWindowStyles {
    fn from((style, style_ex): (WINDOW_STYLE, WINDOW_EX_STYLE)) -> Self {
        Self {
            style: (style, style_ex).into(),
            tab_stop: style.contains(WS_TABSTOP),
            group_leader: style.contains(WS_GROUP),
            clip_isblings: style.contains(WS_CLIPSIBLINGS),
            no_parent_notify: style_ex.contains(WS_EX_NOPARENTNOTIFY),
            // mid_child: style_ex.contains(WS_EX_MDICHILD),
        }
    }
}
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE)> for ChildWindowStyles {
    fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE) {
        let (mut style, mut style_ex) = self.style.into();

        if self.tab_stop {
            style |= WS_TABSTOP;
        };
        if self.group_leader {
            style |= WS_GROUP;
        };
        if self.clip_isblings {
            style |= WS_CLIPSIBLINGS;
        };
        if self.no_parent_notify {
            style_ex |= WS_EX_NOPARENTNOTIFY;
        };
        // if self.mid_child {
        //     style_ex |= WS_EX_MDICHILD;
        // };
        (style, style_ex)
    }
}
#[derive(Debug)]
pub enum WindowType {
    Overlapped {
        style: NormalWindowStyles,
        menu: Option<MenuBar>,
        owner: Option<Window>,
        is_layered: bool, //WS_EX_LAYERED
    }, //重叠窗口
    Popup {
        style: NormalWindowStyles,
        menu: Option<MenuBar>,
        owner: Option<Window>,
        is_layered: bool, //WS_EX_LAYERED
    },
    Child {
        style: ChildWindowStyles,
        identifier: WindowID,
        parent: Window,
        is_layered: bool, //WS_EX_LAYERED
    },
    MessageOnly,
}
impl WindowType {
    pub fn nullify_menu(&mut self) {
        let _ = match self {
            WindowType::Overlapped { menu, .. } => menu.take().map(|mut x| x.nullify()),
            WindowType::Popup { menu, .. } => menu.take().map(|mut x| x.nullify()),
            _ => Some(()),
        };
    }
}
impl Default for WindowType {
    fn default() -> Self {
        Self::Overlapped {
            style: Default::default(),
            menu: None,
            owner: None,
            is_layered: false,
        }
    }
}
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE, HMENU, HWND)> for WindowType {
    fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE, HMENU, HWND) {
        use WindowType::*;
        unsafe {
            match self {
                Overlapped {
                    style,
                    menu,
                    owner,
                    is_layered,
                } => {
                    let (style, mut style_ex) = style.into();
                    if is_layered {
                        style_ex |= WS_EX_LAYERED;
                    };
                    (
                        style,
                        style_ex,
                        menu.unwrap_or(MenuBar::null()).handle(),
                        owner.unwrap_or_default().handle(),
                    )
                }
                Popup {
                    style,
                    menu,
                    owner,
                    is_layered,
                } => {
                    let (style, mut style_ex) = style.into();
                    if is_layered {
                        style_ex |= WS_EX_LAYERED;
                    };
                    (
                        style | WS_POPUP,
                        style_ex,
                        menu.unwrap_or(MenuBar::null()).handle(),
                        owner.unwrap_or_default().handle(),
                    )
                }
                Child {
                    style,
                    identifier,
                    parent,
                    is_layered,
                } => {
                    let (style, mut style_ex) = style.into();
                    if is_layered {
                        style_ex |= WS_EX_LAYERED;
                    };
                    (
                        style | WS_CHILD,
                        style_ex,
                        HMENU(identifier as *mut c_void),
                        parent.handle(),
                    )
                }
                MessageOnly => (
                    WINDOW_STYLE(0),
                    WINDOW_EX_STYLE(0),
                    HMENU(0 as *mut c_void),
                    HWND_MESSAGE,
                ),
            }
        }
    }
}
impl WindowType {
    ///确保wnd是Rust拥有的
    pub unsafe fn from_data(
        style: WINDOW_STYLE,
        style_ex: WINDOW_EX_STYLE,
        menu: HMENU,
        wnd: HWND,
    ) -> Self {
        use WindowType::*;
        if wnd == HWND_MESSAGE {
            return MessageOnly;
        }
        let w: Option<Window> = if wnd.is_invalid() {
            None
        } else {
            unsafe { Some(Window::from_handle(wnd)) }
        };
        let m: Option<MenuBar> = if wnd.is_invalid() {
            None
        } else {
            unsafe { Some(MenuBar::from_handle(menu)) }
        };
        if style.contains(WS_CHILD) && !w.is_some() {
            return Child {
                style: (style, style_ex).into(),
                identifier: menu.0 as u16,
                parent: w.unwrap_or_default().into(),
                is_layered: style_ex.contains(WS_EX_LAYERED),
            };
        };
        if style.contains(WS_POPUP) {
            return Popup {
                style: (style, style_ex).into(),
                menu: m,
                owner: w,
                is_layered: style_ex.contains(WS_EX_LAYERED),
            };
        } else {
            return Overlapped {
                style: (style, style_ex).into(),
                menu: m,
                owner: w,
                is_layered: style_ex.contains(WS_EX_LAYERED),
            };
        };
    }
}
