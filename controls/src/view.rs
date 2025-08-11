use super::*;
use windows_sys::Win32::System::SystemServices::*;
#[derive(Default, Clone, Copy, Hash)]
pub enum Alignment {
    Center,
    #[default]
    Left,
    Right,
    LeftNoWrap,
    Simple,
}
#[derive(Default, Clone, Copy, Hash)]
pub enum EllipsisType {
    #[default]
    None,
    End,
    Path,
    Word,
}
pub struct TextViewContent {
    pub text: String,
    pub align: Alignment,
    pub ellipsis: EllipsisType,
    pub no_prefix: bool,
}
impl Into<(WINDOW_STYLE, String)> for TextViewContent {
    fn into(self) -> (WINDOW_STYLE, String) {
        let mut window_style = (self.no_prefix as WINDOW_STYLE) * SS_NOPREFIX;
        match self.align {
            Alignment::Center => window_style |= SS_CENTER,
            Alignment::Left => window_style |= SS_LEFT,
            Alignment::Right => window_style |= SS_RIGHT,
            Alignment::LeftNoWrap => window_style |= SS_LEFTNOWORDWRAP,
            Alignment::Simple => window_style |= SS_SIMPLE,
        };

        match self.ellipsis {
            EllipsisType::None => (),
            EllipsisType::End => window_style |= SS_ENDELLIPSIS,
            EllipsisType::Path => window_style |= SS_PATHELLIPSIS,
            EllipsisType::Word => window_style |= SS_WORDELLIPSIS,
        };
        (window_style, self.text)
    }
}
pub enum ImageViewContent {
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
    Bitmap {
        image: Bitmap,
        name: String,
        reasize_image: bool, // SS_REALSIZEIMAGE
        right_just: bool,    // SS_RIGHTJUST
        center_image: bool,  // SS_CENTERIMAGE
    },
    EnhMetaFile {
        // SS_ENHMETAFILE
        enh_meta_file: EnhMetaFile,
        name: String,
    },
}
pub enum ImageViewTempleContent {
    Icon {
        icon: ResourceID,
        reasize_control: bool, // SS_REALSIZECONTROL
        right_just: bool,      // SS_RIGHTJUST
    },
    Cursor {
        cursor: ResourceID,
        reasize_control: bool, // SS_REALSIZECONTROL
        right_just: bool,      // SS_RIGHTJUST
    },
    Bitmap {
        image: ResourceID,
        reasize_image: bool, // SS_REALSIZEIMAGE
        right_just: bool,    // SS_RIGHTJUST
        center_image: bool,  // SS_CENTERIMAGE
    },
    EnhMetaFile {
        enh_meta_file: ResourceID,
    },
}
pub type TextViewStyle = ViewOption<TextViewContent>;
pub type ImageViewStyle = ViewOption<ImageViewContent>;
impl Into<(WINDOW_STYLE, (String, HANDLE, GDI_IMAGE_TYPE))> for ImageViewContent {
    fn into(self) -> (WINDOW_STYLE, (String, HANDLE, GDI_IMAGE_TYPE)) {
        match self {
            ImageViewContent::Icon {
                icon,
                name,
                reasize_control,
                right_just,
            } => {
                let mut window_style = SS_ICON;
                set_style(&mut window_style, SS_REALSIZECONTROL, reasize_control);
                set_style(&mut window_style, SS_RIGHTJUST, right_just);

                (window_style, (name, icon.handle, IMAGE_ICON))
            }

            ImageViewContent::Cursor {
                cursor,
                name,
                reasize_control,
                right_just,
            } => {
                let mut window_style = SS_ICON;
                set_style(&mut window_style, SS_REALSIZECONTROL, reasize_control);
                set_style(&mut window_style, SS_RIGHTJUST, right_just);

                (window_style, (name, cursor.handle, IMAGE_CURSOR))
            }
            ImageViewContent::Bitmap {
                image,
                name,
                reasize_image,
                right_just,
                center_image,
            } => {
                let mut window_style = SS_BITMAP;
                set_style(&mut window_style, SS_REALSIZEIMAGE, reasize_image);
                set_style(&mut window_style, SS_RIGHTJUST, right_just);
                set_style(&mut window_style, SS_CENTERIMAGE, center_image);

                (window_style, (name, image.handle, IMAGE_BITMAP))
            }

            ImageViewContent::EnhMetaFile {
                enh_meta_file: enh,
                name,
            } => (SS_ENHMETAFILE, (name, enh.handle, IMAGE_ENHMETAFILE)),
        }
    }
}
pub type TextViewTemple = ViewOption<TextViewContent>;
impl DialogTempleControl for TextViewTemple {
    fn pre_compile(self, pos: FontPoint, size: FontSize, identifier: WindowID) -> String {
        let (style, style_ex, ct) = self.into();
        format!(
            "CONTROL \"{}\", {}, \"Static\", 0x{:04X}, {}, {}, {}, {}, 0x{:04X}",
            ct, identifier, style, pos.x, pos.y, size.width, size.height, style_ex
        )
    }
}
pub type ImageViewTemple = ViewOption<ImageViewTempleContent>;
impl Into<(WINDOW_STYLE, ResourceID)> for ImageViewTempleContent {
    fn into(self) -> (WINDOW_STYLE, ResourceID) {
        match self {
            ImageViewTempleContent::Icon {
                icon,
                reasize_control,
                right_just,
            } => {
                let mut window_style = SS_ICON;
                set_style(&mut window_style, SS_REALSIZECONTROL, reasize_control);
                set_style(&mut window_style, SS_RIGHTJUST, right_just);

                (window_style, icon)
            }

            ImageViewTempleContent::Cursor {
                cursor,
                reasize_control,
                right_just,
            } => {
                let mut window_style = SS_ICON;
                set_style(&mut window_style, SS_REALSIZECONTROL, reasize_control);
                set_style(&mut window_style, SS_RIGHTJUST, right_just);

                (window_style, cursor)
            }
            ImageViewTempleContent::Bitmap {
                image,
                reasize_image,
                right_just,
                center_image,
            } => {
                let mut window_style = SS_BITMAP;
                set_style(&mut window_style, SS_REALSIZEIMAGE, reasize_image);
                set_style(&mut window_style, SS_RIGHTJUST, right_just);
                set_style(&mut window_style, SS_CENTERIMAGE, center_image);

                (window_style, image)
            }
            ImageViewTempleContent::EnhMetaFile { enh_meta_file: enh } => (SS_ENHMETAFILE, enh),
        }
    }
}
impl DialogTempleControl for ImageViewTemple {
    fn pre_compile(self, pos: FontPoint, size: FontSize, identifier: WindowID) -> String {
        let (style, style_ex, ct) = self.into();
        format!(
            "CONTROL \"{}\", {}, \"Static\", 0x{:04X}, {}, {}, {}, {}, 0x{:04X}",
            match ct {
                StringId(y) => {
                    let result = y.to_string();
                    if result.parse::<f32>().is_ok() {
                        panic!("无效的资源ID，StringId不能由纯数字组成（包括小数）")
                    };
                    format!("\"{}\"", result)
                }
                NumberId(x) => x.to_string(),
            },
            identifier,
            style,
            pos.x,
            pos.y,
            size.width,
            size.height,
            style_ex
        )
    }
}
pub struct ViewOption<T> {
    pub style: ChildWindowStyles,
    pub content: T,
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
impl<T> ViewOption<T> {
    #[inline]
    pub fn enable_notify(mut self) -> Self {
        self.extra_notify = true;
        self
    }
}
impl ImageViewStyle {
    pub fn new_icon(name: &str, icon: Icon) -> Self {
        Self {
            style: ChildWindowStyles::default(),
            content: ImageViewContent::Icon {
                icon,
                name: name.to_string(),
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
}
impl TextViewStyle {
    pub fn new(text: &str) -> Self {
        Self {
            style: ChildWindowStyles::default(),
            content: TextViewContent {
                text: text.to_string(),
                align: Alignment::default(),
                ellipsis: EllipsisType::default(),
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

impl<D, T> Into<(WINDOW_STYLE, WINDOW_EX_STYLE, D)> for ViewOption<T>
where
    T: Into<(WINDOW_STYLE, D)>,
{
    fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE, D) {
        let (mut style, style_ex) = self.style.into();
        style |= (self.black_frame as WINDOW_STYLE) * SS_BLACKFRAME
            + (self.black_rect as WINDOW_STYLE) * SS_BLACKRECT
            + (self.etched_frame as WINDOW_STYLE) * SS_ETCHEDFRAME
            + (self.etched_horz as WINDOW_STYLE) * SS_ETCHEDHORZ
            + (self.etched_vert as WINDOW_STYLE) * SS_ETCHEDVERT
            + (self.gray_frame as WINDOW_STYLE) * SS_GRAYFRAME
            + (self.gray_rect as WINDOW_STYLE) * SS_GRAYRECT
            + (self.white_frame as WINDOW_STYLE) * SS_WHITEFRAME
            + (self.white_rect as WINDOW_STYLE) * SS_WHITERECT
            + (self.sunken as WINDOW_STYLE) * SS_SUNKEN
            + (self.extra_notify as WINDOW_STYLE) * SS_NOTIFY;
        let (style2, data) = self.content.into();
        (style | style2, style_ex, data)
    }
}
#[repr(C)]
#[allow(non_snake_case)]
struct NMHDRSTATIC {
    #[allow(non_snake_case)]
    nmhdr: NMHDR,
    #[allow(non_snake_case)]
    DC: HANDLE,
}

pub enum ViewMsgType {
    Clicked,       //WM_COMMAND
    DoubleClicked, //WM_COMMAND
    Disable,       //WM_COMMAND
    Enable,        //WM_COMMAND
    Colour(usize), //WM_CTLCOLORSTATIC
}
pub use ViewMsgType as TextViewMsgType;
pub use ViewMsgType as ImageViewMsgType;
define_control! {
    TextView,
    "STATIC",
    {
        match code {
            STN_CLICKED => Clicked,
            STN_DBLCLK => DoubleClicked,
            STN_DISABLE => Disable,
            STN_ENABLE => Enable,
            WM_CTLCOLORSTATIC => {
                let nmhdr = (*(ptr as *mut NMHDRSTATIC)).DC;
                Colour(nmhdr as usize)
            }
            _ => return Err(ERROR_MSG_CODE_NOT_SUPPORT),
        }
    },
    {
        is_some_window(wnd, L!("Static"))
    },
    {
        todo!()
    }
}
define_control! {
    ImageView,
    "STATIC",
    {
        match code {
            STN_CLICKED => Clicked,
            STN_DBLCLK => DoubleClicked,
            STN_DISABLE => Disable,
            STN_ENABLE => Enable,
            WM_CTLCOLORSTATIC => {
                let nmhdr = (*(ptr as *mut NMHDRSTATIC)).DC;
                Colour(nmhdr as usize)
            }
            _ => return Err(ERROR_MSG_CODE_NOT_SUPPORT),
        }
    },
    {
        is_some_window(wnd, L!("Static"))
    },
    {
        todo!()
    }
}
impl TextControl for TextView {}
impl CommonControl for ImageView {
    type Style = ImageViewStyle;
    fn new(
        wnd: &mut Window,
        pos: Option<Rect>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<Self> {
        let (style, style_ex, (name, data, flag)) = control_style.into();
        let hwnd = new_control(
            wnd,
            w!("Static"),
            name,
            pos,
            identifier,
            style,
            style_ex,
            font,
        )?;
        unsafe {
            let _ = SendMessageW(hwnd.handle(), STM_SETIMAGE, flag as WPARAM, data as LPARAM);
        }
        Ok(ImageView(hwnd))
    }
}
impl CommonControl for TextView {
    type Style = TextViewStyle;
    fn new(
        wnd: &mut Window,
        pos: Option<Rect>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<Self> {
        let (style, style_ex, name) = control_style.into();
        Ok(TextView(new_control(
            wnd,
            w!("Static"),
            name,
            pos,
            identifier,
            style,
            style_ex,
            font,
        )?))
    }
}
