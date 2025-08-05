use super::*;
#[derive(Default, Clone, Copy, Hash)]
pub enum BottonContentPos {
    #[default]
    DefaultPos,
    Center,      //BS_CENTER | BS_VCENTER
    Left,        //BS_LEFT | BS_VCENTER
    Right,       //BS_RIGHT | BS_VCENTER
    Top,         //BS_TOP | BS_CENTER
    Bottom,      //BS_BOTTOM | BS_CENTER
    TopLeft,     //BS_TOP | BS_LEFT
    TopRight,    //BS_TOP | BS_RIGHT
    BottomLeft,  //BS_BOTTOM | BS_LEFT
    BottomRight, //BS_BOTTOM | BS_RIGHT
}
impl Into<WINDOW_STYLE> for BottonContentPos {
    fn into(self) -> WINDOW_STYLE {
        use BottonContentPos::*;
        (match self {
            DefaultPos => 0,
            Center => BS_CENTER | BS_VCENTER,
            Left => BS_LEFT | BS_VCENTER,
            Right => BS_RIGHT | BS_VCENTER,
            Top => BS_TOP | BS_CENTER,
            Bottom => BS_BOTTOM | BS_CENTER,
            TopLeft => BS_TOP | BS_LEFT,
            TopRight => BS_TOP | BS_RIGHT,
            BottomLeft => BS_BOTTOM | BS_LEFT,
            BottomRight => BS_BOTTOM | BS_RIGHT,
        }) as WINDOW_STYLE
    }
}
#[derive(Default, Clone, Copy, Hash)]
pub enum ButtonType {
    #[default]
    Normal,
    Split,
    Link,
}
pub struct ButtonOption<T> {
    pub style: ChildWindowStyles,
    pub btype: ButtonType,
    pub contect: T,
    pub pos: BottonContentPos,
    pub extra_msg: bool, //BS_NOTIFY
    pub flat: bool,      //BS_FLAT
    pub focused: bool,
}
pub type ButtonStyle = ButtonOption<ButtonContent>;
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE, Option<ButtonImage>, String)> for ButtonStyle {
    fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE, Option<ButtonImage>, String) {
        let (mut ms_style, ex) = self.style.into();
        use ButtonType::*;
        ms_style |= match (self.btype, self.focused) {
            (Normal, false) => BS_PUSHBUTTON,
            (Normal, true) => BS_DEFPUSHBUTTON,
            (Split, false) => BS_SPLITBUTTON,
            (Split, true) => BS_DEFSPLITBUTTON,
            (Link, false) => BS_COMMANDLINK,
            (Link, true) => BS_DEFCOMMANDLINK,
        } as WINDOW_STYLE;
        set_style(&mut ms_style, BS_NOTIFY as WINDOW_STYLE, self.extra_msg);
        set_style(&mut ms_style, BS_FLAT as WINDOW_STYLE, self.flat);
        let (style2, ditype, text) = self.contect.into();
        let pos: WINDOW_STYLE = self.pos.into();
        (ms_style | style2 | pos, ex, ditype, text)
    }
}
impl ButtonStyle {
    pub fn new(btype: ButtonType, text: &str) -> Self {
        ButtonOption {
            style: ChildWindowStyles::default(),
            btype,
            contect: ButtonContent::new_text(text),
            pos: BottonContentPos::default(),
            extra_msg: false,
            flat: false,
            focused: false,
        }
    }
}
pub type ButtonTemple = ButtonOption<ButtonTempleContent>;
impl DialogTempleControl for ButtonTemple {
    fn pre_compile(self, pos: Point, size: Size, identifier: WindowID) -> ControlPreCompilePruduct {
        let (mut ms_style, style_ex) = self.style.into();
        use ButtonType::*;
        ms_style |= match (self.btype, self.focused) {
            (Normal, false) => BS_PUSHBUTTON,
            (Normal, true) => BS_DEFPUSHBUTTON,
            (Split, false) => BS_SPLITBUTTON,
            (Split, true) => BS_DEFSPLITBUTTON,
            (Link, false) => BS_COMMANDLINK,
            (Link, true) => BS_DEFCOMMANDLINK,
        } as WINDOW_STYLE;
        set_style(&mut ms_style, BS_NOTIFY as WINDOW_STYLE, self.extra_msg);
        set_style(&mut ms_style, BS_FLAT as WINDOW_STYLE, self.flat);
        let (style2, ct) = self.contect.into();
        let poss: WINDOW_STYLE = self.pos.into();
        ControlPreCompilePruduct::from(format!(
            "CONTROL \"{}\", {}, \"Button\", 0x{:04X}, {}, {}, {}, {}, 0x{:04X}",
            ct,
            identifier,
            (ms_style | style2 | poss),
            pos.x,
            pos.y,
            size.width,
            size.height,
            style_ex
        ))
    }
}
pub struct ButtonTempleContent {
    //BS_TEXT
    pub text: String,
    pub multiple_lines: bool, //BS_MULTILINE
}
impl ButtonTempleContent {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            multiple_lines: false,
        }
    }
}
impl Into<(WINDOW_STYLE, String)> for ButtonTempleContent {
    fn into(self) -> (WINDOW_STYLE, String) {
        (
            if self.multiple_lines {
                (BS_MULTILINE | BS_TEXT) as WINDOW_STYLE
            } else {
                BS_TEXT as WINDOW_STYLE
            },
            self.text,
        )
    }
}

pub enum ButtonContent {
    TextOnly {
        //BS_TEXT
        text: String,
        multiple_lines: bool, //BS_MULTILINE
    },
    IconOnly {
        //BS_ICON
        icon: ButtonImage,
        name: String,
    },
    IconAndText {
        //BS_TEXT BS_ICON
        icon: ButtonImage,
        text: String,
        multiple_lines: bool, //BS_MULTILINE
    },
}
impl ButtonContent {
    pub fn new_text(text: &str) -> Self {
        Self::TextOnly {
            text: text.to_string(),
            multiple_lines: false,
        }
    }
}
impl Into<(WINDOW_STYLE, Option<ButtonImage>, String)> for ButtonContent {
    fn into(self) -> (WINDOW_STYLE, Option<ButtonImage>, String) {
        match self {
            ButtonContent::TextOnly {
                text,
                multiple_lines,
            } => (
                if multiple_lines {
                    (BS_MULTILINE | BS_TEXT) as WINDOW_STYLE
                } else {
                    BS_TEXT as WINDOW_STYLE
                },
                None,
                text,
            ),
            ButtonContent::IconOnly { icon, name } => (
                if icon.is_left() { BS_BITMAP } else { BS_ICON } as WINDOW_STYLE,
                Some(icon),
                name,
            ),
            ButtonContent::IconAndText {
                icon,
                text,
                multiple_lines,
            } => (
                if multiple_lines {
                    (BS_MULTILINE | if icon.is_left() { BS_BITMAP } else { BS_ICON } | BS_TEXT)
                        as WINDOW_STYLE
                } else {
                    (if icon.is_left() { BS_BITMAP } else { BS_ICON } | BS_TEXT) as WINDOW_STYLE
                },
                Some(icon),
                text,
            ),
        }
    }
}
pub enum ButtonMsgType {
    MouseEntering,
    MouseLeaving,
    Clicked,
    DoubleClicked,
    LoseKeyboardFocus,
    GetKeyboardFocus,
    DropDown(Rect),
    Draw(usize),
    #[doc(hidden)]
    Fffffb21Msg, //4294966049这是什么？
}
const BCN_FFFFFB21_MSG: u32 = 4294966049;
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
            BN_DOUBLECLICKED => DoubleClicked,
            BN_KILLFOCUS => LoseKeyboardFocus,
            BN_SETFOCUS => GetKeyboardFocus,
            BCN_DROPDOWN => {
                let data = (*(ptr as *mut NMBCDROPDOWN)).rcButton;
                DropDown(euclid::Box2D::new(
                    Point::new(data.left, data.top),
                    Point::new(data.right, data.bottom),
                ).to_rect())
            }
            NM_CUSTOMDRAW => Draw(ptr),
            BCN_FFFFFB21_MSG => Fffffb21Msg, //这是什么？
            _ => {
                return Err(ERROR_MSG_CODE_NOT_SUPPORT);
            }
        }
    },
    {
        is_some_window(wnd, L!("Button"))
    },
    {
        todo!()
    }
}
// impl From(WINDOW_STYLE, Option<ButtonImage>) for ButtonDrawType {
// 	fn from(data: (WINDOW_STYLE, Option<ButtonImage>)) -> Self {
//
// 	}
// }
impl TextControl for Button {}
impl CommonControl for Button {
    type Style = ButtonStyle;
    fn new(
        wnd: &mut Window,
        pos: Option<Rect>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<Self> {
        let (style, ex, draw, name) = control_style.into();
        Ok(Self(new_button(
            wnd, name, pos, identifier, style, ex, font, draw,
        )?))
    }
}
impl Button {
    pub fn get_note(&self) -> Result<String> {
        let length =
            unsafe { SendMessageW(self.0.handle(), BCM_GETNOTELENGTH, 0 as WPARAM, 0 as LPARAM) }
                as usize;
        if length == 0 {
            return Ok(String::new());
        };
        let mut buffer: Vec<u16> = vec![0; length + 1];
        unsafe {
            SendMessageW(
                self.0.handle(),
                BCM_GETNOTE,
                length as WPARAM,
                buffer.as_mut_ptr() as LPARAM,
            );
        }
        Ok(String::from_utf16_lossy(&buffer[..length]))
    }
    pub fn set_note(&mut self, note: &str) -> Result<()> {
        let (note_ptr, _note_u16) = str_to_pcwstr(note);

        let _ = error_from_win32_zero_num!(SendMessageW(
            self.0.handle(),
            BCM_SETNOTE,
            0 as WPARAM,
            note_ptr as LPARAM,
        ))?;
        Ok(())
    }
}
