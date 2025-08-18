use super::*;
pub struct ListBoxStyle {
    pub style: ChildWindowStyles,
    pub name: String,
    pub auto_hide_scroll: bool,      //取反后的LBS_DISABLENOSCROLL
    pub delayed_rendering: bool,     //LBS_NOREDRAW
    pub extra_nofity: bool,          //LBS_NOTIFY
    pub extra_keyboard_nofity: bool, //LBS_WANTKEYBOARDINPUT
    pub draw_type: DrawType,
    pub sel_type: SelType,
}
pub enum DrawType {
    OwnerDraw(OwnerSaveDataType),
    AutoDraw {
        auto_size: bool, //取反后的LBS_NOINTEGRALHEIGHT
        auto_sort: bool, //LBS_SORT
        costom_tab_size: bool,
    },
}
impl Default for DrawType {
    fn default() -> Self {
        DrawType::AutoDraw {
            auto_size: true,
            auto_sort: false,
            costom_tab_size: false,
        }
    }
}
pub enum OwnerSaveDataType {
    Yes, //LBS_NODATA | LBS_OWNERDRAWFIXED
    No {
        owner_save_list: bool, //取反的LBS_HASSTRINGS
        auto_sort: bool,       //LBS_SORT
        //None: LBS_OWNERDRAWVARIABLE
        //true: LBS_OWNERDRAWFIXED | LBS_NOINTEGRALHEIGHT
        //false: LBS_OWNERDRAWFIXED
        fixed_height: Option<bool>,
    },
}
impl Default for OwnerSaveDataType {
    fn default() -> Self {
        OwnerSaveDataType::No {
            owner_save_list: true,
            auto_sort: false,
            fixed_height: Some(false),
        }
    }
}
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE, String)> for ListBoxStyle {
    fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE, String) {
        let (mut style, ex) = self.style.into();
        style |= (((!self.auto_hide_scroll) as i32) * LBS_DISABLENOSCROLL
            + (self.delayed_rendering as i32) * LBS_NOREDRAW
            + (self.extra_nofity as i32) * LBS_NOTIFY
            + (self.extra_keyboard_nofity as i32) * LBS_WANTKEYBOARDINPUT)
            as WINDOW_STYLE;
        match self.draw_type {
            DrawType::AutoDraw {
                auto_size,
                auto_sort,
                costom_tab_size,
            } => {
                style |= (((!auto_size) as i32) * LBS_NOINTEGRALHEIGHT
                    + (auto_sort as i32) * LBS_SORT
                    + (costom_tab_size as i32) * LBS_USETABSTOPS)
                    as WINDOW_STYLE;
            }
            DrawType::OwnerDraw(sdt) => match sdt {
                OwnerSaveDataType::Yes => {
                    style |= (LBS_NODATA | LBS_OWNERDRAWFIXED) as WINDOW_STYLE
                }
                OwnerSaveDataType::No {
                    owner_save_list,
                    auto_sort,
                    fixed_height,
                } => {
                    style |= (((!owner_save_list) as i32) * LBS_HASSTRINGS
                        + (auto_sort as i32) * CBS_SORT
                        + match fixed_height {
                            None => LBS_OWNERDRAWVARIABLE,
                            Some(x) => LBS_OWNERDRAWFIXED + (x as i32) * LBS_NOINTEGRALHEIGHT,
                        }) as WINDOW_STYLE;
                }
            },
        };

        match self.sel_type {
            SelType::Allow {
                multiple_selection,
                ext_selection,
            } => {
                style |= ((multiple_selection as i32) * LBS_MULTICOLUMN
                    + (ext_selection as i32) * LBS_EXTENDEDSEL)
                    as WINDOW_STYLE;
            }
            SelType::Forbid => style |= LBS_NOSEL as WINDOW_STYLE,
        }
        (style, ex, self.name)
    }
}
type ListBoxTemple = ListBoxStyle;
impl DialogTempleControl for ListBoxTemple {
    #[inline]
    fn pre_compile(self, pos: FontPoint, size: FontSize, identifier: WindowID) -> String {
        let (ms_style, style_ex, ct) = self.into();
        format!(
            "CONTROL \"{}\", {}, \"ListBox\", 0x{:04X}, {}, {}, {}, {}, 0x{:04X}",
            ct, identifier, ms_style, pos.x, pos.y, size.width, size.height, style_ex
        )
    }
}
impl ListBoxStyle {
    pub fn new(name: &str) -> Self {
        ListBoxStyle {
            style: ChildWindowStyles::default(),
            name: name.to_string(),
            auto_hide_scroll: true,
            delayed_rendering: false,
            extra_nofity: false,
            extra_keyboard_nofity: false,
            draw_type: DrawType::default(),
            sel_type: SelType::default(),
        }
    }
}
pub enum ListBoxMsgType {
    DoubleClick,       //LBN_DBLCLK
    NoEnoughMemory,    //LBN_ERRSPACE
    LoseKeyboardFocus, //LBN_KILLFOCUS
    GetKeyboardFocus,  //LBN_SETFOCUS
    SelectionCanceled, //LBN_SELCANCEL
    SelectionChanged,  //LBN_SELCHANGE
    Colour(usize),     //WM_CTLCOLORLISTBOX
                       // 需要主函数支持：
                       // WM_CHARTOITEM
                       // WM_DELETEITEM
                       // WM_VKEYTOITEM
                       // DL_BEGINDRAG
                       // DL_CANCELDRAG
                       // DL_DRAGGING
                       // DL_DROPPED
}
pub enum SelType {
    Allow {
        multiple_selection: bool, //LBS_MULTICOLUMN
        ext_selection: bool,      //LBS_EXTENDEDSEL
    },
    Forbid, //LBS_NOSEL
}
impl Default for SelType {
    fn default() -> Self {
        SelType::Allow {
            multiple_selection: false,
            ext_selection: false,
        }
    }
}
const LBN_ERRSPACE_FIX: u32 = LBN_ERRSPACE as u32;
define_control! {
    ListBox,
    "ListBox",
    {
        match code {
            LBN_DBLCLK => DoubleClick,
            LBN_ERRSPACE_FIX => NoEnoughMemory,
            LBN_KILLFOCUS => LoseKeyboardFocus,
            LBN_SETFOCUS => GetKeyboardFocus,
            LBN_SELCANCEL => SelectionCanceled,
            LBN_SELCHANGE => SelectionChanged,
            WM_CTLCOLORLISTBOX => {
                let nmhdr = (*(ptr as *mut NMHDRCOLOR)).DC;
                Colour(nmhdr as usize)
            },
            _ => {
                return Err(ERROR_MSG_CODE_NOT_SUPPORT);
            }
        }
    },
    {
        is_some_window(wnd, L!("ListBox"))
    },
    {
        todo!()
    }
}
impl TextControl for ListBox {
    const INSUFFICIENT_SPACE_RESULT: u32 = LB_ERRSPACE as u32;
}
impl CommonControl for ListBox {
    type Style = ListBoxStyle;
    #[inline]
    fn new_raw(
        wnd: &mut Window,
        pos: Option<Rect>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<HWND> {
        let (a, b, name) = control_style.into();
        new_control(wnd, w!("Edit"), name, pos, identifier, a, b, font)
    }
}
