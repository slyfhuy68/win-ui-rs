use super::*;
pub enum ButtonMsgType {
    MouseEntering,
    MouseLeaving,
    Clicked,
    DoubleClicked,
    LoseKeyboardFocus,
    GetKeyboardFocus,
    Draw(usize),
}
define_control! {
    ManuallyDrawButton,
    "Button",
    {
        match code {
            BCN_HOTITEMCHANGE => {
                let data = *(ptr as *mut NMBCHOTITEM);
                if data.dwFlags == HICF_MOUSE | HICF_ENTERING {
                    ManuallyDrawButtonMsgType::MouseEntering
                } else if data.dwFlags == HICF_MOUSE | HICF_LEAVING {
                    ManuallyDrawButtonMsgType::MouseLeaving
                } else {
                    return Err(ERROR_MSG_CODE_NOT_SUPPORT);
                }
            }
            BN_CLICKED => ManuallyDrawButtonMsgType::Clicked,
            BN_DBLCLK => ManuallyDrawButtonMsgType::DoubleClicked,
            BN_KILLFOCUS => ManuallyDrawButtonMsgType::LoseKeyboardFocus,
            BN_SETFOCUS => ManuallyDrawButtonMsgType::GetKeyboardFocus,
            _ => return Err(ERROR_MSG_CODE_NOT_SUPPORT),
        }
    },
    {
        if !is_button_window(wnd)? {
            return Ok(false);
        }
        Ok(
            style_of(wnd)
                .contains(WINDOW_STYLE(BS_OWNERDRAW as u32)),
        )
    },
    {
        todo!()
    }
}
pub enum ManuallyDrawButtonMsgType {
    MouseEntering,
    MouseLeaving,
    Clicked,
    DoubleClicked,
    LoseKeyboardFocus,
    GetKeyboardFocus,
}
pub struct ManuallyDrawButtonStyle(ChildWindowStyles);
impl
    Into<(
        WINDOW_STYLE,
        Option<Either<Bitmap, Icon>>,
        ChildWindowStyles,
    )> for ManuallyDrawButtonStyle
{
    fn into(
        self,
    ) -> (
        WINDOW_STYLE,
        Option<Either<Bitmap, Icon>>,
        ChildWindowStyles,
    ) {
        (WINDOW_STYLE(BS_OWNERDRAW as u32), None, self.0)
    }
}
impl ButtonControl for ManuallyDrawButton {
    type Style = ManuallyDrawButtonStyle;
}

//-----------------------------------按钮-----------------------------------------
#[derive(Default)]
pub enum BottonContentPos {
    #[default]
    Center, //BS_CENTER | BS_VCENTER
    Left,        //BS_LEFT | BS_VCENTER
    Right,       //BS_RIGHT | BS_VCENTER
    Top,         //BS_TOP | BS_CENTER
    Bottom,      //BS_BOTTOM | BS_CENTER
    TopLeft,     //BS_TOP | BS_LEFT
    TopRight,    //BS_TOP | BS_RIGHT
    BottomLeft,  //BS_BOTTOM | BS_LEFT
    BottomRight, //BS_BOTTOM | BS_RIGHT
}
pub struct ButtonStyle {
    pub extra_msg: bool, //BS_NOTIFY
    pub light: bool,     //if light BS_DEFPUSHBUTTON else BS_PUSHBUTTON
    pub flat: bool,      //BS_FLAT
}
impl Into<WINDOW_STYLE> for ButtonStyle {
    fn into(self) -> WINDOW_STYLE {
        let mut ms_style = WINDOW_STYLE(0u32);
        if self.extra_msg {
            ms_style |= WINDOW_STYLE(BS_NOTIFY as u32);
        };
        if self.flat {
            ms_style |= WINDOW_STYLE(BS_FLAT as u32);
        };
        if self.light {
            ms_style |= WINDOW_STYLE(BS_DEFPUSHBUTTON as u32);
        } else {
            ms_style |= WINDOW_STYLE(BS_PUSHBUTTON as u32);
        };
        ms_style
    }
}
impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            extra_msg: false,
            light: false,
            flat: true,
        }
    }
}
pub enum ButtonAutoDrawType {
    IconOnly(Either<Bitmap, Icon>),          //BS_ICON
    TextOnly(bool),                          //bool:multiple_lines BS_TEXT
    IconAndText(Either<Bitmap, Icon>, bool), //bool:BS_MULTILINE, BS_TEXT BS_ICON
}
define_control! {
    Button,
    "Button",
    {
        match code {
            BCN_HOTITEMCHANGE => {
                let data = *(ptr as *mut NMBCHOTITEM);
                if data.dwFlags == HICF_MOUSE | HICF_ENTERING {
                    MouseEntering
                } else if data.dwFlags == HICF_MOUSE | HICF_LEAVING {
                    MouseLeaving
                } else {
                    return Err(ERROR_MSG_CODE_NOT_SUPPORT);
                }
            }
            BN_CLICKED => Clicked,
            BN_DBLCLK => DoubleClicked,
            BN_KILLFOCUS => LoseKeyboardFocus,
            BN_SETFOCUS => GetKeyboardFocus,
            NM_CUSTOMDRAW => Draw(ptr),
            _ => return Err(ERROR_MSG_CODE_NOT_SUPPORT),
        }
    },
    {
        if !is_button_window(wnd)? {
            return Ok(false);
        }
        let style = style_of_raw(wnd);
        if (style & BS_3STATE)==0 && (style & BS_AUTO3STATE)==0 && (style & BS_AUTOCHECKBOX)==0 &&
        (style & BS_AUTORADIOBUTTON)==0 && (style & BS_CHECKBOX)==0 && (style & BS_COMMANDLINK)==0 &&
        (style & BS_DEFCOMMANDLINK)==0 && (style & BS_DEFSPLITBUTTON)==0 &&
        (style & BS_GROUPBOX)==0 && (style & BS_OWNERDRAW)==0 &&
        (style & BS_RADIOBUTTON)==0 && (style & BS_SPLITBUTTON)==0
        {
            return Ok(true);
        }
        Ok(false)
    },
    {
        todo!()
    }
}
pub struct ButtonDrawType(
    pub ButtonAutoDrawType,
    pub ButtonStyle,
    pub ChildWindowStyles,
);

impl Default for ButtonDrawType {
    fn default() -> Self {
        Self(
            ButtonAutoDrawType::TextOnly(false),
            Default::default(),
            Default::default(),
        )
    }
}
impl
    Into<(
        WINDOW_STYLE,
        Option<Either<Bitmap, Icon>>,
        ChildWindowStyles,
    )> for ButtonDrawType
{
    fn into(
        self,
    ) -> (
        WINDOW_STYLE,
        Option<Either<Bitmap, Icon>>,
        ChildWindowStyles,
    ) {
        let ButtonDrawType(dtype, bstyle, style) = self;
        let mut wstyle = WINDOW_STYLE(0);
        let ditype = match dtype {
            ButtonAutoDrawType::IconOnly(boi) => Some(boi),
            ButtonAutoDrawType::TextOnly(a) => {
                if a {
                    wstyle |= WINDOW_STYLE(BS_MULTILINE as u32);
                };
                None
            }
            ButtonAutoDrawType::IconAndText(boi, a) => {
                if a {
                    wstyle |= WINDOW_STYLE(BS_MULTILINE as u32)
                };
                Some(boi)
            }
        };
        wstyle |= bstyle.into();
        (wstyle, ditype, style)
    }
}
// impl From(WINDOW_STYLE, Option<Either<Bitmap, Icon>>) for ButtonDrawType {
// 	fn from(data: (WINDOW_STYLE, Option<Either<Bitmap, Icon>>)) -> Self {
//
// 	}
// }
impl ButtonControl for Button {
    type Style = ButtonDrawType;
}
//------------------------------------------分隔按钮----------------------------------
pub struct SplitButtonStyle {
    pub extra_msg: bool, //BS_NOTIFY
    pub light: bool,     //if light BS_DEFSPLITBUTTON else BS_SPLITBUTTON
    pub flat: bool,      //BS_FLAT
}
impl Into<WINDOW_STYLE> for SplitButtonStyle {
    fn into(self) -> WINDOW_STYLE {
        let mut ms_style = WINDOW_STYLE(0u32);
        if self.extra_msg {
            ms_style |= WINDOW_STYLE(BS_NOTIFY as u32);
        };
        if self.flat {
            ms_style |= WINDOW_STYLE(BS_FLAT as u32);
        };
        if self.light {
            ms_style |= WINDOW_STYLE(BS_DEFSPLITBUTTON as u32);
        } else {
            ms_style |= WINDOW_STYLE(BS_SPLITBUTTON as u32);
        };
        ms_style
    }
}
impl Default for SplitButtonStyle {
    fn default() -> Self {
        Self {
            extra_msg: false,
            light: false,
            flat: true,
        }
    }
}
pub enum SplitButtonMsgType {
    MouseEntering,
    MouseLeaving,
    Clicked,
    DoubleClicked,
    LoseKeyboardFocus,
    GetKeyboardFocus,
    DropDown(Rectangle),
    Draw(usize),
    #[doc(hidden)]
    Fffffb21Msg, //4294966049这是什么？
}
const BCN_FFFFFB21_MSG: u32 = 4294966049;
define_control! {
    SplitButton,
    "Button",
    {
        match code {
            BCN_HOTITEMCHANGE => {
                let data = *(ptr as *mut NMBCHOTITEM);
                if data.dwFlags == HICF_MOUSE | HICF_ENTERING {
                    MouseEntering
                } else if data.dwFlags == HICF_MOUSE | HICF_LEAVING {
                    MouseLeaving
                } else {
                    return Err(ERROR_MSG_CODE_NOT_SUPPORT);
                }
            }
            BN_CLICKED => Clicked,
            BN_DOUBLECLICKED => DoubleClicked,
            BN_KILLFOCUS => LoseKeyboardFocus,
            BN_SETFOCUS => GetKeyboardFocus,
            BCN_DROPDOWN => {
                let data = (*(ptr as *mut NMBCDROPDOWN)).rcButton;
                DropDown(Rectangle::Points(
                    Point(data.left, data.top),
                    Point(data.right, data.bottom),
                ))
            }
            NM_CUSTOMDRAW => Draw(ptr),
            BCN_FFFFFB21_MSG => Fffffb21Msg, //这是什么？
            _ => {
                return Err(ERROR_MSG_CODE_NOT_SUPPORT);
            }
        }
    },
    {
        if !is_button_window(wnd)? {
            return Ok(false);
        }
        let style = style_of_raw(wnd);
        if (style & BS_DEFSPLITBUTTON) != 0 || (style & BS_SPLITBUTTON) != 0 {
            return Ok(true);
        }
        Ok(false)
    },
    {
        todo!()
    }
}
pub struct SplitButtonDrawType(
    pub ButtonAutoDrawType,
    pub SplitButtonStyle,
    pub ChildWindowStyles,
);
impl Default for SplitButtonDrawType {
    fn default() -> Self {
        Self(
            ButtonAutoDrawType::TextOnly(false),
            Default::default(),
            Default::default(),
        )
    }
}
impl
    Into<(
        WINDOW_STYLE,
        Option<Either<Bitmap, Icon>>,
        ChildWindowStyles,
    )> for SplitButtonDrawType
{
    fn into(
        self,
    ) -> (
        WINDOW_STYLE,
        Option<Either<Bitmap, Icon>>,
        ChildWindowStyles,
    ) {
        let SplitButtonDrawType(dtype, bstyle, aaa) = self;
        let mut wstyle = WINDOW_STYLE(0);
        let ditype = match dtype {
            ButtonAutoDrawType::IconOnly(boi) => Some(boi),
            ButtonAutoDrawType::TextOnly(a) => {
                if a {
                    wstyle |= WINDOW_STYLE(BS_MULTILINE as u32);
                };
                None
            }
            ButtonAutoDrawType::IconAndText(boi, a) => {
                if a {
                    wstyle |= WINDOW_STYLE(BS_MULTILINE as u32)
                };
                Some(boi)
            }
        };
        wstyle |= bstyle.into();
        (wstyle, ditype, aaa)
    }
}
// impl From(WINDOW_STYLE, Option<BitmapOrIcon>) for SplitButtonDrawType {
// 	fn from(data: (WINDOW_STYLE, Option<BitmapOrIcon>)) -> Self {
//
// 	}
// }
impl ButtonControl for SplitButton {
    type Style = SplitButtonDrawType;
}
//------------------------------------------链接按钮----------------------------------
pub struct LinkButtonStyle {
    pub extra_msg: bool, //BS_NOTIFY
    pub light: bool,     //if light BS_DEFCOMMANDLINK else BS_COMMANDLINK
    pub flat: bool,      //BS_FLAT
}
impl Into<WINDOW_STYLE> for LinkButtonStyle {
    fn into(self) -> WINDOW_STYLE {
        let mut ms_style = WINDOW_STYLE(0u32);
        if self.extra_msg {
            ms_style |= WINDOW_STYLE(BS_NOTIFY as u32);
        };
        if self.flat {
            ms_style |= WINDOW_STYLE(BS_FLAT as u32);
        };
        if self.light {
            ms_style |= WINDOW_STYLE(BS_DEFCOMMANDLINK as u32);
        } else {
            ms_style |= WINDOW_STYLE(BS_COMMANDLINK as u32);
        };
        ms_style
    }
}
impl Default for LinkButtonStyle {
    fn default() -> Self {
        Self {
            extra_msg: false,
            light: false,
            flat: true,
        }
    }
}
pub use ButtonMsgType as LinkButtonMsgType;
define_control! {
    LinkButton,
    "Button",
    {
        match code {
            BCN_HOTITEMCHANGE => {
                let data = *(ptr as *mut NMBCHOTITEM);
                if data.dwFlags == HICF_MOUSE | HICF_ENTERING {
                    MouseEntering
                } else if data.dwFlags == HICF_MOUSE | HICF_LEAVING {
                    MouseLeaving
                } else {
                    return Err(ERROR_MSG_CODE_NOT_SUPPORT);
                }
            }
            BN_CLICKED => Clicked,
            BN_DOUBLECLICKED => DoubleClicked,
            BN_KILLFOCUS => LoseKeyboardFocus,
            BN_SETFOCUS => GetKeyboardFocus,
            NM_CUSTOMDRAW => Draw(ptr),
            _ => return Err(ERROR_MSG_CODE_NOT_SUPPORT),
        }
    },
    {
        if !is_button_window(wnd)? {
            return Ok(false);
        }
        let style = style_of_raw(wnd);
        if (style & BS_DEFCOMMANDLINK) != 0 || (style & BS_COMMANDLINK) != 0 {
            return Ok(true);
        }
        Ok(false)
    },
    {
        todo!()
    }
}
pub struct LinkButtonDrawType(
    pub ButtonAutoDrawType,
    pub LinkButtonStyle,
    pub ChildWindowStyles,
);
impl Default for LinkButtonDrawType {
    fn default() -> Self {
        Self(
            ButtonAutoDrawType::TextOnly(false),
            Default::default(),
            Default::default(),
        )
    }
}
impl
    Into<(
        WINDOW_STYLE,
        Option<Either<Bitmap, Icon>>,
        ChildWindowStyles,
    )> for LinkButtonDrawType
{
    fn into(
        self,
    ) -> (
        WINDOW_STYLE,
        Option<Either<Bitmap, Icon>>,
        ChildWindowStyles,
    ) {
        let LinkButtonDrawType(dtype, bstyle, aaa) = self;
        let mut wstyle = WINDOW_STYLE(0);
        let ditype = match dtype {
            ButtonAutoDrawType::IconOnly(boi) => Some(boi),
            ButtonAutoDrawType::TextOnly(a) => {
                if a {
                    wstyle |= WINDOW_STYLE(BS_MULTILINE as u32);
                };
                None
            }
            ButtonAutoDrawType::IconAndText(boi, a) => {
                if a {
                    wstyle |= WINDOW_STYLE(BS_MULTILINE as u32)
                };
                Some(boi)
            }
        };
        wstyle |= bstyle.into();
        (wstyle, ditype, aaa)
    }
}
// impl From(WINDOW_STYLE, Option<BitmapOrIcon>) for LinkButtonDrawType {
// 	fn from(data: (WINDOW_STYLE, Option<BitmapOrIcon>)) -> Self {
//
// 	}
// }
impl ButtonControl for LinkButton {
    type Style = LinkButtonDrawType;
}
impl LinkButton {
    pub fn get_note(&self) -> Result<String> {
        let length = unsafe {
            SendMessageW(
                self.0.handle(),
                BCM_GETNOTELENGTH,
                Some(WPARAM(0)),
                Some(LPARAM(0)),
            )
            .0
        } as usize;
        if length == 0 {
            return Ok(String::new());
        };
        let mut buffer: Vec<u16> = vec![0; length + 1];
        unsafe {
            SendMessageW(
                self.0.handle(),
                BCM_GETNOTE,
                Some(WPARAM(length)),
                Some(LPARAM(buffer.as_mut_ptr() as isize)),
            )
            .0;
        }
        Ok(String::from_utf16_lossy(&buffer[..length]))
    }
    pub fn set_note(&mut self, note: &str) -> Result<()> {
        let (note_ptr, _note_u16) = str_to_pcwstr(note);

        if unsafe {
            SendMessageW(
                self.0.handle(),
                BCM_SETNOTE,
                Some(WPARAM(0)),
                Some(LPARAM(note_ptr.0 as isize)),
            )
        }
        .0 == 0
        {
            return Err(Error::correct_error());
        }
        Ok(())
    }
}
