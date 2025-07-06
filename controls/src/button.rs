use super::*;
#[derive(Default, Clone, Copy, Hash)]
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
impl Into<WINDOW_STYLE> for BottonContentPos {
    fn into(self) -> WINDOW_STYLE {
        use BottonContentPos::*;
        WINDOW_STYLE(match self {
            Center => BS_CENTER | BS_VCENTER,
            Left => BS_LEFT | BS_VCENTER,
            Right => BS_RIGHT | BS_VCENTER,
            Top => BS_TOP | BS_CENTER,
            Bottom => BS_BOTTOM | BS_CENTER,
            TopLeft => BS_TOP | BS_LEFT,
            TopRight => BS_TOP | BS_RIGHT,
            BottomLeft => BS_BOTTOM | BS_LEFT,
            BottomRight => BS_BOTTOM | BS_RIGHT,
        } as u32)
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
        ms_style |= WINDOW_STYLE(match (self.btype, self.focused) {
            (Normal, false) => BS_PUSHBUTTON,
            (Normal, true) => BS_DEFPUSHBUTTON,
            (Split, false) => BS_SPLITBUTTON,
            (Split, true) => BS_DEFSPLITBUTTON,
            (Link, false) => BS_COMMANDLINK,
            (Link, true) => BS_DEFCOMMANDLINK,
        } as u32);
        if self.extra_msg {
            ms_style |= WINDOW_STYLE(BS_NOTIFY as u32);
        };
        if self.flat {
            ms_style |= WINDOW_STYLE(BS_FLAT as u32);
        };
        let (style2, ditype, text) = self.contect.into();
        (
            ms_style | style2 | self.pos.into() | WS_CHILD,
            ex,
            ditype,
            text,
        )
    }
}
pub type ButtonTemple = ButtonOption<ButtonTempleContent>;
impl DialogTempleControl for ButtonTemple {
    fn pre_compile(
        self,
        pos: Point,
        size: Size,
        identifier: WindowID,
    ) -> ControlPreCompilePruduct{
        let (mut ms_style, style_ex) = self.style.into();
        use ButtonType::*;
        ms_style |= WINDOW_STYLE(match (self.btype, self.focused) {
            (Normal, false) => BS_PUSHBUTTON,
            (Normal, true) => BS_DEFPUSHBUTTON,
            (Split, false) => BS_SPLITBUTTON,
            (Split, true) => BS_DEFSPLITBUTTON,
            (Link, false) => BS_COMMANDLINK,
            (Link, true) => BS_DEFCOMMANDLINK,
        } as u32);
        if self.extra_msg {
            ms_style |= WINDOW_STYLE(BS_NOTIFY as u32);
        };
        if self.flat {
            ms_style |= WINDOW_STYLE(BS_FLAT as u32);
        };
        let (style2, ct) = self.contect.into();
        ControlPreCompilePruduct::from(format!("CONTROL \"{}\", {}, \"Button\", 0x{:04X}, {}, {}, {}, {}, 0x{:04X}", 
            ct, 
            identifier, 
            (ms_style | style2 | self.pos.into() | WS_CHILD).0, 
            pos.x, 
            pos.y, 
            size.width, 
            size.height, 
            style_ex.0
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
        (if self.multiple_lines {
            WINDOW_STYLE((BS_MULTILINE | BS_TEXT) as u32)
        } else {
            WINDOW_STYLE(BS_TEXT as u32)
        },
        self.text,)
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
                    WINDOW_STYLE((BS_MULTILINE | BS_TEXT) as u32)
                } else {
                    WINDOW_STYLE(BS_TEXT as u32)
                },
                None,
                text,
            ),
            ButtonContent::IconOnly { icon, name} => (WINDOW_STYLE(if icon.is_left(){BS_BITMAP}else{ BS_ICON} as u32), Some(icon), name),
            ButtonContent::IconAndText {
                icon,
                text,
                multiple_lines,
            } => (
                if multiple_lines {
                    WINDOW_STYLE((BS_MULTILINE|if icon.is_left(){BS_BITMAP}else{ BS_ICON} | BS_TEXT) as u32)
                } else {
                    WINDOW_STYLE((if icon.is_left(){BS_BITMAP}else{ BS_ICON} | BS_TEXT)as u32)
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
        is_some_window(wnd, capdows::L!("Button"))
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
