use super::*;
use windows::Win32::Graphics::Gdi::HBRUSH;
#[derive(Clone, PartialEq)]
pub struct WindowClass {
    pub name: Option<(PCWSTR,Vec<u16>)>,
    pub atom: PCWSTR,
    pub handle_instance: Option<HINSTANCE>,
}
pub mod WindowClassP {
    use super::*;
    #[derive(Clone, PartialEq)]
    pub enum BrushC {
        Brush(crate::Win32::brush::Brush),
        //ai写的，ai写重复代码真好用
        ActiveBorder,
        ActiveCaption,
        AppWorkspace,
        Background,
        BtnFace,
        BtnShadow,
        BtnText,
        CaptionText,
        GrayText,
        Highlight,
        HighlightText,
        InactiveBorder,
        InactiveCaption,
        Menu,
        MenuText,
        Scrollbar,
        Window,
        WindowFrame,
        WindowText,
    }
    impl Into<HBRUSH> for BrushC {
        fn into(self) -> HBRUSH {
            let mut result = match self {
                BrushC::Brush(ush) => ush.into(),
                //ai写的，ai写重复代码真好用
                BrushC::ActiveBorder => HBRUSH(COLOR_ACTIVEBORDER.0 as *mut c_void),
                BrushC::ActiveCaption => HBRUSH(COLOR_ACTIVECAPTION.0 as *mut c_void),
                BrushC::AppWorkspace => HBRUSH(COLOR_APPWORKSPACE.0 as *mut c_void),
                BrushC::Background => HBRUSH(COLOR_BACKGROUND.0 as *mut c_void),
                BrushC::BtnFace => HBRUSH(COLOR_BTNFACE.0 as *mut c_void),
                BrushC::BtnShadow => HBRUSH(COLOR_BTNSHADOW.0 as *mut c_void),
                BrushC::BtnText => HBRUSH(COLOR_BTNTEXT.0 as *mut c_void),
                BrushC::CaptionText => HBRUSH(COLOR_CAPTIONTEXT.0 as *mut c_void),
                BrushC::GrayText => HBRUSH(COLOR_GRAYTEXT.0 as *mut c_void),
                BrushC::Highlight => HBRUSH(COLOR_HIGHLIGHT.0 as *mut c_void),
                BrushC::HighlightText => HBRUSH(COLOR_HIGHLIGHTTEXT.0 as *mut c_void),
                BrushC::InactiveBorder => HBRUSH(COLOR_INACTIVEBORDER.0 as *mut c_void),
                BrushC::InactiveCaption => HBRUSH(COLOR_INACTIVECAPTION.0 as *mut c_void),
                BrushC::Menu => HBRUSH(COLOR_MENU.0 as *mut c_void),
                BrushC::MenuText => HBRUSH(COLOR_MENUTEXT.0 as *mut c_void),
                BrushC::Scrollbar => HBRUSH(COLOR_SCROLLBAR.0 as *mut c_void),
                BrushC::Window => HBRUSH(COLOR_WINDOW.0 as *mut c_void),
                BrushC::WindowFrame => HBRUSH(COLOR_WINDOWFRAME.0 as *mut c_void),
                BrushC::WindowText => HBRUSH(COLOR_WINDOWTEXT.0 as *mut c_void),
                // BrushC::ActiveBorder => COLOR_ACTIVEBORDER,
                // BrushC::ActiveCaption => COLOR_ACTIVECAPTION,
                // BrushC::AppWorkspace => COLOR_APPWORKSPACE,
                // BrushC::Background => COLOR_BACKGROUND,
                // BrushC::BtnFace => COLOR_BTNFACE,
                // BrushC::BtnShadow => COLOR_BTNSHADOW,
                // BrushC::BtnText => COLOR_BTNTEXT,
                // BrushC::CaptionText => COLOR_CAPTIONTEXT,
                // BrushC::GrayText => COLOR_GRAYTEXT,
                // BrushC::Highlight => COLOR_HIGHLIGHT,
                // BrushC::HighlightText => COLOR_HIGHLIGHTTEXT,
                // BrushC::InactiveBorder => COLOR_INACTIVEBORDER,
                // BrushC::InactiveCaption => COLOR_INACTIVECAPTION,
                // BrushC::Menu => COLOR_MENU,
                // BrushC::MenuText => COLOR_MENUTEXT,
                // BrushC::Scrollbar => COLOR_SCROLLBAR,
                // BrushC::Window => COLOR_WINDOW,
                // BrushC::WindowFrame => COLOR_WINDOWFRAME,
                // BrushC::WindowText => COLOR_WINDOWTEXT,
            };
            HBRUSH((result.0 as usize +1) as *mut c_void)
        }
    }
    impl From<HBRUSH> for BrushC {
        fn from(ush: HBRUSH) -> Self {
            match HBRUSH((ush.0 as usize -1) as *mut c_void) {
                HBRUSH(val) if val == COLOR_ACTIVEBORDER.0 as *mut c_void => BrushC::ActiveBorder,
                HBRUSH(val) if val == COLOR_ACTIVECAPTION.0 as *mut c_void => BrushC::ActiveCaption,
                HBRUSH(val) if val == COLOR_APPWORKSPACE.0 as *mut c_void => BrushC::AppWorkspace,
                HBRUSH(val) if val == COLOR_BACKGROUND.0 as *mut c_void => BrushC::Background,
                HBRUSH(val) if val == COLOR_BTNFACE.0 as *mut c_void => BrushC::BtnFace,
                HBRUSH(val) if val == COLOR_BTNSHADOW.0 as *mut c_void => BrushC::BtnShadow,
                HBRUSH(val) if val == COLOR_BTNTEXT.0 as *mut c_void => BrushC::BtnText,
                HBRUSH(val) if val == COLOR_CAPTIONTEXT.0 as *mut c_void => BrushC::CaptionText,
                HBRUSH(val) if val == COLOR_GRAYTEXT.0 as *mut c_void => BrushC::GrayText,
                HBRUSH(val) if val == COLOR_HIGHLIGHT.0 as *mut c_void => BrushC::Highlight,
                HBRUSH(val) if val == COLOR_HIGHLIGHTTEXT.0 as *mut c_void => BrushC::HighlightText,
                HBRUSH(val) if val == COLOR_INACTIVEBORDER.0 as *mut c_void => {
                    BrushC::InactiveBorder
                }
                HBRUSH(val) if val == COLOR_INACTIVECAPTION.0 as *mut c_void => {
                    BrushC::InactiveCaption
                }
                HBRUSH(val) if val == COLOR_MENU.0 as *mut c_void => BrushC::Menu,
                HBRUSH(val) if val == COLOR_MENUTEXT.0 as *mut c_void => BrushC::MenuText,
                HBRUSH(val) if val == COLOR_SCROLLBAR.0 as *mut c_void => BrushC::Scrollbar,
                HBRUSH(val) if val == COLOR_WINDOW.0 as *mut c_void => BrushC::Window,
                HBRUSH(val) if val == COLOR_WINDOWFRAME.0 as *mut c_void => BrushC::WindowFrame,
                HBRUSH(val) if val == COLOR_WINDOWTEXT.0 as *mut c_void => BrushC::WindowText,
                HBRUSH(x) => BrushC::Brush(HBRUSH(x).into()),
            }
        }
    }
}
impl WindowClass {
    pub fn is_invalid(&self) -> bool {
        self.name.is_none() && self.atom.is_null()
    }
}
impl Default for WindowClass {
    fn default() -> Self {
        Self {
            name: None, 
            atom: PCWSTR::null(),
            handle_instance: None,
        }
    }
}
// impl std::fmt::Display for WindowClass {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if self.name.is_null() {
//             write!(f, "WindowClass(<NULL>)",)
//         } else {
//             write!(
//                 f,
//                 "WindowClass({})",
//                 unsafe { self.name.to_string() }.unwrap_or(String::from("<FromUtf16Error>"))
//             )
//         }
//     }
// }
// impl Drop for WindowClass {
//     fn drop(&mut self) {
//         if self.is_invalid() {
//             return;
//         }
//         if  {
//             unsafe {
//                 UnregisterClassW(self.name, self.handle_instance);
//             }
//         } else {
//             unsafe {
//                 UnregisterClassW(self.atom, self.handle_instance);
//             }
//         }
//     }
// }
impl Drop for WindowClass {
    fn drop(&mut self) {
        if !self.atom.is_null() {
            unsafe {
                UnregisterClassW(self.atom, self.handle_instance);
            }
        } else if let Some(x) = &self.name {
            unsafe {
                UnregisterClassW(x.0, self.handle_instance);
            }
        } else {
            return;
        }
    }
}
///如果窗口类名长度大于255或小于4（以字节为单位，而不是字符或字素）将失败并返回ERROR_SECRET_TOO_LONG
///如果class_extra和window_extra的值大于4，将失败并返回ERROR_NOT_ENOUGH_MEMORY
impl WindowClass {
    pub fn register(
        class_name: &str,
        style: WindowClassStyle,
        default_menu_resource: Option<Either<&str, usize>>,
        icon: Option<Icon>,
        icon_small: Option<Icon>,
        cursor: Option<Cursor>,
        background_brush: Option<WindowClassP::BrushC>,
        class_extra: u8,
        window_extra: u8,
    ) -> Result<Self> {
        if class_name.len() < 4 || class_name.len() > 255 {
            return Err(Error::new(ERROR_SECRET_TOO_LONG.into(), ""));
        }
        if class_extra > 4 || window_extra > 4 {
            return Err(Error::new(ERROR_NOT_ENOUGH_MEMORY.into(), ""));
        }
        let (class_name, classddd) = str_to_pcwstr(class_name);
        let hInstance = unsafe { GetModuleHandleW(PCWSTR::null()) }?.into();
        let background_brush = match background_brush {
            None => HBRUSH(NULL_PTR()),
            Some(x) => x.into(),
        };
        let (dmr, dmr_ptr) = _po_to_pcwstr(default_menu_resource);
        let result = unsafe {
            RegisterClassExW(&WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: style.into(),
                lpfnWndProc: Some(window_proc),
                cbClsExtra: class_extra as i32 * 8,
                cbWndExtra: window_extra as i32 * 8,
                hInstance,
                hIcon: icon.unwrap_or(Icon::invalid()).into(),
                hCursor: cursor.unwrap_or(Cursor::null()).handle,
                hbrBackground: background_brush,
                lpszMenuName: dmr,
                lpszClassName: class_name,
                hIconSm: icon_small.unwrap_or(Icon::invalid()).into(),
            })
        };
        if result == 0 {
            return Err(Error::from_win32());
        };
        Ok(Self {
            name: Some((class_name, classddd)),
            atom:  PCWSTR(result as *mut u16),
            handle_instance: Some(hInstance),
        })
    }
    fn get(&self) -> PCWSTR {
        if !self.atom.is_null() {
            unsafe {
                return self.atom;
            }
        } else if let Some(x) = &self.name {
            unsafe {
                return x.0;
            }
        } else {
            return self.atom;
        }
    }
    pub fn create_window(
        &self,
        name: &str,
        wtype: WindowType,
        pos: Option<RectangleWH>,
        msgr: CallBackObj,
    ) -> Result<Window> {
        let (style, ex_style, menu, parent) = wtype.into();
        let (wname, wnameptr) = str_to_pcwstr(name);
        let cname = self.get();
        let ((x, y), width, height) = match pos {
            None => ((CW_USEDEFAULT, CW_USEDEFAULT), CW_USEDEFAULT, CW_USEDEFAULT),
            Some(x) => x,
        };
        let hInstance = unsafe { GetModuleHandleW(PCWSTR::null())? }.into();
        let ptr = Box::into_raw(Box::new(msgr)) as usize;
        let mut result = Window {
            handle: unsafe {
                CreateWindowExW(
                    ex_style,
                    cname,
                    wname,
                    style,
                    x,
                    y,
                    width,
                    height,
                    parent,
                    menu,
                    Some(hInstance),
                    Some(ptr as *const c_void),
                )?
            },
        };
        Ok(result)
    }
}
