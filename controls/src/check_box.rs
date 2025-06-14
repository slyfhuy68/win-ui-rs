use super::*;
use button::*;
pub struct CheckBoxStyle {
    pub extra_msg: bool,   //BS_NOTIFY
    pub auto: bool,        //if
    pub three_state: bool, //if
    pub flat: bool,        //BS_FLAT
    pub like_button: bool, //BS_PUSHLIKE
    pub left_text: bool,   //BS_LEFTTEXT
}
impl Into<WINDOW_STYLE> for CheckBoxStyle {
    fn into(self) -> WINDOW_STYLE {
        let mut ms_style = WINDOW_STYLE(0u32);
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

        ms_style
    }
}
impl Default for CheckBoxStyle {
    fn default() -> Self {
        Self {
            extra_msg: false,
            auto: true,
            three_state: false,
            flat: true,
            like_button: false,
            left_text: false,
        }
    }
}
pub use button::ButtonMsgType as CheckBoxMsgType;
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
        if !is_button_window(wnd)? {
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
pub struct CheckBoxDrawType(pub ButtonAutoDrawType, pub CheckBoxStyle, pub ChildWindowStyles);
impl Default for CheckBoxDrawType {
    fn default() -> Self {
        Self(ButtonAutoDrawType::TextOnly(false), Default::default(), Default::default())
    }
}
impl Into<(WINDOW_STYLE, Option<Either<Bitmap, Icon>>, ChildWindowStyles)> for CheckBoxDrawType {
    fn into(self) -> (WINDOW_STYLE, Option<Either<Bitmap, Icon>>, ChildWindowStyles) {
        let CheckBoxDrawType(dtype, bstyle, aaa) = self;
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
// impl From(WINDOW_STYLE, Option<BitmapOrIcon>) for CheckBoxDrawType {
// 	fn from(data: (WINDOW_STYLE, Option<BitmapOrIcon>)) -> Self {
//
// 	}
// }
#[derive(Debug)]
pub enum CheckBoxState {
    Checked,
    Indeterminate,
    UnChecked,
}
impl std::fmt::Display for CheckBoxState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CheckBoxState::Checked => write!(f, "Checked"),
            CheckBoxState::Indeterminate => write!(f, "Indeterminate"),
            CheckBoxState::UnChecked => write!(f, "UnChecked"),
        }
    }
}
pub use CheckBoxState::*;
impl ButtonControl for CheckBox{type Style = CheckBoxDrawType;}
impl CheckBox {

    pub fn is_checked(&self) -> Result<CheckBoxState> {
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
            BST_CHECKED => Ok(Checked),
            BST_UNCHECKED => Ok(UnChecked),
            BST_INDETERMINATE => Ok(Indeterminate),
            _ => return Err(Error::correct_error()),
        }
    }
}
