use super::button::*;
use super::*;
#[derive(Clone)]
pub struct CheckBox(HWND); //PUSHBUTTON
unsafe impl Send for CheckBox {}
unsafe impl Sync for CheckBox {}
pub struct CheckBoxStyle {
    pub extra_msg: bool,   //BS_NOTIFY
    pub auto: bool,        //if
    pub three_state: bool, // if
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
pub struct CheckBoxMsg {
    hwnd: HWND,
    pub bm_type: ButtonMsgType,
}
impl Control for CheckBox {
    type MsgType = CheckBoxMsg;
    fn from_window(wnd: Window) -> Result<Box<Self>> {
        unsafe {
            if Self::is_self(&wnd.handle)? {
                Ok(Box::new(Self(wnd.handle)))
            } else {
                Err(Error::new(ERROR_INVALID_WINDOW_HANDLE.into(), ""))
            }
        }
    }
    fn to_window(self) -> Window {
        Window { handle: self.0 }
    }
    unsafe fn force_from_window(wnd: Window) -> Self {
        Self(wnd.handle)
    }
    unsafe fn is_self(wnd: &HWND) -> Result<bool> {
        if !is_button_window(wnd)? {
            return Ok(false);
        }
        let style = unsafe { GetWindowLongW(*wnd, GWL_STYLE) };
        if (style & BS_CHECKBOX) != 0 || (style & BS_AUTOCHECKBOX) != 0 {
            return Ok(true);
        }
        if (style & BS_3STATE) != 0 || (style & BS_AUTO3STATE) != 0 {
            return Ok(true);
        }
        Ok(false)
    }
}
impl ControlMsg for CheckBoxMsg {
    type ControlType = CheckBox;
    unsafe fn from_msg(ptr: usize) -> Option<Box<Self>> {
        unsafe {
            let nmhdr = *(ptr as *mut NMHDR);
            let code = nmhdr.code;
            let w = nmhdr.hwndFrom.clone();
            let _ = nmhdr;
            use ButtonMsgType::*;
            let bmtype = match code {
                BCN_HOTITEMCHANGE => {
                    let data = *(ptr as *mut NMBCHOTITEM);
                    if data.dwFlags == HICF_MOUSE | HICF_ENTERING {
                        MouseEntering
                    } else if data.dwFlags == HICF_MOUSE | HICF_LEAVING {
                        MouseLaveing
                    } else {
                        return None;
                    }
                }
                BN_CLICKED => Clicked,
                BN_DBLCLK => DoubleClicked,
                BN_KILLFOCUS => LoseKeyboardFocus,
                BN_SETFOCUS => GetKeyboardFocus,
                NM_CUSTOMDRAW => Draw(ptr),
                _ => return None,
            };
            Some(Box::new(Self {
                hwnd: w,
                bm_type: bmtype,
            }))
        }
    }
    fn get_control(&self) -> Self::ControlType {
        CheckBox(self.hwnd)
    }
}
pub struct CheckBoxDrawType(pub ButtonAutoDrawType, pub CheckBoxStyle);
impl Default for CheckBoxDrawType {
    fn default() -> Self {
        Self(ButtonAutoDrawType::TextOnly(false), Default::default())
    }
}
impl Into<(WINDOW_STYLE, Option<Either<Bitmap, Icon>>)> for CheckBoxDrawType {
    fn into(self) -> (WINDOW_STYLE, Option<Either<Bitmap, Icon>>) {
        let CheckBoxDrawType(dtype, bstyle) = self;
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
impl CheckBox {
    // fn is_check_box(&self) -> bool {
    // 	self.is_sure_check_box() | self.is_3state()
    // }
    // fn is_sure_check_box(&self) -> bool {
    //     let style = WINDOW_STYLE(unsafe {
    //         GetWindowLongW(self.0, GWL_STYLE) as u32
    //     });
    //     style.contains(WINDOW_STYLE(BS_AUTOCHECKBOX as u32)) | style.contains(WINDOW_STYLE(BS_CHECKBOX as u32))
    // }
    // fn is_3state(&self) -> bool {
    //     let style = WINDOW_STYLE(unsafe {
    //         GetWindowLongW(self.0, GWL_STYLE) as u32
    //     });
    //     style.contains(WINDOW_STYLE(BS_AUTO3STATE as u32)) | style.contains(WINDOW_STYLE(BS_3STATE as u32))
    // }
    pub fn new(
        wnd: &mut Window,
        name: &str,
        pos: Option<Rectangle>,
        identifier: WindowID,
        control_style: CheckBoxDrawType,
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
        Ok(CheckBox(hwnd))
    }
    pub fn is_checked(&self) -> Result<CheckBoxState> {
        if !unsafe { Self::is_self(&self.0) }? {
            return Err(Error::new(ERROR_NOT_SUPPORTED.to_hresult(), ""));
        }
        let result =
            unsafe { SendMessageW(self.0, BM_GETCHECK, Some(WPARAM(0)), Some(LPARAM(0))).0 };
        match DLG_BUTTON_CHECK_STATE(match result.try_into() {
            Ok(x) => x,
            Err(_) => return Err(Error::new(ERROR_NOT_SUPPORTED.to_hresult(), "")),
        }) {
            BST_CHECKED => Ok(Checked),
            BST_UNCHECKED => Ok(UnChecked),
            BST_INDETERMINATE => Ok(Indeterminate),
            _ => return Err(Error::from_win32()),
        }
    }
}
