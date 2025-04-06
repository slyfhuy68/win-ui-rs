use super::*;
pub struct GroupBox(HWND);
unsafe impl Send for GroupBox {}
unsafe impl Sync for GroupBox {}
pub struct GroupBoxMsg(pub usize, HWND);
impl Control for GroupBox {
    type MsgType = GroupBoxMsg;
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
        Ok(
            WINDOW_STYLE(unsafe { GetWindowLongW(*wnd, GWL_STYLE) as u32 })
                .contains(WINDOW_STYLE(BS_GROUPBOX as u32)),
        )
        // let style = unsafe { GetWindowLongW(*wnd, GWL_STYLE) };
        // if (style & BS_3STATE)==0 			&& (style & BS_AUTO3STATE)==0	 && (style & BS_AUTOCHECKBOX)==0	&&
        // 	(style & BS_AUTORADIOBUTTON)==0	&& (style & BS_DEFCOMMANDLINK)==0&& (style & BS_COMMANDLINK)==0		&&
        // 	(style & BS_SPLITBUTTON)==0 	&& (style & BS_DEFSPLITBUTTON)==0&& (style & BS_DEFPUSHBUTTON)==0 	&&
        // 	(style & BS_OWNERDRAW)==0		&& //(style & BS_GROUPBOX)==0  	 && //(style & BS_PUSHBUTTON)==0 	&&
        // 	(style & BS_RADIOBUTTON)==0 	&& (style & BS_CHECKBOX)==0
        // {
        //     return Ok(true);
        // }
        // println!("{:x}", style);
        // Ok(false)
    }
}
impl UnsafeControlMsg for GroupBoxMsg {
    type ControlType = GroupBox;
    unsafe fn from_msg(ptr: usize) -> Result<Self> {
        unsafe {
            let nmhdr = *(ptr as *mut NMHDR);
            let code = nmhdr.code;
            let w = nmhdr.hwndFrom.clone();
            let _ = nmhdr;
            let bmtype = match code {
                NM_CUSTOMDRAW => ptr,
                _ => return Err(Error::new(ERROR_INVALID_DATA.into(), "")),
            };
            Ok(Self(bmtype, w))
        }
    }
    fn get_control(&self) -> Self::ControlType {
        GroupBox(self.1)
    }
    unsafe fn into_raw(&mut self) -> Result<Either<u16, *mut NMHDR>> {
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
