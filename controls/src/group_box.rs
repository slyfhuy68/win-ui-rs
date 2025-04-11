use super::*;
// pub struct GroupBox(HWND);
// unsafe impl Send for GroupBox {}
// unsafe impl Sync for GroupBox {}
pub enum GroupBoxMsgType {
    Draw(usize),
}
// pub struct GroupBoxMsg(pub usize, HWND);
define_control! {
    GroupBox,
    "Button",
    {
        match code {
            NM_CUSTOMDRAW => Draw(ptr),
            _ => return Err(Error::new(ERROR_INVALID_DATA.into(), "")),
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
impl GroupBox {
    pub fn new(
        wnd: &mut Window,
        name: &str,
        pos: Option<Rectangle>,
        identifier: WindowID,
        style: ChildWindowStyles,
        style_ex: NormalWindowExStyles,
        font: bool,
        parent_draw: bool,
    ) -> Result<Self> {
        let control_style_ms = if parent_draw {
            WINDOW_STYLE(BS_OWNERDRAW as u32)
        } else {
            WINDOW_STYLE(0)
        } | WINDOW_STYLE(BS_GROUPBOX as u32);
        let hwnd = new_button(
            wnd,
            name,
            pos,
            identifier,
            style,
            style_ex,
            control_style_ms,
            font,
            !parent_draw,
            None,
        )?;
        Ok(GroupBox(hwnd))
    }
}
