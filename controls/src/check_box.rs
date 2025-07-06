use super::*;
pub use button::{
    BottonContentPos as CheckBoxContentPos, ButtonContent as CheckBoxContent,
    ButtonMsgType as CheckBoxMsgType, ButtonTempleContent as CheckBoxTempleContent
};
pub struct CheckBoxOption<T> {
    pub style: ChildWindowStyles,
    pub contect: T,
    pub pos: CheckBoxContentPos,
    pub extra_msg: bool,   //BS_NOTIFY
    pub auto: bool,        //if
    pub three_state: bool, //if
    pub flat: bool,        //BS_FLAT
    pub like_button: bool, //BS_PUSHLIKE
    pub left_text: bool,   //BS_LEFTTEXT
}
pub type CheckBoxStyle = CheckBoxOption<CheckBoxContent>;
pub type CheckBoxTemple = CheckBoxOption<CheckBoxTempleContent>;
impl DialogTempleControl for CheckBoxTemple {
    fn pre_compile(
        self,
        pos: Point,
        size: Size,
        identifier: WindowID,
    ) -> ControlPreCompilePruduct{
        let (mut ms_style, ex) = self.style.into();
        let (style2, ct) = self.contect.into();
        ms_style |= style2 | self.pos.into() | WS_CHILD;
        if self.extra_msg {
            ms_style |= WINDOW_STYLE(BS_NOTIFY as u32);
        };
        if self.flat {
            ms_style |= WINDOW_STYLE(BS_FLAT as u32);
        };
        if self.three_state {
            if self.auto {
                ms_style |= WINDOW_STYLE(BS_AUTO3STATE as u32);
            } else {
                ms_style |= WINDOW_STYLE(BS_3STATE as u32);
            };
        } else {
            if self.auto {
                ms_style |= WINDOW_STYLE(BS_AUTOCHECKBOX as u32);
            } else {
                ms_style |= WINDOW_STYLE(BS_CHECKBOX as u32);
            };
        };
        if self.like_button {
            ms_style |= WINDOW_STYLE(BS_PUSHLIKE as u32);
        };
        if self.left_text {
            ms_style |= WINDOW_STYLE(BS_LEFTTEXT as u32);
        };
        ControlPreCompilePruduct::from(format!("CONTROL \"{}\", {}, \"Button\", 0x{:04X}, {}, {}, {}, {}, 0x{:04X}", 
            ct, 
            identifier, 
            (ms_style | style2 | self.pos.into() | WS_CHILD).0, 
            pos.x, 
            pos.y, 
            size.width, 
            size.height, 
            ex.0
        ))
    }
}
impl<T> CheckBoxOption<T> {
    #[inline]
    pub const fn three_state(mut self) -> Self {
        self.three_state = true;
        self
    }
}
impl CheckBoxStyle{
    #[inline]
    pub fn new_text(text: &str) -> Self {
        Self {
            style: Default::default(),
            contect: CheckBoxContent::new_text(text),
            pos: Default::default(),
            extra_msg: false,
            auto: true,
            three_state: false,
            flat: false,
            like_button: false,
            left_text: false,
        }
    }
}
impl CheckBoxTemple{
    #[inline]
    pub fn new(text: &str) -> Self {
        Self {
            style: Default::default(),
            contect: CheckBoxTempleContent::new(text),
            pos: Default::default(),
            extra_msg: false,
            auto: true,
            three_state: false,
            flat: false,
            like_button: false,
            left_text: false,
        }
    }
}
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE, Option<ButtonImage>, String)> for CheckBoxStyle {
    fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE, Option<ButtonImage>, String) {
        let (mut ms_style, ex) = self.style.into();
        let (style2, ditype, text) = self.contect.into();
        ms_style |= style2 | self.pos.into() | WS_CHILD;
        if self.extra_msg {
            ms_style |= WINDOW_STYLE(BS_NOTIFY as u32);
        };
        if self.flat {
            ms_style |= WINDOW_STYLE(BS_FLAT as u32);
        };
        if self.three_state {
            if self.auto {
                ms_style |= WINDOW_STYLE(BS_AUTO3STATE as u32);
            } else {
                ms_style |= WINDOW_STYLE(BS_3STATE as u32);
            };
        } else {
            if self.auto {
                ms_style |= WINDOW_STYLE(BS_AUTOCHECKBOX as u32);
            } else {
                ms_style |= WINDOW_STYLE(BS_CHECKBOX as u32);
            };
        };
        if self.like_button {
            ms_style |= WINDOW_STYLE(BS_PUSHLIKE as u32);
        };
        if self.left_text {
            ms_style |= WINDOW_STYLE(BS_LEFTTEXT as u32);
        };

        (ms_style, ex, ditype, text)
    }
}
define_control! {
    CheckBox,
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
        if !is_some_window(wnd, L!("Button"))? {
            return Ok(false);
        }
        let style = style_of_raw(wnd);
        if (style & BS_CHECKBOX) != 0 || (style & BS_AUTOCHECKBOX) != 0 {
            return Ok(true);
        }
        if (style & BS_3STATE) != 0 || (style & BS_AUTO3STATE) != 0 {
            return Ok(true);
        }
        Ok(false)
    },
    {
        todo!()
    }
}
#[derive(Debug, Copy, Clone)]
pub enum CheckBoxState {
    Checked,
    Indeterminate,
    UnChecked,
}
pub use CheckBoxState::*;
impl std::fmt::Display for CheckBoxState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CheckBoxState::Checked => write!(f, "Checked"),
            CheckBoxState::Indeterminate => write!(f, "Indeterminate"),
            CheckBoxState::UnChecked => write!(f, "UnChecked"),
        }
    }
}
impl CommonControl for CheckBox {
    type Style = CheckBoxStyle;
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
impl CheckBox {
    pub fn is_checked(&self) -> Result<CheckBoxState> {
        match DLG_BUTTON_CHECK_STATE(unsafe {
            SendMessageW(self.0.handle(), BM_GETCHECK, None, None).0 as u32
        }) {
            BST_CHECKED => Ok(Checked),
            BST_UNCHECKED => Ok(UnChecked),
            BST_INDETERMINATE => Ok(Indeterminate),
            _ => return Err(ERROR_NOT_SUPPORTED),
        }
    }
}
