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
impl From<TextViewContent> for (WINDOW_STYLE, String) {
    fn from(val: TextViewContent) -> Self {
        let mut window_style = (val.no_prefix as WINDOW_STYLE) * SS_NOPREFIX;
        match val.align {
            Alignment::Center => window_style |= SS_CENTER,
            Alignment::Left => window_style |= SS_LEFT,
            Alignment::Right => window_style |= SS_RIGHT,
            Alignment::LeftNoWrap => window_style |= SS_LEFTNOWORDWRAP,
            Alignment::Simple => window_style |= SS_SIMPLE,
        };

        match val.ellipsis {
            EllipsisType::None => (),
            EllipsisType::End => window_style |= SS_ENDELLIPSIS,
            EllipsisType::Path => window_style |= SS_PATHELLIPSIS,
            EllipsisType::Word => window_style |= SS_WORDELLIPSIS,
        };
        (window_style, val.text)
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
impl From<ImageViewContent> for (WINDOW_STYLE, (String, HANDLE, GDI_IMAGE_TYPE)) {
    fn from(val: ImageViewContent) -> Self {
        match val {
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
    #[inline]
    fn pre_compile(self, pos: FontPoint, size: FontSize, identifier: WindowID) -> String {
        let ((style, style_ex), ct) = self.into();
        format!(
            "CONTROL \"{}\", {}, \"Static\", 0x{:04X}, {}, {}, {}, {}, 0x{:04X}",
            ct, identifier, style, pos.x, pos.y, size.width, size.height, style_ex
        )
    }
}
pub type ImageViewTemple = ViewOption<ImageViewTempleContent>;
impl From<ImageViewTempleContent> for (WINDOW_STYLE, ResourceID) {
    fn from(val: ImageViewTempleContent) -> Self {
        match val {
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
    #[inline]
    fn pre_compile(self, pos: FontPoint, size: FontSize, identifier: WindowID) -> String {
        let ((style, style_ex), ct) = self.into();
        format!(
            "CONTROL \"{}\", {}, \"Static\", 0x{:04X}, {}, {}, {}, {}, 0x{:04X}",
            match ct {
                StringId(y) => {
                    let result = y.to_string();
                    check_res_id(&result);
                    format!("\"{result}\"")
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
impl ImageViewTemple {
    pub fn new_icon(icon: ResourceID) -> Self {
        Self {
            style: ChildWindowStyles::default(),
            content: ImageViewTempleContent::Icon {
                icon,
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
impl<D, T> From<ViewOption<T>> for ((WINDOW_STYLE, WINDOW_EX_STYLE), D)
where
    T: Into<(WINDOW_STYLE, D)>,
{
    fn from(val: ViewOption<T>) -> Self {
        let (mut style, style_ex) = val.style.into();
        style |= (val.black_frame as WINDOW_STYLE) * SS_BLACKFRAME
            + (val.black_rect as WINDOW_STYLE) * SS_BLACKRECT
            + (val.etched_frame as WINDOW_STYLE) * SS_ETCHEDFRAME
            + (val.etched_horz as WINDOW_STYLE) * SS_ETCHEDHORZ
            + (val.etched_vert as WINDOW_STYLE) * SS_ETCHEDVERT
            + (val.gray_frame as WINDOW_STYLE) * SS_GRAYFRAME
            + (val.gray_rect as WINDOW_STYLE) * SS_GRAYRECT
            + (val.white_frame as WINDOW_STYLE) * SS_WHITEFRAME
            + (val.white_rect as WINDOW_STYLE) * SS_WHITERECT
            + (val.sunken as WINDOW_STYLE) * SS_SUNKEN
            + (val.extra_notify as WINDOW_STYLE) * SS_NOTIFY;
        let (style2, data) = val.content.into();
        ((style | style2, style_ex), data)
    }
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
                let nmhdr = (*(ptr as *mut NMHDRCOLOR)).DC;
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
                let nmhdr = (*(ptr as *mut NMHDRCOLOR)).DC;
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
    #[inline]
    fn new_raw(
        wnd: &mut Window,
        pos: Option<Rect>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<HWND> {
        let (style, (name, data, flag)) = control_style.into();
        let hwnd = new_control(wnd, w!("Static"), name, pos, identifier, style, font)?;
        unsafe {
            let _ = SendMessageW(hwnd, STM_SETIMAGE, flag as WPARAM, data as LPARAM);
        }
        Ok(hwnd)
    }
}
impl CommonControl for TextView {
    type Style = TextViewStyle;
    #[inline]
    fn new_raw(
        wnd: &mut Window,
        pos: Option<Rect>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<HWND> {
        let (style, name) = control_style.into();
        new_control(wnd, w!("Static"), name, pos, identifier, style, font)
    }
}
