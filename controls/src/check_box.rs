use super::*;
#[doc(no_inline)]
pub use button::{
    BottonContentPos as CheckBoxContentPos, ButtonContent as CheckBoxContent,
    ButtonMsgType as CheckBoxMsgType, ButtonTempleContent as CheckBoxTempleContent,
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
    fn pre_compile(self, pos: FontPoint, size: FontSize, identifier: WindowID) -> String {
        let (mut ms_style, ex) = self.style.into();
        let (style2, ct) = self.contect.into();
        let poss: WINDOW_STYLE = self.pos.into();
        ms_style |= style2 | poss;
        set_style(&mut ms_style, BS_NOTIFY as WINDOW_STYLE, self.extra_msg);
        set_style(&mut ms_style, BS_FLAT as WINDOW_STYLE, self.flat);
        set_style(&mut ms_style, BS_PUSHLIKE as WINDOW_STYLE, self.like_button);
        set_style(&mut ms_style, BS_LEFTTEXT as WINDOW_STYLE, self.left_text);
        ms_style |= match (self.three_state, self.auto) {
            (true, true) => BS_AUTO3STATE,
            (true, false) => BS_3STATE,
            (false, true) => BS_AUTOCHECKBOX,
            (false, false) => BS_CHECKBOX,
        } as WINDOW_STYLE;
        format!(
            "CONTROL \"{}\", {}, \"Button\", 0x{:04X}, {}, {}, {}, {}, 0x{:04X}",
            ct, identifier, ms_style, pos.x, pos.y, size.width, size.height, ex
        )
    }
}
impl<T> CheckBoxOption<T> {
    #[inline]
    pub const fn three_state(mut self) -> Self {
        self.three_state = true;
        self
    }
}
impl CheckBoxStyle {
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
impl CheckBoxTemple {
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
impl From<CheckBoxStyle> for ((WINDOW_STYLE, WINDOW_EX_STYLE), Option<ButtonImage>, String) {
    fn from(val: CheckBoxStyle) -> Self {
        let (mut ms_style, ex) = val.style.into();
        let pos: WINDOW_STYLE = val.pos.into();
        let (style2, ditype, text) = val.contect.into();
        ms_style |= style2 | pos;
        set_style(&mut ms_style, BS_NOTIFY as WINDOW_STYLE, val.extra_msg);
        set_style(&mut ms_style, BS_FLAT as WINDOW_STYLE, val.flat);
        if val.three_state {
            if val.auto {
                ms_style |= BS_AUTO3STATE as WINDOW_STYLE;
            } else {
                ms_style |= BS_3STATE as WINDOW_STYLE;
            };
        } else if val.auto {
            ms_style |= BS_AUTOCHECKBOX as WINDOW_STYLE;
        } else {
            ms_style |= BS_CHECKBOX as WINDOW_STYLE;
        };
        set_style(&mut ms_style, BS_PUSHLIKE as WINDOW_STYLE, val.like_button);
        set_style(&mut ms_style, BS_LEFTTEXT as WINDOW_STYLE, val.left_text);

        ((ms_style, ex), ditype, text)
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
    #[inline]
    fn new_raw(
        wnd: &mut Window,
        pos: Option<Rect>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<HWND> {
        let (style, draw, name) = control_style.into();
        new_button(wnd, name, pos, identifier, style, font, draw)
    }
}
impl CheckBox {
    pub fn is_checked(&self) -> Result<CheckBoxState> {
        match unsafe {
            SendMessageW(self.0.handle(), BM_GETCHECK, 0 as WPARAM, 0 as LPARAM)
                as DLG_BUTTON_CHECK_STATE
        } {
            BST_CHECKED => Ok(Checked),
            BST_UNCHECKED => Ok(UnChecked),
            BST_INDETERMINATE => Ok(Indeterminate),
            _ => Err(ERROR_NOT_SUPPORTED),
        }
    }
}
impl TextControl for CheckBox {}
