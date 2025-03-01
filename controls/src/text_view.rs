use super::*;
pub struct TextView(HWND); //PUSH1234567890
unsafe impl Send for TextView {}
unsafe impl Sync for TextView {}
//------------------------------------------------------------AI生成
const SS_ETCHEDHORZ: i32 = 0x00000010;
const SS_ETCHEDVERT: i32 = 0x00000020;
const SS_BLACKFRAME: i32 = 0x00000040;
const SS_BLACKRECT: i32 = 0x00000080;
const SS_GRAYFRAME: i32 = 0x00000100;
const SS_GRAYRECT: i32 = 0x00000200;
const SS_NOTIFY: i32 = 0x00000400; // 示例值，请根据实际API文档调整
const SS_WHITEFRAME: i32 = 0x00000400;
const SS_WHITERECT: i32 = 0x00000800;
const SS_SUNKEN: i32 = 0x00001000;
const SS_CENTER: i32 = 0x00000004;
const SS_EDITCONTROL: i32 = 0x00002000;
const SS_NOPREFIX: i32 = 0x00000800; // 示例值，请根据实际API文档调整
const SS_SIMPLE: i32 = 0x00000001;
const SS_LEFTNOWORDWRAP: i32 = 0x00000002;
const SS_ENDELLIPSIS: i32 = 0x00000040;
const SS_PATHELLIPSIS: i32 = 0x00000080;
const SS_WORDELLIPSIS: i32 = 0x00000100;
const SS_RIGHT: i32 = 0x00000002;
//-------------------------------------------------------
pub enum EllipsisType {
    Smiple,       //SS_SIMPLE
    NoEllipsis,   //SS_LEFTNOWORDWRAP
    EndEllipsis,  //SS_ENDELLIPSIS
    PathEllipsis, //SS_PATHELLIPSIS
    WordEllipsis, //SS_WORDELLIPSIS
}
#[derive(Default)]
pub enum TextPos {
    Center, //SS_CENTER
    #[default]
    Left, //SS_LEFT 默认
    Right,  //SS_RIGHT
}
#[derive(Default)]
pub struct TextViewStyle {
    pub etched_horz: bool, //SS_ETCHEDHORZ
    pub etched_vert: bool, //SS_ETCHEDVERT
    pub black_frame: bool, //SS_BLACKFRAME
    pub black_rect: bool,  //SS_BLACKRECT
    pub gray_fame: bool,   //SS_GRAYFRAME
    pub gray_rect: bool,   //SS_GRAYRECT
    pub white_fame: bool,  //SS_WHITEFRAME
    pub white_rect: bool,  //SS_WHITERECT
    pub sun_ken: bool,     //SS_SUNKEN
    pub text_pos: TextPos,
    pub like_edit: bool,                   //SS_EDITCONTROL
    pub single_line: Option<EllipsisType>, //None => NULL 默认为None
    pub prefix: bool,                      // !SS_NOPREFIX 记得取反
}
impl Into<WINDOW_STYLE> for TextViewStyle {
    //此代码使用AI生成
    fn into(self) -> WINDOW_STYLE {
        let mut style = WINDOW_STYLE(0);

        if self.etched_horz {
            style |= WINDOW_STYLE(SS_ETCHEDHORZ as u32);
        };
        if self.etched_vert {
            style |= WINDOW_STYLE(SS_ETCHEDVERT as u32);
        };
        if self.black_frame {
            style |= WINDOW_STYLE(SS_BLACKFRAME as u32);
        };
        if self.black_rect {
            style |= WINDOW_STYLE(SS_BLACKRECT as u32);
        };
        if self.gray_fame {
            style |= WINDOW_STYLE(SS_GRAYFRAME as u32);
        };
        if self.gray_rect {
            style |= WINDOW_STYLE(SS_GRAYRECT as u32);
        };
        if self.white_fame {
            style |= WINDOW_STYLE(SS_WHITEFRAME as u32);
        };
        if self.white_rect {
            style |= WINDOW_STYLE(SS_WHITERECT as u32);
        };
        if self.sun_ken {
            style |= WINDOW_STYLE(SS_SUNKEN as u32);
        };
        if self.like_edit {
            style |= WINDOW_STYLE(SS_EDITCONTROL as u32);
        };

        match self.text_pos {
            TextPos::Center => style |= WINDOW_STYLE(SS_CENTER as u32),
            TextPos::Left => {} // 默认值，无需设置
            TextPos::Right => style |= WINDOW_STYLE(SS_RIGHT as u32),
        };

        if let Some(single_line) = self.single_line {
            match single_line {
                EllipsisType::Smiple => style |= WINDOW_STYLE(SS_SIMPLE as u32),
                EllipsisType::NoEllipsis => style |= WINDOW_STYLE(SS_LEFTNOWORDWRAP as u32),
                EllipsisType::EndEllipsis => style |= WINDOW_STYLE(SS_ENDELLIPSIS as u32),
                EllipsisType::PathEllipsis => style |= WINDOW_STYLE(SS_PATHELLIPSIS as u32),
                EllipsisType::WordEllipsis => style |= WINDOW_STYLE(SS_WORDELLIPSIS as u32),
            }
        };

        if !self.prefix {
            //根据你的描述调整了 prefix 字段的处理逻辑
            style |= WINDOW_STYLE(SS_NOPREFIX as u32);
        };

        style
    }
}
pub enum TextViewMsgType {
    Clicked,
    DoubleClicked,
    Disable,
    Enable,
}
pub struct TextViewMsg {
    hwnd: HWND,
    pub bm_type: TextViewMsgType,
}
impl Control for TextView {
    type MsgType = TextViewMsg;
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
    unsafe fn is_self(_wnd: &HWND) -> Result<bool> {
        eprint!("impl Control for TextView ->  unsafe fn is_self 未实现");
        Ok(true)
    }
}
impl ControlMsg for TextViewMsg {
    type ControlType = TextView;
    unsafe fn from_msg(ptr: usize) -> Option<Box<Self>> {
        unsafe {
            let nmhdr = *(ptr as *mut NMHDR);
            let code = nmhdr.code;
            let w = nmhdr.hwndFrom.clone();
            let _ = nmhdr;
            use TextViewMsgType::*;
            let bmtype = match code {
                STN_CLICKED => Clicked,
                STN_DBLCLK => DoubleClicked,
                STN_DISABLE => Disable,
                STN_ENABLE => Enable,
                _ => return None,
            };
            Some(Box::new(Self {
                hwnd: w,
                bm_type: bmtype,
            }))
        }
    }
    fn get_control(&self) -> Self::ControlType {
        TextView(self.hwnd)
    }
}

impl TextView {
    pub fn new(
        wnd: &mut Window,
        name: &str,
        pos: Option<RectangleWH>,
        identifier: WindowID,
        control_style: TextViewStyle,
        style: ChildWindowStyles,
        style_ex: NormalWindowExStyles,
        font: bool,
        no_notify: bool,
    ) -> Result<Self> {
        let mut x = control_style.into();
        if !no_notify {
            x |= WINDOW_STYLE(SS_NOTIFY as u32);
        }
        let hwnd = new_control(
            wnd, "STATIC", name, pos, identifier, style, style_ex, x, font, no_notify,
        )?;
        Ok(TextView(hwnd))
    }
}
