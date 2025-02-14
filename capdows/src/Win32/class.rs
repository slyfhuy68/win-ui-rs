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
impl WindowClass {
    pub fn register(
        class_name: &str,
        //fn_window_proc: impl ,创建窗口时传
        style: WindowClassStyle,
        default_menu_resource: impl IntOrName,
        icon: Option<Icon>,
        icon_small: Option<Icon>,
        cursor: Option<Cursor>,
        background_brush: Option<WindowClassP::BrushC>,
        class_extra: i32,
        window_extra: i32,
    ) -> Result<Self> {
        if class_name.len() + 1 <= 5 || class_name.len() + 1 >= 256 {
            return Err(Error::new(ERROR_SECRET_TOO_LONG.into(), ""));
        }
        let (class_name, classddd) = str_to_pcwstr(class_name);
        let hInstance = unsafe { GetModuleHandleW(PCWSTR::null()) }?.into();
        let background_brush = match background_brush {
            None => HBRUSH(NULL_PTR()),
            Some(x) => x.into(),
        };
        unsafe extern "system" fn window_proc(
            hWnd: HWND,
            msg: u32,
            wParam: WPARAM,
            lParam: LPARAM,
        ) -> LRESULT {
            let mut window = Window { handle: hWnd };
            let user_callback_ptr = match window.get_prop(PROC_KEY_NAME) {
                Ok(x) => x as *mut CallBackObj,
                Err(_) => {
                    if msg == WM_NCCREATE  {
                        let mut s = *(lParam.0 as *mut CREATESTRUCTW);
                        let mm = window.set_prop(PROC_KEY_NAME, s.lpCreateParams as usize);
                        s.lpCreateParams as *mut CallBackObj
                    } else {
                        return DefWindowProcW(hWnd, msg, wParam, lParam)
                    }
                    },
            };
            if user_callback_ptr.is_null() {
                return DefWindowProcW(hWnd, msg, wParam, lParam);
            }
            let mut user_callback_s = unsafe { Box::from_raw(user_callback_ptr) };
            // if user_callback_s.0 != PROC_MEMORY_SINGS {
            //     return DefWindowProcW(hWnd, msg, wParam, lParam);
            // };
            let mut c = user_callback_s;
            pub use MessageReceiverError::*;
            let result = {
                let mut w = window;
                match msg {
                    //unimplemented!()
                    //----------------------------------------------------------------------------------
                    // WM_ACTIVATEAPP => {},
                    // WM_CANCELMODE => {},
                    // WM_CHILDACTIVATE => {},
                    // WM_CLOSE => {
                    //     mstch user_callback_s {
                                        //        
                    //     }
                    // },
                    // WM_COMPACTING => {},
                    WM_CREATE => {
                        let mut s = *(lParam.0 as *mut CREATESTRUCTW);
                        let wc = w.get_class();
                        match c.create(&mut w,
                                    &s.lpszName.to_string().unwrap_or(String::from("")),
                                    match wc {Err(_) => WindowClass {name:None,atom:s.lpszClass,handle_instance:None,},Ok(x) => x}, 
                                    ExecutableFile{name:None,handle:Some(HMODULE(s.hInstance.0))},
                                    ((s.x,s.y),s.cx,s.cy),
                                    (
                                        WINDOW_STYLE(s.style as u32), 
                                        s.dwExStyle, 
                                        if s.hMenu.is_invalid() { None } else { Some(s.hMenu) }, 
                                        if s.hwndParent.is_invalid() { None } else { Some(s.hwndParent) }
                                    ).into(),
                                ) {
                            Ok(x) => match x {
                                true => 0isize,
                                false => -1isize,
                            },
                            Err(NoProcessed) => DefWindowProcW(hWnd, msg, wParam, lParam).0,
                            Err(x) => x.code() as isize,
                        }
                    },
                    WM_DESTROY => {
                        return LRESULT(match c.destroy(&mut w) {
                            Ok(_) => 0isize, 
                            Err(NoProcessed) => DefWindowProcW(hWnd, msg, wParam, lParam).0,
                            Err(x) => x.code() as isize,
                        });
                    },
                    WM_COMMAND if lParam.0 != 0 => {
                        let lParame = lParam.0;
                        let wParame = wParam.0;
                        let mut nmhdr = NMHDR {
                            hwndFrom: HWND(lParame as *mut c_void), 
                            idFrom: (wParame & 0xffff) as usize, 
                            code: ((wParame >> 16) & 0xffff) as u32
                        };
                        let nmhdr_ptr: *mut NMHDR = &mut nmhdr;
                        match c.control_message(&mut w, nmhdr_ptr as usize, nmhdr.idFrom as WindowID) {
                            Ok(x) => x, 
                            Err(NoProcessed) => DefWindowProcW(hWnd, msg, wParam, lParam).0,
                            Err(x) => x.code() as isize,
                        }
                    }, 
                    WM_NOTIFYFORMAT => {
                        2isize//NFR_UNICODE
                    }, 
                    WM_NOTIFY => {
                        let nmhdr_ptr = lParam.0 as *mut NMHDR;
                        match c.control_message(&mut w, nmhdr_ptr as usize, (*nmhdr_ptr).idFrom as WindowID) {
                            Ok(x) => x, 
                            Err(NoProcessed) => DefWindowProcW(hWnd, msg, wParam, lParam).0,
                            Err(x) => x.code() as isize,
                        }
                    }, 
                    // WM_DPICHANGED => {},
                    // WM_ENABLE => {},
                    // WM_ENTERSIZEMOVE => {},
                    // WM_EXITSIZEMOVE => {},
                    // WM_GETICON => {},
                    // WM_GETMINMAXINFO => {},
                    // WM_INPUTLANGCHANGE => {},
                    // WM_INPUTLANGCHANGEREQUEST => {},
                    // WM_MOVE => {},
                    // WM_MOVING => {},
                    // WM_NCACTIVATE => {},
                    // WM_NCCALCSIZE => {},
                    // WM_NCCREATE => {},
                    // WM_NCDESTROY => {},
                    // WM_NULL => {},
                    // WM_QUERYDRAGICON => {},
                    // WM_QUERYOPEN => {},
                    // WM_SHOWWINDOW => {},
                    // WM_SIZE => {},
                    // WM_SIZING => {},
                    // WM_STYLECHANGED => {},
                    // WM_STYLECHANGING => {},
                    // WM_THEMECHANGED => {},
                    // WM_USERCHANGED => {},
                    // WM_WINDOWPOSCHANGED => {},
                    // WM_WINDOWPOSCHANGING => {},
                    //----------------------------------------------------------------------------------
                    _ => {
                        DefWindowProcW(hWnd, msg, wParam, lParam).0
                        },
                }
            };
            Box::into_raw(c);
            LRESULT(result)
        }
        let result = unsafe {
            RegisterClassExW(&WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: style.into(),
                lpfnWndProc: Some(window_proc),
                cbClsExtra: class_extra,
                cbWndExtra: window_extra,
                hInstance,
                hIcon: icon.unwrap_or(Icon::invalid()).into(),
                hCursor: cursor.unwrap_or(Cursor::invalid()).into(),
                hbrBackground: background_brush,
                lpszMenuName: default_menu_resource.to_pcwstr(),
                lpszClassName: class_name,
                hIconSm: icon_small.unwrap_or(Icon::invalid()).into(),
            })
        };
        if result == 0 {
            get_last_error()?;
        };
        Ok(Self {
            name: Some((class_name, classddd)),
            atom: result.to_pcwstr(),
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
        //result.set_prop(PROC_KEY_NAME, ptr);
        Ok(result)
    }
}
