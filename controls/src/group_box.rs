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
        if !is_button_window(wnd)? {
            return Ok(false);
        }
        Ok(
            style_of(wnd)
                .contains(WINDOW_STYLE(BS_GROUPBOX as u32)),
        )
    },
   {
       todo!()
    }
}
#[derive(Default)]
pub struct GroupBoxStyle(ChildWindowStyles);
impl Into<(WINDOW_STYLE, ChildWindowStyles)> for GroupBoxStyle {
    fn into(self) -> (WINDOW_STYLE, ChildWindowStyles) {
        (WINDOW_STYLE(BS_GROUPBOX as u32), self.0)
    }
}
impl CommonControl for GroupBox {
    type Style = GroupBoxStyle;
}
