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
pub type GroupBoxTemple = GroupBoxStyle;
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE, String)> for GroupBoxStyle {
    fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE, String) {
        let (a, b) = self.style.into();
        (a | WINDOW_STYLE(BS_GROUPBOX as u32), b, self.text)
    }
}
impl DialogTempleControl for GroupBoxTemple {
    fn pre_compile(self, pos: Point, size: Size, identifier: WindowID) -> ControlPreCompilePruduct {
        let (ms_style, ex, name) = control_style.into();
        ControlPreCompilePruduct::from(format!(
            "CONTROL \"{}\", {}, \"Button\", 0x{:04X}, {}, {}, {}, {}, 0x{:04X}",
            ct, identifier, ms_style.0, pos.x, pos.y, size.width, size.height, ex.0
        ))
    }
}
impl CommonControl for GroupBox {
    type Style = GroupBoxStyle;
    fn new(
        wnd: &mut Window,
        pos: Option<Rect>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<Self> {
        let (style, ex, name) = control_style.into();
        Ok(Self(new_button(
            wnd, name, pos, identifier, style, ex, font, None,
        )?))
    }
}
