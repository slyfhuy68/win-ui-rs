use super::*;
pub enum GroupBoxMsgType {
    Draw(usize),
}
define_control! {
    GroupBox,
    "Button",
    {
        match code {
            NM_CUSTOMDRAW => Draw(ptr),
            _ => return Err(ERROR_MSG_CODE_NOT_SUPPORT),
        }
    },
    {
        if !is_some_window(wnd, L!("Button"))? {
            return Ok(false);
        }
        Ok(
            (style_of_raw(wnd) & BS_GROUPBOX )!= 0
        )
    },
   {
       todo!()
    }
}
#[derive(Default)]
pub struct GroupBoxStyle {
    pub style: ChildWindowStyles,
    pub text: String,
}
impl GroupBoxStyle {
    pub fn new(text: &str) -> Self {
        GroupBoxStyle {
            style: ChildWindowStyles::default(),
            text: text.to_string(),
        }
    }
}
pub type GroupBoxTemple = GroupBoxStyle;
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE, String)> for GroupBoxStyle {
    fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE, String) {
        let (a, b) = self.style.into();
        (a | (BS_GROUPBOX as u32), b, self.text)
    }
}
impl DialogTempleControl for GroupBoxTemple {
    fn pre_compile(self, pos: FontPoint, size: FontSize, identifier: WindowID) -> String {
        let (ms_style, ex, ct) = self.into();
        format!(
            "CONTROL \"{}\", {}, \"Button\", 0x{:04X}, {}, {}, {}, {}, 0x{:04X}",
            ct, identifier, ms_style, pos.x, pos.y, size.width, size.height, ex
        )
    }
}
impl CommonControl for GroupBox {
    type Style = GroupBoxStyle;
    #[inline]
    fn new_raw(
        wnd: &mut Window,
        pos: Option<Rect>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<HWND> {
        let (style, ex, name) = control_style.into();
        new_button(wnd, name, pos, identifier, style, ex, font, None)
    }
}
impl TextControl for GroupBox {}
