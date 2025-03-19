use super::*;
pub struct ImageTextView(HWND); //PUSH1234567890
unsafe impl Send for ImageTextView {}
unsafe impl Sync for ImageTextView {}
use windows::Win32::System::SystemServices::*;
pub enum ViewContent{
    Text(String), 
    Icon(Icon), 
    Cursor(Cursor), 
    Bitmap(Bitmap), 
    EnhMetaFile(EnhMetaFile), 
}
#[repr(C)]
struct NMHDRSTATIC {
	nmhdr:NMHDR, 
	DC:HANDLE, 
}
//样式部分为AI生成
pub enum Alignment {
    Center,
    Left,
    Right,
    LeftNoWrap,
    Simple,
}
pub enum EllipsisType {
    None,
    End,
    Path,
    Word,
}

pub enum ViewType {
    Text {
        text: String,
        align: Alignment,
        ellipsis: EllipsisType,
        no_prefix: bool,
    },
    Bitmap {
        image: Bitmap,
        reasize_image: bool,    // SS_REALSIZEIMAGE
        right_just: bool,       // SS_RIGHTJUST
        center_image: bool,     // SS_CENTERIMAGE
    },
    Icon {
        icon: Icon,
        reasize_control: bool,  // SS_REALSIZECONTROL
        right_just: bool,       // SS_RIGHTJUST
    },
    Cursor {
        cursor: Cursor,
        reasize_control: bool,  // SS_REALSIZECONTROL
        right_just: bool,       // SS_RIGHTJUST
    },
    EnhMetaFile(EnhMetaFile),   // SS_ENHMETAFILE
}
impl Into<ViewContent> for ViewType {
    fn into(self) -> ViewContent {
        use ViewType::*;
        match self {
            Text {text: s,align: _,ellipsis: _,no_prefix: _} => ViewContent::Text(s), 
            Bitmap {image: b, reasize_image: _,right_just: _,center_image: _,} => ViewContent::Bitmap(b), 
            Icon {icon: c,reasize_control: _,right_just: _,} => ViewContent::Icon(c),
            Cursor {cursor: s,reasize_control: _,right_just: _,} => ViewContent::Cursor(s), 
            EnhMetaFile(e) => ViewContent::EnhMetaFile(e)
        }
    }
}
pub struct ImageTextViewStyle {
    pub stype: ViewType, 

    pub black_frame: bool,      // SS_BLACKFRAME
    pub black_rect: bool,       // SS_BLACKRECT
    pub etched_frame: bool,     // SS_ETCHEDFRAME
    pub etched_horz: bool,      // SS_ETCHEDHORZ
    pub etched_vert: bool,      // SS_ETCHEDVERT
    pub gray_frame: bool,       // SS_GRAYFRAME
    pub gray_rect: bool,        // SS_GRAYRECT
    pub white_frame: bool,      // SS_WHITEFRAME
    pub white_rect: bool,       // SS_WHITERECT
    pub sunken: bool,           // SS_SUNKEN
    pub extra_notify: bool,     // SS_NOTIFY
}

impl Into<(WINDOW_STYLE, ViewContent)> for ImageTextViewStyle {
    fn into(self) -> (WINDOW_STYLE, ViewContent) {
        let mut window_style = WINDOW_STYLE(0);
        let content_data = match self.stype {
            ViewType::Text { text, align, ellipsis, no_prefix } => {
                window_style.0 |= SS_LEFT.0;
                match align {
                    Alignment::Center => window_style.0 |= SS_CENTER.0,
                    Alignment::Left => window_style.0 |= SS_LEFT.0,
                    Alignment::Right => window_style.0 |= SS_RIGHT.0,
                    Alignment::LeftNoWrap => window_style.0 |= SS_LEFTNOWORDWRAP.0,
                    Alignment::Simple => window_style.0 |= SS_SIMPLE.0,
                }
                
                match ellipsis {
                    EllipsisType::End => window_style.0 |= SS_ENDELLIPSIS.0,
                    EllipsisType::Path => window_style.0 |= SS_PATHELLIPSIS.0,
                    EllipsisType::Word => window_style.0 |= SS_WORDELLIPSIS.0,
                    _ => (),
                }

                if no_prefix {
                    window_style.0 |= SS_NOPREFIX.0;
                }

                ViewContent::Text(text)
            }

            ViewType::Bitmap { image, reasize_image, right_just, center_image } => {
                window_style.0 |= SS_BITMAP.0;
                if reasize_image {
                    window_style.0 |= SS_REALSIZEIMAGE.0;
                }
                if right_just {
                    window_style.0 |= SS_RIGHTJUST.0;
                }
                if center_image {
                    window_style.0 |= SS_CENTERIMAGE.0;
                }

                ViewContent::Bitmap(image) 
            }

            ViewType::Icon { icon, reasize_control, right_just } => {
                window_style.0 |= SS_ICON.0;
                if reasize_control {
                    window_style.0 |= SS_REALSIZECONTROL.0;
                }
                if right_just {
                    window_style.0 |= SS_RIGHTJUST.0;
                }

                ViewContent::Icon(icon)
            }

            ViewType::Cursor { cursor, reasize_control, right_just } => {
                window_style.0 |= SS_ICON.0;
                if reasize_control {
                    window_style.0 |= SS_REALSIZECONTROL.0;
                }
                if right_just {
                    window_style.0 |= SS_RIGHTJUST.0;
                }

                ViewContent::Cursor(cursor)
            }

            ViewType::EnhMetaFile(enh) => {
                window_style.0 |= SS_ENHMETAFILE.0;
                ViewContent::EnhMetaFile(enh)
            }
        };

        window_style.0 |= 
            (self.black_frame as u32)   * SS_BLACKFRAME.0 +
            (self.black_rect as u32)    * SS_BLACKRECT .0 +
            (self.etched_frame as u32)  * SS_ETCHEDFRAME.0+
            (self.etched_horz as u32)   * SS_ETCHEDHORZ.0 +
            (self.etched_vert as u32)   * SS_ETCHEDVERT.0 +
            (self.gray_frame as u32)    * SS_GRAYFRAME .0 +
            (self.gray_rect as u32)     * SS_GRAYRECT  .0 +
            (self.white_frame as u32)   * SS_WHITEFRAME.0 +
            (self.white_rect as u32)    * SS_WHITERECT .0 +
            (self.sunken as u32)        * SS_SUNKEN    .0 +
            (self.extra_notify as u32)  * SS_NOTIFY    .0 ;

        (window_style, content_data)
    }
}

pub enum ImageTextViewMsgType {
    Clicked,
    DoubleClicked,
    Disable,
    Enable,
    Colour(HANDLE)
}
pub struct ImageTextViewMsg {
    hwnd: HWND,
    pub bm_type: ImageTextViewMsgType,
}
impl Control for ImageTextView {
    type MsgType = ImageTextViewMsg;
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
        eprint!("impl Control for ImageTextView ->  unsafe fn is_self 未实现");
        Ok(true)
    }
}
impl ControlMsg for ImageTextViewMsg {
    type ControlType = ImageTextView;
    unsafe fn from_msg(ptr: usize) -> Option<Box<Self>> {
        unsafe {
            let nmhdr = *(ptr as *mut NMHDR);
            let code = nmhdr.code;
            let w = nmhdr.hwndFrom.clone();
            let _ = nmhdr;
            use ImageTextViewMsgType::*;
            let bmtype = match code {
                STN_CLICKED => Clicked,
                STN_DBLCLK => DoubleClicked,
                STN_DISABLE => Disable,
                STN_ENABLE => Enable,
                WM_CTLCOLORSTATIC => {
                    let nmhdr = (*(ptr as *mut NMHDRSTATIC)).DC;
                    Colour(nmhdr)
                }
                _ => return None,
            };
            Some(Box::new(Self {
                hwnd: w,
                bm_type: bmtype,
            }))
        }
    }
    fn get_control(&self) -> Self::ControlType {
        ImageTextView(self.hwnd)
    }
}

impl ImageTextView {
    pub fn new(
        wnd: &mut Window,
        pos: Option<Rectangle>,
        identifier: WindowID,
        control_style: ImageTextViewStyle,
        style: ChildWindowStyles,
        style_ex: NormalWindowExStyles,
        font: bool,
        no_notify: bool,
    ) -> Result<Self> {
        let (mut x,y) = control_style.into();
        if !no_notify {
            x |= WINDOW_STYLE(SS_NOTIFY.0 as u32);
        }
        let hwnd = match y {
            ViewContent::Text(z) => ImageTextView(new_control(wnd, "STATIC", &z, pos, identifier, style, style_ex, x, font, no_notify)?), 
            v => {
                let mut ra = ImageTextView(new_control(wnd, "STATIC", &("id:".to_owned() + &identifier.to_string()), pos, identifier, style, style_ex, x, font, no_notify)?);
                ra.change_content(v)?;
                ra
            },
        };
        
        Ok(hwnd)
    }
    //get_content\change_content ai+修改
    pub fn get_content(&self) -> Result<ViewContent> {
        unsafe {
            let style = WINDOW_STYLE(GetWindowLongW(self.0, GWL_STYLE) as u32);
            let hwnd = self.0;
            if style.contains(WINDOW_STYLE(SS_BITMAP.0)) {
                let hbitmap = SendMessageW(
                    hwnd,
                    STM_GETIMAGE,
                    Some(WPARAM(IMAGE_BITMAP.0 as usize)),
                    None
                ).0 as *mut c_void;
                if !hbitmap.is_null() {
                    return Ok(ViewContent::Bitmap(HBITMAP(hbitmap).into()));
                }
            }
            else if style.contains(WINDOW_STYLE(SS_ICON.0)) {
                let hicon = SendMessageW(
                    hwnd,
                    STM_GETICON,
                    None,
                    None
                ).0 as *mut c_void;
                if !hicon.is_null() {
                    return Ok(ViewContent::Icon(HICON(hicon).into()));
                }
            }
            else if style.contains(WINDOW_STYLE(SS_ENHMETAFILE.0)) {
                let henh = SendMessageW(
                    hwnd,
                    STM_GETIMAGE,
                    Some(WPARAM(IMAGE_ENHMETAFILE as usize)),
                    None
                ).0 as *mut c_void;
                if !henh.is_null() {
                    return Ok(ViewContent::EnhMetaFile(HENHMETAFILE(henh).into()));
                }
            }
            else {
                let text = {
                    let length = 
                        unsafe { SendMessageW(hwnd, WM_GETTEXTLENGTH, None, None).0 }
                            as usize;
                    if length == 0{
                        String::new() 
                    } else {
                        let mut buffer: Vec<u16> = vec![0; length + 1];
                        unsafe {
                            SendMessageW(
                                hwnd,
                                WM_GETTEXT,
                                Some(WPARAM(length)),
                                Some(LPARAM(buffer.as_mut_ptr() as isize)),
                            )
                            .0;
                        }
                        String::from_utf16_lossy(&buffer[..length])
                    }
                };
                return Ok(ViewContent::Text(text));
            }
            Err(Error::new(ERROR_NOT_SUPPORTED.into(), ""))
        }
    }

    pub fn change_content(&mut self, content: ViewContent) -> Result<()> {
        unsafe {
            let hwnd = self.0;
            let (new_style, msg, wparam, lparam) = match content {
                ViewContent::Text(text) => {
                    let mut style = GetWindowLongW(hwnd, GWL_STYLE);
                    style &= !(SS_BITMAP.0 as i32 | SS_ICON.0 as i32 | SS_ENHMETAFILE.0 as i32);
                    SetWindowLongW(hwnd, GWL_STYLE, style);
                    let (note_ptr, _note_u16) = str_to_pcwstr(&text);
                    if unsafe {
                        SendMessageW(
                            self.0,
                            BCM_SETNOTE,
                            Some(WPARAM(0)),
                            Some(LPARAM(note_ptr.0 as isize)),
                        )
                    }.0 == 0{
                        return Err(Error::from_win32());
                    }
                    return Ok(());
                },
                ViewContent::Icon(icon) => {
                    (SS_ICON.0 as i32, STM_SETICON, Some(WPARAM(<Icon as Into<HICON>>::into(icon).0 as usize)), None)
                },
                ViewContent::Cursor(cursor) => {
                    (SS_ICON.0 as i32, STM_SETICON, Some(WPARAM(<Cursor as Into<HCURSOR>>::into(cursor).0 as usize)), None)
                },
                ViewContent::Bitmap(bitmap) => {
                    (SS_BITMAP.0 as i32, STM_SETIMAGE, Some(WPARAM(IMAGE_BITMAP.0 as usize)), Some(LPARAM(<Bitmap as Into<HBITMAP>>::into(bitmap).0 as isize)))
                },
                ViewContent::EnhMetaFile(enh) => {
                    (SS_ENHMETAFILE.0 as i32, STM_SETIMAGE, Some(WPARAM(IMAGE_ENHMETAFILE as usize)), Some(LPARAM(<EnhMetaFile as Into<HENHMETAFILE>>::into(enh).0 as isize)))
                },
            };
            let mut style = GetWindowLongW(hwnd, GWL_STYLE);
            style &= !(SS_BITMAP.0 as i32 | SS_ICON.0 as i32 | SS_ENHMETAFILE.0 as i32);
            style |= new_style;
            if SetWindowLongW(hwnd, GWL_STYLE, style) == 0{
                return Err(Error::from_win32());
            };
            if SendMessageW(hwnd,msg,wparam,lparam).0 == 0{
                return Err(Error::from_win32());
            };
            Ok(())
        }
    }
}

