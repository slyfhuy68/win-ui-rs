use super::*;
use windows::Win32::Graphics::Gdi::HBRUSH;
#[derive(Clone, PartialEq)]
pub struct WindowClass {
    pub name: Option<(PCWSTR, Vec<u16>)>,
    pub atom: PCWSTR,
    pub handle_instance: Option<HINSTANCE>,
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
                let _ = UnregisterClassW(self.atom, self.handle_instance);
            }
        } else if let Some(x) = &self.name {
            unsafe {
                let _ = UnregisterClassW(x.0, self.handle_instance);
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
        background_brush: Option<BrushC>,
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
        let hinstance = unsafe { GetModuleHandleW(PCWSTR::null()) }?.into();
        let background_brush = match background_brush {
            None => HBRUSH(NULL_PTR()),
            Some(x) => x.__class_into(),
        };
        let (dmr, _dmr_ptr) = _po_to_pcwstr(default_menu_resource);
        let result = unsafe {
            RegisterClassExW(&WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: style.into(),
                lpfnWndProc: Some(window_proc),
                cbClsExtra: class_extra as i32 * 8,
                cbWndExtra: window_extra as i32 * 8,
                hInstance: hinstance,
                hIcon: icon.unwrap_or(Icon::null()).into(),
                hCursor: cursor.unwrap_or(Cursor::null()).handle,
                hbrBackground: background_brush,
                lpszMenuName: dmr,
                lpszClassName: class_name,
                hIconSm: icon_small.unwrap_or(Icon::null()).into(),
            })
        };
        if result == 0 {
            return Err(Error::from_win32());
        };
        Ok(Self {
            name: Some((class_name, classddd)),
            atom: PCWSTR(result as *mut u16),
            handle_instance: Some(hinstance),
        })
    }
    fn get(&self) -> PCWSTR {
        if let Some(x) = &self.name {
            x.0
        } else {
            self.atom
        }
    }
    pub fn create_window(
        &self,
        name: &str,
        wtype: WindowType,
        pos: Option<Rectangle>,
        msgr: CallBackObj,
    ) -> Result<Window> {
        let (style, ex_style, menu, parent) = wtype.into();
        let (wname, _wnameptr) = str_to_pcwstr(name);
        let cname = self.get();
        let (Point(x, y), Size(width, height)) = match pos {
            None => (
                Point(CW_USEDEFAULT, CW_USEDEFAULT),
                Size(CW_USEDEFAULT, CW_USEDEFAULT),
            ),
            Some(x) => x.get_size(),
        };
        let hinstance = unsafe { GetModuleHandleW(PCWSTR::null())? }.into();
        let ptr = Box::into_raw(Box::new(msgr)) as usize;
        let result = Window {
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
                    Some(hinstance),
                    Some(ptr as *const c_void),
                )?
            },
        };
        Ok(result)
    }
}
