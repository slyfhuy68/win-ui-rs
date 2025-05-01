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
        if !is_button_window(wnd)? {
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
pub struct RadioButtonDrawType(pub ButtonAutoDrawType, pub RadioButtonStyle);
impl Default for RadioButtonDrawType {
    fn default() -> Self {
        Self(ButtonAutoDrawType::TextOnly(false), Default::default())
    }
}
impl Into<(WINDOW_STYLE, Option<Either<Bitmap, Icon>>)> for RadioButtonDrawType {
    fn into(self) -> (WINDOW_STYLE, Option<Either<Bitmap, Icon>>) {
        let RadioButtonDrawType(dtype, bstyle) = self;
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
        (wstyle, ditype)
    }
}
// impl From(WINDOW_STYLE, Option<BitmapOrIcon>) for RadioButtonDrawType {
// 	fn from(data: (WINDOW_STYLE, Option<BitmapOrIcon>)) -> Self {
//
// 	}
// }
impl RadioButton {
    pub fn new(
        wnd: &mut Window,
        name: &str,
        pos: Option<Rectangle>,
        identifier: WindowID,
        control_style: RadioButtonDrawType,
        style: ChildWindowStyles,
        style_ex: NormalWindowExStyles,
        font: bool,
        no_notify: bool,
    ) -> Result<Self> {
        let (control_style_ms, draw) = control_style.into();
        let hwnd = new_button(
            wnd,
            name,
            pos,
            identifier,
            style,
            style_ex,
            control_style_ms,
            font,
            no_notify,
            draw,
        )?;
        Ok(RadioButton(hwnd))
    }
    pub fn is_checked(&self) -> Result<bool> {
        if !Self::is_self(&self.0)? {
            return Err(ERROR_NOT_SUPPORTED);
        }
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
