use super::*;
pub struct ImageTextView(HWND); //PUSH1234567890
unsafe impl Send for ImageTextView {}
unsafe impl Sync for ImageTextView {}
use windows::Win32::System::SystemServices::*;
pub enum ViewContent{
    Text(String)
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

use either::Either;

// 新的ViewType枚举
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

// 修改后的StaticStyle结构
pub struct ImageTextViewStyle {
    pub stype: StaticType, // 保持与系统样式位的关联

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

// 实现Into转换
impl Into<(WINDOW_STYLE, Either<String, HANDLE>)> for (StaticStyle, ViewType) {
    fn into(self) -> (WINDOW_STYLE, Either<String, HANDLE>) {
        let (style, content) = (self.0, self.1);

        let mut window_style = WINDOW_STYLE(0);
        let mut content_data = Either::Left("".to_string());

        // 处理ViewType的样式和内容
        match content {
            ViewType::Text { text, align, ellipsis, no_prefix } => {
                window_style.0 |= SS_LEFT as u32; // 默认对齐方式
                match align {
                    Alignment::Center => window_style.0 |= SS_CENTER as u32,
                    Alignment::Left => window_style.0 |= SS_LEFT as u32,
                    Alignment::Right => window_style.0 |= SS_RIGHT as u32,
                    Alignment::LeftNoWrap => window_style.0 |= SS_LEFTNOWORDWRAP as u32,
                    Alignment::Simple => window_style.0 |= SS_SIMPLE as u32,
                }
                
                match ellipsis {
                    EllipsisType::End => window_style.0 |= SS_ENDELLIPSIS as u32,
                    EllipsisType::Path => window_style.0 |= SS_PATHELLIPSIS as u32,
                    EllipsisType::Word => window_style.0 |= SS_WORDELLIPSIS as u32,
                    _ => (),
                }

                if no_prefix {
                    window_style.0 |= SS_NOPREFIX as u32;
                }

                content_data = Either::Left(text);
            }

            ViewType::Bitmap { image, reasize_image, right_just, center_image } => {
                window_style.0 |= SS_BITMAP as u32;
                if reasize_image {
                    window_style.0 |= SS_REALSIZEIMAGE as u32;
                }
                if right_just {
                    window_style.0 |= SS_RIGHTJUST as u32;
                }
                if center_image {
                    window_style.0 |= SS_CENTERIMAGE as u32;
                }

                content_data = Either::Right(image.into()); // Bitmap实现Into<HANDLE>
            }

            ViewType::Icon { icon, reasize_control, right_just } => {
                window_style.0 |= SS_ICON as u32;
                if reasize_control {
                    window_style.0 |= SS_REALSIZECONTROL as u32;
                }
                if right_just {
                    window_style.0 |= SS_RIGHTJUST as u32;
                }

                content_data = Either::Right(icon.into()); // Icon实现Into<HANDLE>
            }

            ViewType::Cursor { cursor, reasize_control, right_just } => {
                window_style.0 |= SS_ICON as u32; // 假设SS_CURSOR存在
                if reasize_control {
                    window_style.0 |= SS_REALSIZECONTROL as u32;
                }
                if right_just {
                    window_style.0 |= SS_RIGHTJUST as u32;
                }

                content_data = Either::Right(cursor.into()); // Cursor实现Into<HANDLE>
            }

            ViewType::EnhMetaFile(enh) => {
                window_style.0 |= SS_ENHMETAFILE as u32;
                content_data = Either::Right(enh.into()); // EnhMetaFile实现Into<HANDLE>
            }
        }

        // 合并StaticStyle的样式
        window_style.0 |= 
            (style.black_frame as u32) * SS_BLACKFRAME as u32 +
            (style.black_rect as u32) * SS_BLACKRECT as u32 +
            (style.etched_frame as u32) * SS_ETCHEDFRAME as u32 +
            (style.etched_horz as u32) * SS_ETCHEDHORZ as u32 +
            (style.etched_vert as u32) * SS_ETCHEDVERT as u32 +
            (style.gray_frame as u32) * SS_GRAYFRAME as u32 +
            (style.gray_rect as u32) * SS_GRAYRECT as u32 +
            (style.white_frame as u32) * SS_WHITEFRAME as u32 +
            (style.white_rect as u32) * SS_WHITERECT as u32 +
            (style.sunken as u32) * SS_SUNKEN as u32 +
            (style.extra_notify as u32) * SS_NOTIFY as u32;

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
                    let nmhdr = *(ptr as *mut NMHDRSTATIC).DC;
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
        name: &str,
        pos: Option<Rectangle>,
        identifier: WindowID,
        control_style: ImageTextViewStyle,
        style: ChildWindowStyles,
        style_ex: NormalWindowExStyles,
        font: bool,
        no_notify: bool,
    ) -> Result<Self> {
        let mut (x,y) = control_style.into();
        if !no_notify {
            x |= WINDOW_STYLE(SS_NOTIFY.0 as u32);
        }
        let hwnd = new_control(
            wnd, "STATIC", name, pos, identifier, style, style_ex, x, font, no_notify,
        )?;
        Ok(ImageTextView(hwnd))
    }
    //get_content,change_content是AI生成 
    pub fn get_content(&self) -> Result<ViewContent> {
        unsafe {
            let style = GetWindowLongW(self.0, GWL_STYLE).0 as u32;
            let hwnd = self.0;

            if (style & SS_BITMAP.0 as u32) != 0 {
                let hbitmap = SendMessageW(
                    hwnd,
                    STM_GETIMAGE,
                    WPARAM(IMAGE_BITMAP),
                    LPARAM(0)
                ).0 as *mut _;
                if !hbitmap.is_null() {
                    return Ok(ViewContent::Bitmap(Bitmap::from_hbitmap(HANDLE(hbitmap))));
                }
            } else if (style & SS_ICON.0 as u32) != 0 {
                let hicon = SendMessageW(
                    hwnd,
                    STM_GETICON,
                    WPARAM(0),
                    LPARAM(0)
                ).0 as *mut _;
                if !hicon.is_null() {
                    return Ok(ViewContent::Icon(Icon::from_hicon(HANDLE(hicon))));
                }
            } else if (style & SS_ENHMETAFILE.0 as u32) != 0 {
                let henh = SendMessageW(
                    hwnd,
                    STM_GETMETARESOURCE,
                    WPARAM(0),
                    LPARAM(0)
                ).0 as *mut _;
                if !henh.is_null() {
                    return Ok(ViewContent::EnhMetaFile(EnhMetaFile::from_henh(HANDLE(henh))));
                }
            } else {
                let text = get_window_text(hwnd)?;
                return Ok(ViewContent::Text(text));
            }
            Err(Error::new(ERROR_NOT_SUPPORTED, ""))
        }
    }

    pub fn change_content(&mut self, content: ViewContent) -> Result<()> {
        unsafe {
            let hwnd = self.0;
            let (new_style, msg, param) = match &content {
                ViewContent::Text(text) => {
                    // 清除所有非文本样式
                    let mut style = GetWindowLongW(hwnd, GWL_STYLE);
                    style &= !(SS_BITMAP.0 as isize | SS_ICON.0 as isize | SS_ENHMETAFILE.0 as isize);
                    SetWindowLongW(hwnd, GWL_STYLE, style);

                    // 发送消息清除旧资源
                    // SendMessageW(hwnd, STM_SETICON, WPARAM(0), LPARAM(0));
                    // SendMessageW(hwnd, STM_SETIMAGE, WPARAM(0), LPARAM(0));
                    // SendMessageW(hwnd, STM_SETMETARESOURCE, WPARAM(0), LPARAM(0));
                    //
                    // 应用样式更改（强制重绘）
                    // SetWindowPos(hwnd, None, 0, 0, 0, 0, 
                    //     SWP_NOSIZE | SWP_NOMOVE | SWP_NOZORDER);

                    // 直接设置文本
                    SetWindowTextW(hwnd, text)?;
                    return Ok(());
                },
                ViewContent::Icon(icon) => {
                    (SS_ICON.0 as isize, STM_SETICON, WPARAM(icon.into().0 as isize))
                },
                ViewContent::Cursor(cursor) => {
                    (SS_ICON.0 as isize, STM_SETICON, WPARAM(cursor.into().0 as isize))
                },
                ViewContent::Bitmap(bitmap) => {
                    (SS_BITMAP.0 as isize, STM_SETIMAGE, WPARAM(IMAGE_BITMAP))
                },
                ViewContent::EnhMetaFile(enh) => {
                    (SS_ENHMETAFILE.0 as isize, STM_SETMETARESOURCE, WPARAM(0))
                },
            };

            // 更新窗口样式
            let mut style = GetWindowLongW(hwnd, GWL_STYLE);
            style &= !(SS_BITMAP.0 as isize | SS_ICON.0 as isize | SS_ENHMETAFILE.0 as isize); // 清除旧样式
            style |= new_style;
            SetWindowLongW(hwnd, GWL_STYLE, style);

            // 发送消息设置内容
            let handle = match &content {
                ViewContent::Icon(icon) => icon.into().0,
                ViewContent::Cursor(cursor) => cursor.into().0,
                ViewContent::Bitmap(bitmap) => bitmap.into().0,
                ViewContent::EnhMetaFile(enh) => enh.into().0,
                _ => 0,
            };
            let _ = SendMessageW(
                hwnd,
                msg,
                param,
                LPARAM(handle as isize)
            );

            // 应用样式更改（强制重绘）
            SetWindowPos(hwnd, None, 0, 0, 0, 0, 
                SWP_NOSIZE | SWP_NOMOVE | SWP_NOZORDER)?;

            Ok(())
        }
    }
}
}
