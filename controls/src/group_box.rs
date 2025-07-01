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
        if !is_some_window(wnd, "Button")? {
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
pub struct GroupBoxStyle(ChildWindowStyles);
impl Into<(WINDOW_STYLE, ChildWindowStyles)> for GroupBoxStyle {
    fn into(self) -> (WINDOW_STYLE, ChildWindowStyles) {
        (WINDOW_STYLE(BS_GROUPBOX as u32), self.0)
    }
}
impl CommonControl for GroupBox {
    type Style = GroupBoxStyle;
}
