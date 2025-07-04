use super::*;
use button::*;
pub struct RadioButtonStyle {
    pub extra_msg: bool,   //BS_NOTIFY
    pub auto: bool,        //if light BS_AUTORADIOBUTTON else BS_RADIOBUTTON
    pub flat: bool,        //BS_FLAT
    pub like_button: bool, //BS_PUSHLIKE
    pub left_text: bool,   //BS_LEFTTEXT
}
impl Into<WINDOW_STYLE> for RadioButtonStyle {
    fn into(self) -> WINDOW_STYLE {
        let mut ms_style = WINDOW_STYLE(0u32);
        if self.extra_msg {
            ms_style |= WINDOW_STYLE(BS_NOTIFY as u32);
        };
        if self.flat {
            ms_style |= WINDOW_STYLE(BS_FLAT as u32);
        };
        if self.auto {
            ms_style |= WINDOW_STYLE(BS_AUTORADIOBUTTON as u32);
        } else {
            ms_style |= WINDOW_STYLE(BS_RADIOBUTTON as u32);
        };
        if self.like_button {
            ms_style |= WINDOW_STYLE(BS_PUSHLIKE as u32);
        };
        if self.left_text {
            ms_style |= WINDOW_STYLE(BS_LEFTTEXT as u32);
        };

        ms_style
    }
}
impl Default for RadioButtonStyle {
    fn default() -> Self {
        Self {
            extra_msg: false,
            auto: true,
            flat: true,
            like_button: false,
            left_text: false,
        }
    }
}
pub use button::ButtonMsgType as RadioButtonMsgType;
define_control! {
    RadioButton,
    "Button",
    {match code {
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
    }},
    {
        if !is_some_window(wnd, "Button")? {
            return Ok(false);
        }
        let style = style_of_raw(wnd);
        if (style & BS_RADIOBUTTON) != 0 || (style & BS_AUTORADIOBUTTON) != 0 {
            return Ok(true);
        }
        Ok(false)
    },
    {
        todo!()
    }
}
pub struct RadioButtonDrawType(
    pub ButtonContent,
    pub RadioButtonStyle,
    pub ChildWindowStyles,
);
impl Default for RadioButtonDrawType {
    fn default() -> Self {
        Self(
            ButtonContent::TextOnly(false),
            Default::default(),
            Default::default(),
        )
    }
}
impl RadioButtonDrawType {
    pub fn group_leader() -> Self {
        Self(
            ButtonContent::TextOnly(false),
            Default::default(),
            ChildWindowStyles {
                group_leader: true,
                ..Default::default()
            },
        )
    }
}
impl Into<(WINDOW_STYLE, Option<ButtonImage>, ChildWindowStyles)> for RadioButtonDrawType {
    fn into(self) -> (WINDOW_STYLE, Option<ButtonImage>, ChildWindowStyles) {
        let RadioButtonDrawType(dtype, bstyle, bbb) = self;
        let mut wstyle = WINDOW_STYLE(0);
        let ditype = match dtype {
            ButtonContent::IconOnly(boi) => Some(boi),
            ButtonContent::TextOnly(a) => {
                if a {
                    wstyle |= WINDOW_STYLE(BS_MULTILINE as u32);
                };
                None
            }
            ButtonContent::IconAndText(boi, a) => {
                if a {
                    wstyle |= WINDOW_STYLE(BS_MULTILINE as u32)
                };
                Some(boi)
            }
        };
        wstyle |= bstyle.into();
        (wstyle, ditype, bbb)
    }
}
// impl From(WINDOW_STYLE, Option<BitmapOrIcon>) for RadioButtonDrawType {
// 	fn from(data: (WINDOW_STYLE, Option<BitmapOrIcon>)) -> Self {
//
// 	}
// }
impl ButtonControl for RadioButton {
    type Style = RadioButtonDrawType;
}
impl RadioButton {
    pub fn is_checked(&self) -> Result<bool> {
        let result = unsafe {
            SendMessageW(
                self.0.handle(),
                BM_GETCHECK,
                Some(WPARAM(0)),
                Some(LPARAM(0)),
            )
            .0
        };
        match DLG_BUTTON_CHECK_STATE(match result.try_into() {
            Ok(x) => x,
            Err(_) => return Err(ERROR_NOT_SUPPORTED),
        }) {
            BST_CHECKED => Ok(true),
            BST_UNCHECKED => Ok(false),
            BST_INDETERMINATE => Err(ERROR_NOT_SUPPORTED),
            _ => return Err(Error::correct_error()),
        }
    }
}
