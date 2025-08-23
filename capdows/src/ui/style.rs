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
        use ClassBackgroundBrush::*;
        match (ush as i32) - 1 {
            COLOR_ACTIVEBORDER => ActiveBorder,
            COLOR_ACTIVECAPTION => ActiveCaption,
            COLOR_APPWORKSPACE => AppWorkspace,
            COLOR_BACKGROUND => Background,
            COLOR_BTNFACE => BtnFace,
            COLOR_BTNSHADOW => BtnShadow,
            COLOR_BTNTEXT => BtnText,
            COLOR_CAPTIONTEXT => CaptionText,
            COLOR_GRAYTEXT => GrayText,
            COLOR_HIGHLIGHT => Highlight,
            COLOR_HIGHLIGHTTEXT => HighlightText,
            COLOR_INACTIVEBORDER => InactiveBorder,
            COLOR_INACTIVECAPTION => InactiveCaption,
            COLOR_MENU => Menu,
            COLOR_MENUTEXT => MenuText,
            COLOR_SCROLLBAR => Scrollbar,
            COLOR_WINDOW => Window,
            COLOR_WINDOWFRAME => WindowFrame,
            COLOR_WINDOWTEXT => WindowText,
            _ => Brush(super::brush::Brush { handle: ush }),
        }
    }
}
impl From<ClassBackgroundBrush> for HBRUSH {
    fn from(val: ClassBackgroundBrush) -> Self {
        use ClassBackgroundBrush::*;
        ((match val {
            Brush(ush) => return ush.into(),
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
        } as usize)
            + 1) as HBRUSH
    }
}

#[derive(Clone, PartialEq, Copy, Default, Debug)]
#[repr(C, packed)]
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
            globa: ucontain(ms_style, CS_GLOBALCLASS),
            no_close_button: ucontain(ms_style, CS_NOCLOSE),
            ver_draw: ucontain(ms_style, CS_VREDRAW),
            her_draw: ucontain(ms_style, CS_HREDRAW),
            dbl_clk_msg: ucontain(ms_style, CS_DBLCLKS),
            parent_clipping: ucontain(ms_style, CS_PARENTDC),
            save_bits: ucontain(ms_style, CS_SAVEBITS),
            byte_ailgn_client: ucontain(ms_style, CS_BYTEALIGNCLIENT),
            byte_ailgn_window: ucontain(ms_style, CS_BYTEALIGNWINDOW),
            drop_shadrow: ucontain(ms_style, CS_DROPSHADOW),
            dc_type: ms_style.into(),
        }
    }
}
impl From<WindowClassStyle> for WNDCLASS_STYLES {
    fn from(val: WindowClassStyle) -> Self {
        let mut ms_style = 0u32;
        set_style(&mut ms_style, CS_GLOBALCLASS, val.globa);
        set_style(&mut ms_style, CS_NOCLOSE, val.no_close_button);
        set_style(&mut ms_style, CS_VREDRAW, val.ver_draw);
        set_style(&mut ms_style, CS_HREDRAW, val.her_draw);
        set_style(&mut ms_style, CS_DBLCLKS, val.dbl_clk_msg);
        set_style(&mut ms_style, CS_PARENTDC, val.parent_clipping);
        set_style(&mut ms_style, CS_SAVEBITS, val.save_bits);
        set_style(&mut ms_style, CS_BYTEALIGNCLIENT, val.byte_ailgn_client);
        set_style(&mut ms_style, CS_BYTEALIGNWINDOW, val.byte_ailgn_window);
        set_style(&mut ms_style, CS_DROPSHADOW, val.drop_shadrow);
        ms_style | <DCtype as Into<WNDCLASS_STYLES>>::into(val.dc_type)
    }
}
#[derive(Clone, PartialEq, Copy, Default, Debug)]
pub enum DCtype {
    #[default]
    DefaultDC, //NULL
    WindowDC, //CS_OWNDC
    ClassDC,  //CS_CLASSDC
}
impl From<DCtype> for WNDCLASS_STYLES {
    fn from(val: DCtype) -> Self {
        use DCtype::*;
        match val {
            WindowDC => CS_OWNDC,
            ClassDC => CS_CLASSDC,
            DefaultDC => 0u32,
        }
    }
}
impl From<WNDCLASS_STYLES> for DCtype {
    fn from(ms_style: WNDCLASS_STYLES) -> Self {
        use DCtype::*;
        if ucontain(ms_style, CS_OWNDC) {
            WindowDC
        } else if ucontain(ms_style, CS_CLASSDC) {
            ClassDC
        } else {
            DefaultDC
        }
    }
}
#[derive(Clone, PartialEq, Copy, Default, Debug)]
pub enum WindowSizeState {
    #[default]
    None, //NULL
    Min, //WS_MINIMIZE
    Max, //WS_MAXIMIZE
}
impl From<WINDOW_STYLE> for WindowSizeState {
    fn from(ms_style: WINDOW_STYLE) -> Self {
        if ucontain(ms_style, WS_MAXIMIZE) {
            Self::Max
        } else if ucontain(ms_style, WS_MINIMIZE) {
            Self::Min
        } else {
            Self::None
        }
    }
}
impl From<WindowSizeState> for WINDOW_STYLE {
    fn from(val: WindowSizeState) -> Self {
        match val {
            WindowSizeState::Max => WS_MAXIMIZE,
            WindowSizeState::Min => WS_MINIMIZE,
            WindowSizeState::None => 0u32,
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
        if ucontain(ms_style_ex, WS_EX_CONTEXTHELP) {
            Help
        } else if ucontain(ms_style, WS_MINIMIZEBOX | WS_MAXIMIZEBOX) {
            MinimizeAndMaximize
        } else if ucontain(ms_style, WS_MAXIMIZEBOX) {
            Maximize
        } else if ucontain(ms_style, WS_MINIMIZEBOX) {
            Minimize
        } else {
            NoButton
        }
    }
}
impl From<WindowContextBarButton> for (WINDOW_STYLE, WINDOW_EX_STYLE) {
    fn from(val: WindowContextBarButton) -> Self {
        use WindowContextBarButton::*;
        match val {
            NoButton => (0u32, 0u32),
            Minimize => (WS_MINIMIZEBOX, 0u32),
            Maximize => (WS_MAXIMIZEBOX, 0u32),
            MinimizeAndMaximize => (WS_MINIMIZEBOX | WS_MAXIMIZEBOX, 0u32),
            Help => (0u32, WS_EX_CONTEXTHELP),
        }
    }
}
// const WS_ONLYCAPTION: WINDOW_STYLE = WINDOW_STYLE(4194304u32);
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
        if ucontain(ms_style, WS_SYSMENU) {
            SystemMenu((ms_style, ms_style_ex).into())
        } else if ucontain(ms_style, WS_CAPTION) {
            Caption
        } else if ucontain(ms_style, WS_DLGFRAME | WS_BORDER) {
            ThinLineDlgFame
        } else if ucontain(ms_style, WS_BORDER) {
            ThinLine
        } else if ucontain(ms_style, WS_DLGFRAME) {
            DlgFame
        } else {
            NoBorder
        }
    }
}
impl From<WindowBorderType> for (WINDOW_STYLE, WINDOW_EX_STYLE) {
    fn from(val: WindowBorderType) -> Self {
        use WindowBorderType::*;
        (
            match val {
                NoBorder => 0u32,
                WindowBorderType::DlgFame => WS_DLGFRAME,
                WindowBorderType::ThinLineDlgFame => WS_DLGFRAME | WS_BORDER,
                ThinLine => WS_BORDER,
                Caption => WS_CAPTION,
                SystemMenu(x) => {
                    let (z, y) = x.into();
                    return (WS_SYSMENU | WS_CAPTION | z, y);
                }
            },
            0u32,
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
impl From<WindowEdgeType> for WINDOW_EX_STYLE {
    fn from(val: WindowEdgeType) -> Self {
        use WindowEdgeType::*;
        match val {
            None => 0u32,
            Raised => WS_EX_WINDOWEDGE,
            Sunken => WS_EX_CLIENTEDGE,
            ThreeD => WS_EX_STATICEDGE,
        }
    }
}
impl From<WINDOW_EX_STYLE> for WindowEdgeType {
    fn from(style: WINDOW_EX_STYLE) -> Self {
        use WindowEdgeType::*;
        if ucontain(style, WS_EX_WINDOWEDGE) {
            Raised
        } else if ucontain(style, WS_EX_CLIENTEDGE) {
            Sunken
        } else if ucontain(style, WS_EX_STATICEDGE) {
            ThreeD
        } else {
            None
        }
    }
}
#[derive(Clone, PartialEq, Copy, Default, Debug)]
#[repr(C, packed)]
pub struct NormalWindowStyles {
    pub size_box: bool,              // WS_SIZEBOX
    pub horizontal_roll: bool,       // WS_HSCROLL
    pub vertical_roll: bool,         // WS_VSCROLL
    pub clip_children: bool,         // WS_CLIPCHILDREN
    pub disabled: bool,              // WS_DISABLED
    pub visible: bool,               // WS_VISIBLE
    pub dlg_modal_frame: bool,       // WS_EX_DLGMODALFRAME
    pub top_most: bool,              // WS_EX_TOPMOST
    pub accept_files: bool,          // WS_EX_ACCEPTFILES
    pub transparent: bool,           // WS_EX_TRANSPARENT
    pub tool_window: bool,           // WS_EX_TOOLWINDOW
    pub right_aligned: bool,         // WS_EX_RIGHT
    pub right_to_left_reading: bool, // WS_EX_RTLREADING
    pub left_scroll_bar: bool,       // WS_EX_LEFTSCROLLBAR
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
#[repr(C, packed)]
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
                visible: true,
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
            size_box: ucontain(style, WS_SIZEBOX),
            horizontal_roll: ucontain(style, WS_HSCROLL),
            vertical_roll: ucontain(style, WS_VSCROLL),
            clip_children: ucontain(style, WS_CLIPCHILDREN),
            disabled: ucontain(style, WS_DISABLED),
            visible: ucontain(style, WS_VISIBLE),
            dlg_modal_frame: ucontain(style_ex, WS_EX_DLGMODALFRAME),
            top_most: ucontain(style_ex, WS_EX_TOPMOST),
            accept_files: ucontain(style_ex, WS_EX_ACCEPTFILES),
            transparent: ucontain(style_ex, WS_EX_TRANSPARENT),
            tool_window: ucontain(style_ex, WS_EX_TOOLWINDOW),
            right_aligned: ucontain(style_ex, WS_EX_RIGHT),
            right_to_left_reading: ucontain(style_ex, WS_EX_RTLREADING),
            left_scroll_bar: ucontain(style_ex, WS_EX_LEFTSCROLLBAR),
            control_parent: ucontain(style_ex, WS_EX_CONTROLPARENT),
            app_window: ucontain(style_ex, WS_EX_APPWINDOW),
            no_inherit_layout: ucontain(style_ex, WS_EX_NOINHERITLAYOUT),
            right_layout: ucontain(style_ex, WS_EX_LAYOUTRTL),
            com_posited: ucontain(style_ex, WS_EX_COMPOSITED),
            no_auto_active: ucontain(style_ex, WS_EX_NOACTIVATE),
            no_redirection_bitmap: ucontain(style_ex, WS_EX_NOREDIRECTIONBITMAP),
            edge_type: WindowEdgeType::from(style_ex),
            size_state: WindowSizeState::from(style),
            border_type: WindowBorderType::from((style, style_ex)),
        }
    }
}
impl From<NormalWindowStyles> for (WINDOW_STYLE, WINDOW_EX_STYLE) {
    fn from(val: NormalWindowStyles) -> Self {
        let (mut style, mut style_ex) = val.border_type.into();
        set_style(&mut style, WS_SIZEBOX, val.size_box);
        set_style(&mut style, WS_HSCROLL, val.horizontal_roll);
        set_style(&mut style, WS_VSCROLL, val.vertical_roll);
        set_style(&mut style, WS_CLIPCHILDREN, val.clip_children);
        set_style(&mut style, WS_DISABLED, val.disabled);
        set_style(&mut style, WS_VISIBLE, val.visible);
        set_style(&mut style_ex, WS_EX_DLGMODALFRAME, val.dlg_modal_frame);
        set_style(&mut style_ex, WS_EX_TOPMOST, val.top_most);
        set_style(&mut style_ex, WS_EX_ACCEPTFILES, val.accept_files);
        set_style(&mut style_ex, WS_EX_TRANSPARENT, val.transparent);
        set_style(&mut style_ex, WS_EX_TOOLWINDOW, val.tool_window);
        set_style(&mut style_ex, WS_EX_RIGHT, val.right_aligned);
        set_style(&mut style_ex, WS_EX_RTLREADING, val.right_to_left_reading);
        set_style(&mut style_ex, WS_EX_LEFTSCROLLBAR, val.left_scroll_bar);
        set_style(&mut style_ex, WS_EX_CONTROLPARENT, val.control_parent);
        set_style(&mut style_ex, WS_EX_APPWINDOW, val.app_window);
        set_style(&mut style_ex, WS_EX_NOINHERITLAYOUT, val.no_inherit_layout);
        set_style(&mut style_ex, WS_EX_LAYOUTRTL, val.right_layout);
        set_style(&mut style_ex, WS_EX_COMPOSITED, val.com_posited);
        set_style(&mut style_ex, WS_EX_NOACTIVATE, val.no_auto_active);
        set_style(
            &mut style_ex,
            WS_EX_NOREDIRECTIONBITMAP,
            val.no_redirection_bitmap,
        );
        style |= <WindowSizeState as Into<WINDOW_STYLE>>::into(val.size_state);
        style_ex |= <WindowEdgeType as Into<WINDOW_EX_STYLE>>::into(val.edge_type);
        (style, style_ex)
    }
}

impl From<(WINDOW_STYLE, WINDOW_EX_STYLE)> for ChildWindowStyles {
    fn from((style, style_ex): (WINDOW_STYLE, WINDOW_EX_STYLE)) -> Self {
        Self {
            style: (style, style_ex).into(),
            tab_stop: ucontain(style, WS_TABSTOP),
            group_leader: ucontain(style, WS_GROUP),
            clip_isblings: ucontain(style, WS_CLIPSIBLINGS),
            no_parent_notify: ucontain(style_ex, WS_EX_NOPARENTNOTIFY),
            // mid_child: ucontain(style_ex, WS_EX_MDICHILD),
        }
    }
}
impl From<ChildWindowStyles> for (WINDOW_STYLE, WINDOW_EX_STYLE) {
    fn from(val: ChildWindowStyles) -> Self {
        let (mut style, mut style_ex) = val.style.into();
        set_style(&mut style, WS_TABSTOP, val.tab_stop);
        set_style(&mut style, WS_GROUP, val.group_leader);
        set_style(&mut style, WS_CLIPSIBLINGS, val.clip_isblings);
        set_style(&mut style_ex, WS_EX_NOPARENTNOTIFY, val.no_parent_notify);
        // set_style(&mut style_ex, WS_EX_MDICHILD, self.mid_child);
        (style | WS_CHILD, style_ex)
    }
}
#[derive(Debug)]
pub enum WindowType<'a> {
    Overlapped {
        style: NormalWindowStyles,
        menu: Option<MenuBar>,
        owner: Option<&'a Window>,
        is_layered: bool, //WS_EX_LAYERED
    }, //重叠窗口
    Popup {
        style: NormalWindowStyles,
        menu: Option<MenuBar>,
        owner: Option<&'a Window>,
        is_layered: bool, //WS_EX_LAYERED
    },
    Child {
        style: ChildWindowStyles,
        identifier: WindowID,
        parent: &'a Window,
        is_layered: bool, //WS_EX_LAYERED
    },
    MessageOnly,
}
// impl WindowType {
//     pub fn nullify_menu(&mut self) {
//         let _ = match self {
//             WindowType::Overlapped { menu, .. } => menu.take().map(|mut x| x.nullify()),
//             WindowType::Popup { menu, .. } => menu.take().map(|mut x| x.nullify()),
//             _ => Some(()),
//         };
//     }
// }
impl Default for WindowType<'_> {
    fn default() -> Self {
        Self::Overlapped {
            style: Default::default(),
            menu: None,
            owner: None,
            is_layered: false,
        }
    }
}
impl From<WindowType<'_>> for (WINDOW_STYLE, WINDOW_EX_STYLE, HMENU, HWND) {
    fn from(val: WindowType<'_>) -> Self {
        use WindowType::*;
        unsafe {
            match val {
                Overlapped {
                    style,
                    menu,
                    owner,
                    is_layered,
                } => {
                    let (style, mut style_ex) = style.into();
                    set_style(&mut style_ex, WS_EX_LAYERED, is_layered);
                    (
                        style,
                        style_ex,
                        menu.unwrap_or(MenuBar::null()).handle(),
                        match owner {
                            None => 0 as HWND,
                            Some(x) => x.handle(),
                        },
                    )
                }
                Popup {
                    style,
                    menu,
                    owner,
                    is_layered,
                } => {
                    let (style, mut style_ex) = style.into();
                    set_style(&mut style_ex, WS_EX_LAYERED, is_layered);
                    (
                        style | WS_POPUP,
                        style_ex,
                        menu.unwrap_or(MenuBar::null()).handle(),
                        match owner {
                            None => 0 as HWND,
                            Some(x) => x.handle(),
                        },
                    )
                }
                Child {
                    style,
                    identifier,
                    parent,
                    is_layered,
                } => {
                    let (style, mut style_ex) = style.into();
                    set_style(&mut style_ex, WS_EX_LAYERED, is_layered);
                    (style, style_ex, identifier as HMENU, parent.handle())
                }
                MessageOnly => (0u32, 0u32, 0 as HMENU, HWND_MESSAGE),
            }
        }
    }
}
impl WindowType<'_> {
    ///确保wnd是Rust拥有的
    pub unsafe fn from_data<'a>(
        style: WINDOW_STYLE,
        style_ex: WINDOW_EX_STYLE,
        menu: HMENU,
        wnd: &'a HWND,
    ) -> WindowType<'a> {
        use WindowType::*;
        if *wnd == HWND_MESSAGE {
            return MessageOnly;
        }
        let w: Option<&Window> = if wnd.is_null() {
            None
        } else {
            Some(Window::from_ref(wnd))
        };
        let m: Option<MenuBar> = if menu.is_null() {
            None
        } else {
            unsafe { Some(MenuBar::from_handle(menu)) }
        };
        if ucontain(style, WS_CHILD) {
            return Child {
                style: (style, style_ex).into(),
                identifier: menu as u16,
                parent: Window::from_ref(wnd),
                is_layered: ucontain(style_ex, WS_EX_LAYERED),
            };
        };
        if ucontain(style, WS_POPUP) {
            Popup {
                style: (style, style_ex).into(),
                menu: m,
                owner: w,
                is_layered: ucontain(style_ex, WS_EX_LAYERED),
            }
        } else {
            Overlapped {
                style: (style, style_ex).into(),
                menu: m,
                owner: w,
                is_layered: ucontain(style_ex, WS_EX_LAYERED),
            }
        }
    }
}
