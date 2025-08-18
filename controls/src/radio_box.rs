use super::*;
#[doc(no_inline)]
pub use button::{
    BottonContentPos as RadioBoxContentPos, ButtonContent as RadioBoxContent,
    ButtonMsgType as RadioBoxMsgType, ButtonTempleContent as RadioBoxTempleContent,
};
pub struct RadioBoxOption<T> {
    pub style: ChildWindowStyles,
    pub contect: T,
    pub pos: RadioBoxContentPos,
    pub extra_msg: bool,   //BS_NOTIFY
    pub auto: bool,        //if light BS_AUTORADIOBUTTON else BS_RADIOBUTTON
    pub flat: bool,        //BS_FLAT
    pub like_button: bool, //BS_PUSHLIKE
    pub left_text: bool,   //BS_LEFTTEXT
}
pub type RadioBoxStyle = RadioBoxOption<RadioBoxContent>;
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE, Option<ButtonImage>, String)> for RadioBoxStyle {
    fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE, Option<ButtonImage>, String) {
        let (mut ms_style, ex) = self.style.into();
        let (style2, ditype, text) = self.contect.into();
        let pos: WINDOW_STYLE = self.pos.into();
        ms_style |= style2 | pos;
        set_style(&mut ms_style, BS_NOTIFY as WINDOW_STYLE, self.extra_msg);
        set_style(&mut ms_style, BS_FLAT as WINDOW_STYLE, self.flat);
        set_style(&mut ms_style, BS_PUSHLIKE as WINDOW_STYLE, self.like_button);
        set_style(&mut ms_style, BS_LEFTTEXT as WINDOW_STYLE, self.left_text);
        if self.auto {
            ms_style |= BS_AUTORADIOBUTTON as WINDOW_STYLE;
        } else {
            ms_style |= BS_RADIOBUTTON as WINDOW_STYLE;
        };
        (ms_style, ex, ditype, text)
    }
}
pub type RadioBoxTemple = RadioBoxOption<RadioBoxTempleContent>;
impl DialogTempleControl for RadioBoxTemple {
    fn pre_compile(self, pos: FontPoint, size: FontSize, identifier: WindowID) -> String {
        let (mut ms_style, ex) = self.style.into();
        let (style2, ct) = self.contect.into();
        let poss: WINDOW_STYLE = self.pos.into();
        ms_style |= style2 | poss;
        set_style(&mut ms_style, BS_NOTIFY as WINDOW_STYLE, self.extra_msg);
        set_style(&mut ms_style, BS_FLAT as WINDOW_STYLE, self.flat);
        set_style(&mut ms_style, BS_PUSHLIKE as WINDOW_STYLE, self.like_button);
        set_style(&mut ms_style, BS_LEFTTEXT as WINDOW_STYLE, self.left_text);
        if self.auto {
            ms_style |= BS_AUTORADIOBUTTON as WINDOW_STYLE;
        } else {
            ms_style |= BS_RADIOBUTTON as WINDOW_STYLE;
        };
        format!(
            "CONTROL \"{}\", {}, \"Button\", 0x{:04X}, {}, {}, {}, {}, 0x{:04X}",
            ct, identifier, ms_style, pos.x, pos.y, size.width, size.height, ex
        )
    }
}
define_control! {
    RadioBox,
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
        if !is_some_window(wnd, L!("Button"))? {
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
impl<T> RadioBoxOption<T> {
    #[inline]
    pub fn group_leader(mut self) -> Self {
        self.style.group_leader = true;
        self
    }
}
impl RadioBoxStyle {
    #[inline]
    pub fn new_text(text: &str) -> Self {
        Self {
            style: Default::default(),
            contect: RadioBoxContent::new_text(text),
            pos: Default::default(),
            extra_msg: false,
            auto: true,
            flat: false,
            like_button: false,
            left_text: false,
        }
    }
}
impl RadioBoxTemple {
    #[inline]
    pub fn new(text: &str) -> Self {
        Self {
            style: Default::default(),
            contect: RadioBoxTempleContent::new(text),
            pos: Default::default(),
            extra_msg: false,
            auto: true,
            flat: false,
            like_button: false,
            left_text: false,
        }
    }
}
impl CommonControl for RadioBox {
    type Style = RadioBoxStyle;
    #[inline]
    fn new_raw(
        wnd: &mut Window,
        pos: Option<Rect>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<HWND> {
        let (style, ex, draw, name) = control_style.into();
        new_button(wnd, name, pos, identifier, style, ex, font, draw)
    }
}
impl RadioBox {
    pub fn is_checked(&self) -> Result<bool> {
        let result = error_from_win32_zero_num!(SendMessageW(
            self.0.handle(),
            BM_GETCHECK,
            0 as WPARAM,
            0 as LPARAM
        ) as DLG_BUTTON_CHECK_STATE)?;
        match result {
            BST_CHECKED => Ok(true),
            BST_UNCHECKED => Ok(false),
            _ => Err(ERROR_NOT_SUPPORTED),
        }
    }
}
impl TextControl for RadioBox {}
