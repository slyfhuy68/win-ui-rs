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
        ms_style |= style2 | self.pos.into();
        set_style(&mut ms_style, BS_NOTIFY as WINDOW_STYLE, self.extra_msg);
        set_style(&mut ms_style, BS_FLAT as WINDOW_STYLE, self.flat);
        if self.auto {
            ms_style |= BS_AUTORADIOBUTTON as WINDOW_STYLE;
        } else {
            ms_style |= BS_RADIOBUTTON as WINDOW_STYLE;
        };
        set_style(&mut ms_style, BS_PUSHLIKE as WINDOW_STYLE, self.like_button);
        set_style(&mut ms_style, BS_LEFTTEXT as WINDOW_STYLE, self.left_text);

        (ms_style, ex, ditype, text)
    }
}
pub type RadioBoxTemple = RadioBoxOption<RadioBoxTempleContent>;
impl DialogTempleControl for RadioBoxTemple {
    fn pre_compile(self, pos: Point, size: Size, identifier: WindowID) -> ControlPreCompilePruduct {
        let (mut ms_style, ex) = self.style.into();
        let (style2, ct) = self.contect.into();
        ms_style |= style2 | self.pos.into();
        set_style(&mut ms_style, BS_NOTIFY as WINDOW_STYLE, self.extra_msg);
        set_style(&mut ms_style, BS_FLAT as WINDOW_STYLE, self.flat);
        if self.auto {
            ms_style |= BS_AUTORADIOBUTTON as WINDOW_STYLE;
        } else {
            ms_style |= BS_RADIOBUTTON as WINDOW_STYLE;
        };
        set_style(&mut ms_style, BS_PUSHLIKE as WINDOW_STYLE, self.like_button);
        set_style(&mut ms_style, BS_LEFTTEXT as WINDOW_STYLE, self.left_text);
        ControlPreCompilePruduct::from(format!(
            "CONTROL \"{}\", {}, \"Button\", 0x{:04X}, {}, {}, {}, {}, 0x{:04X}",
            ct,
            identifier,
            (ms_style | style2 | self.pos.into()).0,
            pos.x,
            pos.y,
            size.width,
            size.height,
            ex.0
        ))
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
impl RadioBox {
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
