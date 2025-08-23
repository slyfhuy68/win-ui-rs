use super::*;
pub enum CaseType {
    Normal,
    Lower, // CBS_LOWERCASE
    Upper, // CBS_UPPERCASE
}
pub enum ComboBoxShow {
    ViewLike, //CBS_DROPDOWNLIST
    EditLike {
        auto_scroll: bool, //CBS_AUTOHSCROLL
        always_show: bool, //true CBS_SIMPLE false CBS_DROPDOWN
    },
}
pub struct ComboBoxStyle {
    pub style: ChildWindowStyles,
    pub contect: String,
    pub auto_hide_scroll: bool, //取反后的CBS_DISABLENOSCROLL
    pub auto_size: bool,        //取反后的CBS_NOINTEGRALHEIGHT
    pub auto_sort: bool,        //CBS_SORT
    pub case_type: CaseType,
    pub owner_draw: Option<OwnerDrawType>,
    pub show_type: ComboBoxShow,
}
pub struct OwnerDrawType {
    pub owner_save_list: bool, //CBS_HASSTRINGS
    pub variable_height: bool, //false: CBS_OWNERDRAWFIXED, true: CBS_OWNERDRAWVARIABLE
}
pub type ComboBoxTemple = ComboBoxStyle;
impl DialogTempleControl for ComboBoxTemple {
    fn pre_compile(self, pos: FontPoint, size: FontSize, identifier: WindowID) -> String {
        let ((ms_style, ex), ct) = self.into();
        format!(
            "CONTROL \"{}\", {}, \"ComboBox\", 0x{:04X}, {}, {}, {}, {}, 0x{:04X}",
            ct, identifier, ms_style, pos.x, pos.y, size.width, size.height, ex
        )
    }
}
impl ComboBoxStyle {
    pub fn new(s: &str) -> Self {
        Self {
            style: Default::default(),
            contect: s.to_string(),
            auto_hide_scroll: true, //取反后的CBS_DISABLENOSCROLL
            auto_size: true,        //取反后的CBS_NOINTEGRALHEIGHT
            auto_sort: false,       //CBS_SORT
            case_type: CaseType::Normal,
            owner_draw: None,
            show_type: ComboBoxShow::EditLike {
                auto_scroll: true,  //CBS_AUTOHSCROLL
                always_show: false, //true CBS_SIMPLE false CBS_DROPDOWN
            },
        }
    }
}
impl From<ComboBoxStyle> for ((WINDOW_STYLE, WINDOW_EX_STYLE), String) {
    fn from(val: ComboBoxStyle) -> Self {
        use CaseType::*;
        use ComboBoxShow::*;
        let mut style: i32 = 0;
        let (style1, ex) = val.style.into();
        match val.show_type {
            ViewLike => style |= CBS_DROPDOWNLIST,
            EditLike {
                auto_scroll,
                always_show,
            } => {
                set_istyle(&mut style, CBS_AUTOHSCROLL, auto_scroll);
                if always_show {
                    style |= CBS_SIMPLE;
                } else {
                    style |= CBS_DROPDOWN;
                };
            }
        }
        set_istyle(&mut style, CBS_DISABLENOSCROLL, !val.auto_hide_scroll);
        set_istyle(&mut style, CBS_NOINTEGRALHEIGHT, !val.auto_size);
        set_istyle(&mut style, CBS_SORT, val.auto_sort);
        match val.case_type {
            Normal => {}
            Lower => style |= CBS_LOWERCASE,
            Upper => style |= CBS_UPPERCASE,
        }
        if let Some(owner_draw) = val.owner_draw {
            set_istyle(&mut style, CBS_HASSTRINGS, owner_draw.owner_save_list);
            if owner_draw.variable_height {
                style |= CBS_OWNERDRAWVARIABLE;
            } else {
                style |= CBS_OWNERDRAWFIXED;
            }
        }
        (((style as WINDOW_STYLE) | style1, ex), val.contect)
    }
}
impl ComboBoxStyle {
    pub fn new_view(s: &str) -> Self {
        Self {
            style: Default::default(),
            contect: s.to_string(),
            auto_hide_scroll: true, //取反后的CBS_DISABLENOSCROLL
            auto_size: true,        //取反后的CBS_NOINTEGRALHEIGHT
            auto_sort: false,       //CBS_SORT
            case_type: CaseType::Normal,
            owner_draw: None,
            show_type: ComboBoxShow::ViewLike,
        }
    }
    pub fn new_edit(s: &str) -> Self {
        Self {
            style: ChildWindowStyles {
                style: NormalWindowStyles {
                    edge_type: WindowEdgeType::Sunken,
                    ..Default::default()
                },
                ..Default::default()
            },
            contect: s.to_string(),
            auto_hide_scroll: true, //取反后的CBS_DISABLENOSCROLL
            auto_size: true,        //取反后的CBS_NOINTEGRALHEIGHT
            auto_sort: false,       //CBS_SORT
            case_type: CaseType::Normal,
            owner_draw: None,
            show_type: ComboBoxShow::EditLike {
                auto_scroll: true,  //CBS_AUTOHSCROLL
                always_show: false, //true CBS_SIMPLE false CBS_DROPDOWN
            },
        }
    }
}
impl CommonControl for ComboBox {
    type Style = ComboBoxStyle;
    #[inline]
    fn new_raw(
        wnd: &mut Window,
        pos: Option<Rect>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<HWND> {
        let (a, b) = control_style.into();
        new_control(wnd, w!("ComboBox"), b, pos, identifier, a, font)
    }
}
define_control! {
    ComboBox,
    "ComboBox",
    {
        const CBN_CLOSEUP_FIX: u32 = CBN_CLOSEUP;
        const CBN_ERRSPACE_FIX: u32 = CBN_ERRSPACE as u32;
        match code {
            CBN_CLOSEUP_FIX => ListClose,
            CBN_DBLCLK => DoubleClick,
            CBN_DROPDOWN => DropDown,
            CBN_EDITCHANGE => EditChange(true),
            CBN_EDITUPDATE => EditChange(false),
            CBN_ERRSPACE_FIX => NoEnoughMemory,
            CBN_KILLFOCUS => LoseKeyboardFocus,
            CBN_SELCHANGE => SelChanged,
            CBN_SELENDCANCEL => SelEnDok,
            CBN_SELENDOK => SelEnCancel,
            CBN_SETFOCUS=> GetKeyboardFocus,
            WM_COMPAREITEM => Compareing(ptr),
            WM_DRAWITEM => Draw(ptr),
            WM_MEASUREITEM => Measure(ptr),
            _ => return Err(ERROR_MSG_CODE_NOT_SUPPORT),
        }
    },
    {
        is_some_window(wnd, L!("ComboBox"))
    },
    {
        todo!()
    }
}
pub enum ComboBoxMsgType {
    ListClose,
    DoubleClick,
    DropDown,
    EditChange(
        ///是否已更新屏幕
        bool,
    ),
    NoEnoughMemory,
    LoseKeyboardFocus,
    GetKeyboardFocus,
    SelChanged,
    SelEnDok,
    SelEnCancel,
    Compareing(usize),
    Draw(usize),
    Measure(usize),
}
pub type ListBoxItemPos = u16;
pub type ListBoxMaxSize = u16;
impl ComboBox {
    pub fn add_item(&mut self, text: &str) -> Result<Option<ListBoxItemPos>> {
        let (text_ptr, _text_u16) = str_to_pcwstr(text);
        match error_from_win32_zero_num!(SendMessageW(
            self.0.handle(),
            CB_ADDSTRING,
            0 as WPARAM,
            text_ptr as LPARAM,
        ))? as i32
        {
            0 => Ok(Some(0)),
            CB_ERR => Ok(None),
            CB_ERRSPACE => Err(ERROR_NOT_ENOUGH_MEMORY),
            x => Ok(Some(x as ListBoxItemPos)),
        }
    }
    pub fn remove_item(&mut self, pos: ListBoxItemPos) -> Result<Option<ListBoxMaxSize>> {
        match error_from_win32_zero_num!(SendMessageW(
            self.0.handle(),
            CB_DELETESTRING,
            pos as WPARAM,
            0 as LPARAM
        ))? as i32
        {
            0 => Ok(Some(0)),
            CB_ERR => Ok(None),
            x => Ok(Some(x as ListBoxMaxSize)),
        }
    }
    ///在项目列表中搜索以指定字符串中的字符****开头****的项。    
    ///搜索不区分大小写    
    pub fn element_offset_start(
        &self,
        start: ListBoxItemPos,
        text: &str,
    ) -> Result<Option<ListBoxItemPos>> {
        let (text_ptr, _text_u16) = str_to_pcwstr(text);
        match error_from_win32_zero_num!(SendMessageW(
            self.0.handle(),
            CB_FINDSTRING,
            (start - 1) as WPARAM,
            (text_ptr) as LPARAM,
        ))? as i32
        {
            0 => Ok(Some(0)),
            CB_ERR => Ok(None),
            x => Ok(Some(x as ListBoxItemPos)),
        }
    }
    ///在项目列表中搜索与指定字符串中的字符****匹配****的项。    
    ///搜索不区分大小写    
    pub fn element_offset(
        &self,
        start: ListBoxItemPos,
        text: &str,
    ) -> Result<Option<ListBoxItemPos>> {
        let (text_ptr, _text_u16) = str_to_pcwstr(text);
        match error_from_win32_zero_num!(SendMessageW(
            self.0.handle(),
            CB_FINDSTRINGEXACT,
            (start - 1) as WPARAM,
            text_ptr as LPARAM,
        ))? as i32
        {
            0 => Ok(Some(0)),
            CB_ERR => Ok(None),
            x => Ok(Some(x as ListBoxItemPos)),
        }
    }
    pub fn info(&self) {
        todo!() // GetComboBoxInfo
    }
    pub fn count(&self) -> Result<Option<ListBoxMaxSize>> {
        match error_from_win32_zero_num!(SendMessageW(
            self.0.handle(),
            CB_GETCOUNT,
            0 as WPARAM,
            0 as LPARAM
        ))? as i32
        {
            0 => Ok(Some(0)),
            CB_ERR => Ok(None),
            x => Ok(Some(x as ListBoxMaxSize)),
        }
    }
    pub fn get_cur_sel(&self) -> Result<Option<ListBoxItemPos>> {
        match error_from_win32_zero_num!(SendMessageW(
            self.0.handle(),
            CB_GETCURSEL,
            0 as WPARAM,
            0 as LPARAM
        ))? as i32
        {
            0 => Ok(Some(0)),
            CB_ERR => Ok(None),
            x => Ok(Some(x as ListBoxItemPos)),
        }
    }
    pub fn get_item_raw(&mut self, pos: ListBoxItemPos) -> Result<Option<isize>> {
        let data = error_from_win32_zero_num!(SendMessageW(
            self.0.handle(),
            CB_GETITEMDATA,
            pos as WPARAM,
            0 as LPARAM
        ))?;
        match data as i32 {
            0 => Ok(Some(0)),
            CB_ERR => Ok(None),
            _ => Ok(Some(data)),
        }
    }
    // CB_GETITEMHEIGHT
    // CB_GETLBTEXT
    // CB_GETLBTEXTLEN
    // CB_GETLOCALE
    // CB_GETMINVISIBLE
    // CB_GETTOPINDEX
    // CB_INITSTORAGE
    // CB_INSERTSTRING
    // CB_LIMITTEXT
    // CB_RESETCONTENT
    // CB_SELECTSTRING
    // CB_SETCUEBANNER
    // CB_SETCURSEL
    // CB_SETDROPPEDWIDTH
    // CB_SETEDITSEL
    // CB_SETEXTENDEDUI
    // CB_SETHORIZONTALEXTENT
    // CB_SETITEMDATA
    // CB_SETITEMHEIGHT
    // CB_SETLOCALE
    // CB_SETMINVISIBLE
    // CB_SETTOPINDEX
    // CB_SHOWDROPDOWN
}
impl TextControl for ComboBox {
    const INSUFFICIENT_SPACE_RESULT: u32 = CB_ERRSPACE as u32;
}
