use super::*;
use windows::Win32::System::SystemServices::*;
pub enum ViewContent {
    Text(String),
    Icon(Icon),
    Cursor(Cursor),
    Bitmap(Bitmap),
    EnhMetaFile(EnhMetaFile),
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
        name: String,
        reasize_image: bool, // SS_REALSIZEIMAGE
        right_just: bool,    // SS_RIGHTJUST
        center_image: bool,  // SS_CENTERIMAGE
    },
    Icon {
        icon: Icon,
        name: String,
        reasize_control: bool, // SS_REALSIZECONTROL
        right_just: bool,      // SS_RIGHTJUST
    },
    Cursor {
        cursor: Cursor,
        name: String,
        reasize_control: bool, // SS_REALSIZECONTROL
        right_just: bool,      // SS_RIGHTJUST
    },
    EnhMetaFile {
        // SS_ENHMETAFILE
        enh_meta_file: EnhMetaFile,
        name: String,
    },
}
pub struct ImageTextViewStyle {
    pub style: ChildWindowStyles,
    pub stype: ViewType,
    pub black_frame: bool,  // SS_BLACKFRAME
    pub black_rect: bool,   // SS_BLACKRECT
    pub etched_frame: bool, // SS_ETCHEDFRAME
    pub etched_horz: bool,  // SS_ETCHEDHORZ
    pub etched_vert: bool,  // SS_ETCHEDVERT
    pub gray_frame: bool,   // SS_GRAYFRAME
    pub gray_rect: bool,    // SS_GRAYRECT
    pub white_frame: bool,  // SS_WHITEFRAME
    pub white_rect: bool,   // SS_WHITERECT
    pub sunken: bool,       // SS_SUNKEN
    pub extra_notify: bool, // SS_NOTIFY
}
impl ImageTextViewStyle {
    pub fn enable_notify(mut self) -> Self {
        self.extra_notify = true;
        self
    }
    pub fn text(mut self, new_text: &str) -> Self {
        use ViewType::*;
        match &mut self.stype {
            Text { text, .. } => text = new_text.to_string(),
            Bitmap { name, .. } => name = new_text.to_string(),
            Icon { name, .. } => name = new_text.to_string(),
            Cursor { name, .. } => name = new_text.to_string(),
        };
        self
    }
    pub fn new_icon(icon: Icon) -> Self {
        ImageTextViewStyle {
            style: Default::default(),
            stype: ViewType::Icon {
                icon,
                name: icon.handle().0.to_string(),
                reasize_control: false,
                right_just: false,
            },
            black_frame: false,
            black_rect: false,
            etched_frame: false,
            etched_horz: false,
            etched_vert: false,
            gray_frame: false,
            gray_rect: false,
            white_frame: false,
            white_rect: false,
            sunken: false,
            extra_notify: false,
        }
    }
    pub fn new_text(text: &str) -> Self {
        ImageTextViewStyle {
            style: Default::default(),
            stype: ViewType::Text {
                text: text.to_string(),
                align: Alignment::Center,
                ellipsis: EllipsisType::None,
                no_prefix: false,
            },
            black_frame: false,
            black_rect: false,
            etched_frame: false,
            etched_horz: false,
            etched_vert: false,
            gray_frame: false,
            gray_rect: false,
            white_frame: false,
            white_rect: false,
            sunken: false,
            extra_notify: false,
        }
    }
}
impl Into<ViewContent> for ViewType {
    fn into(self) -> ViewContent {
        use ViewType::*;
        match self {
            Text {
                text: s,
                align: _,
                ellipsis: _,
                no_prefix: _,
            } => ViewContent::Text(s),
            Bitmap {
                image: b,
                reasize_image: _,
                right_just: _,
                center_image: _,
            } => ViewContent::Bitmap(b),
            Icon {
                icon: c,
                reasize_control: _,
                right_just: _,
            } => ViewContent::Icon(c),
            Cursor {
                cursor: s,
                reasize_control: _,
                right_just: _,
            } => ViewContent::Cursor(s),
            EnhMetaFile {
                enh_meta_file: e,
                name,
            } => ViewContent::EnhMetaFile(e),
        }
    }
}

impl Into<(WINDOW_STYLE, ViewContent, ChildWindowStyles)> for ImageTextViewStyle {
    fn into(self) -> (WINDOW_STYLE, ViewContent, ChildWindowStyles) {
        let mut window_style = WINDOW_STYLE(0);
        let content_data = match self.stype {
            ViewType::Text {
                text,
                align,
                ellipsis,
                no_prefix,
            } => {
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

            ViewType::Bitmap {
                image,
                reasize_image,
                right_just,
                center_image,
            } => {
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

            ViewType::Icon {
                icon,
                reasize_control,
                right_just,
            } => {
                window_style.0 |= SS_ICON.0;
                if reasize_control {
                    window_style.0 |= SS_REALSIZECONTROL.0;
                }
                if right_just {
                    window_style.0 |= SS_RIGHTJUST.0;
                }

                ViewContent::Icon(icon)
            }

            ViewType::Cursor {
                cursor,
                reasize_control,
                right_just,
            } => {
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

        window_style.0 |= (self.black_frame as u32) * SS_BLACKFRAME.0
            + (self.black_rect as u32) * SS_BLACKRECT.0
            + (self.etched_frame as u32) * SS_ETCHEDFRAME.0
            + (self.etched_horz as u32) * SS_ETCHEDHORZ.0
            + (self.etched_vert as u32) * SS_ETCHEDVERT.0
            + (self.gray_frame as u32) * SS_GRAYFRAME.0
            + (self.gray_rect as u32) * SS_GRAYRECT.0
            + (self.white_frame as u32) * SS_WHITEFRAME.0
            + (self.white_rect as u32) * SS_WHITERECT.0
            + (self.sunken as u32) * SS_SUNKEN.0
            + (self.extra_notify as u32) * SS_NOTIFY.0;

        (window_style, content_data, self.style)
    }
}
#[repr(C)]
#[allow(non_snake_case)]
struct NMHDRSTATIC {
    #[allow(non_snake_case)]
    nmhdr: NMHDR,
    #[allow(non_snake_case)]
    DC: wHANDLE,
}
pub enum ImageTextViewMsgType {
    Clicked,       //WM_COMMAND
    DoubleClicked, //WM_COMMAND
    Disable,       //WM_COMMAND
    Enable,        //WM_COMMAND
    Colour(usize), //WM_CTLCOLORSTATIC
}
define_control! {
    ImageTextView,
    "STATIC",
    {
        match code {
                STN_CLICKED => Clicked,
                STN_DBLCLK => DoubleClicked,
                STN_DISABLE => Disable,
                STN_ENABLE => Enable,
                WM_CTLCOLORSTATIC => {
                    let nmhdr = (*(ptr as *mut NMHDRSTATIC)).DC.0;
                    Colour(nmhdr as usize)
                }
                _ => return Err(ERROR_MSG_CODE_NOT_SUPPORT),
            }
    },
    {
        is_some_window(wnd, "Static")
    },
    {
        todo!()
    }
}
impl DataControl for ImageTextView {
    type Data = ViewContent;
    type Style = ImageTextViewStyle;
    fn new(
        wnd: &mut Window,
        name: &str,
        pos: Option<Rect>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<Self> {
        let (cs, data, cws) = control_style.into();
        let hwnd = match data {
            ViewContent::Text(text) => ImageTextView(new_control(
                wnd,
                "STATIC",
                &text,
                pos,
                identifier,
                (cs, cws),
                font,
            )?),
            other => {
                let mut ra = ImageTextView(new_control(
                    wnd,
                    "STATIC",
                    name,
                    pos,
                    identifier,
                    (cs, cws),
                    font,
                )?);
                ra.change_content(other)?;
                ra
            }
        };

        Ok(hwnd)
    }
    fn set_data(_: Window, _: Self::Data) -> Result<Self> {
        unreachable!()
    }
}
impl ImageTextView {
    //get_content\change_content ai+修改
    pub fn get_content(&self) -> Result<ViewContent> {
        unsafe {
            let hwnd = self.0.handle();
            let style = WINDOW_STYLE(GetWindowLongW(hwnd, GWL_STYLE) as u32);
            if style.contains(WINDOW_STYLE(SS_BITMAP.0)) {
                let hbitmap = SendMessageW(
                    hwnd,
                    STM_GETIMAGE,
                    Some(WPARAM(IMAGE_BITMAP.0 as usize)),
                    None,
                )
                .0 as *mut c_void;
                if !hbitmap.is_null() {
                    return Ok(ViewContent::Bitmap(
                        windows::Win32::Graphics::Gdi::HBITMAP(hbitmap).into(),
                    ));
                }
            } else if style.contains(WINDOW_STYLE(SS_ICON.0)) {
                let hicon = SendMessageW(hwnd, STM_GETICON, None, None).0 as *mut c_void;
                if !hicon.is_null() {
                    return Ok(ViewContent::Icon(HICON(hicon).into()));
                }
            } else if style.contains(WINDOW_STYLE(SS_ENHMETAFILE.0)) {
                let henh = SendMessageW(
                    hwnd,
                    STM_GETIMAGE,
                    Some(WPARAM(IMAGE_ENHMETAFILE as usize)),
                    None,
                )
                .0 as *mut c_void;
                if !henh.is_null() {
                    return Ok(ViewContent::EnhMetaFile(
                        windows::Win32::Graphics::Gdi::HENHMETAFILE(henh).into(),
                    ));
                }
            } else {
                let text = {
                    let length = SendMessageW(hwnd, WM_GETTEXTLENGTH, None, None).0 as usize;
                    if length == 0 {
                        String::new()
                    } else {
                        let mut buffer: Vec<u16> = vec![0; length + 1];
                        SendMessageW(
                            hwnd,
                            WM_GETTEXT,
                            Some(WPARAM(length)),
                            Some(LPARAM(buffer.as_mut_ptr() as isize)),
                        )
                        .0;

                        String::from_utf16_lossy(&buffer[..length])
                    }
                };
                return Ok(ViewContent::Text(text));
            }
            Err(ERROR_NOT_SUPPORTED)
        }
    }

    pub fn change_content(&mut self, content: ViewContent) -> Result<()> {
        unsafe {
            let hwnd = self.0.handle();
            let (new_style, msg, wparam, lparam) = match content {
                ViewContent::Text(text) => {
                    let mut style = GetWindowLongW(hwnd, GWL_STYLE);
                    style &= !(SS_BITMAP.0 as i32 | SS_ICON.0 as i32 | SS_ENHMETAFILE.0 as i32);
                    SetWindowLongW(hwnd, GWL_STYLE, style);
                    let (note_ptr, _note_u16) = str_to_pcwstr(&text);
                    if SendMessageW(
                        hwnd,
                        WM_SETTEXT,
                        Some(WPARAM(0)),
                        Some(LPARAM(note_ptr.0 as isize)),
                    )
                    .0 == 0
                    {
                        return Err(Error::correct_error());
                    }
                    return Ok(());
                }
                ViewContent::Icon(icon) => (
                    SS_ICON.0 as i32,
                    STM_SETICON,
                    Some(WPARAM(<Icon as Into<HICON>>::into(icon).0 as usize)),
                    None,
                ),
                ViewContent::Cursor(cursor) => (
                    SS_ICON.0 as i32,
                    STM_SETICON,
                    Some(WPARAM(<Cursor as Into<HCURSOR>>::into(cursor).0 as usize)),
                    None,
                ),
                ViewContent::Bitmap(bitmap) => (
                    SS_BITMAP.0 as i32,
                    STM_SETIMAGE,
                    Some(WPARAM(IMAGE_BITMAP.0 as usize)),
                    Some(LPARAM(
                        <Bitmap as Into<windows::Win32::Graphics::Gdi::HBITMAP>>::into(bitmap).0
                            as isize,
                    )),
                ),
                ViewContent::EnhMetaFile(enh) => (
                    SS_ENHMETAFILE.0 as i32,
                    STM_SETIMAGE,
                    Some(WPARAM(IMAGE_ENHMETAFILE as usize)),
                    Some(LPARAM(
                        <EnhMetaFile as Into<windows::Win32::Graphics::Gdi::HENHMETAFILE>>::into(
                            enh,
                        )
                        .0 as isize,
                    )),
                ),
            };
            let mut style = GetWindowLongW(hwnd, GWL_STYLE);
            style &= !(SS_BITMAP.0 as i32 | SS_ICON.0 as i32 | SS_ENHMETAFILE.0 as i32);
            style |= new_style;
            if SetWindowLongW(hwnd, GWL_STYLE, style) == 0 {
                return Err(Error::correct_error());
            };
            SendMessageW(hwnd, msg, wparam, lparam);
            Ok(())
        }
    }
}
