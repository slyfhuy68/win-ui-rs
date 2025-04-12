use super::*;
use windows::Win32::Graphics::Gdi::HBRUSH;
#[derive(Clone, PartialEq)]
pub struct WindowClass {
    pub name: PCWSTR,
    pub owner: Option<Vec<u16>>,
}
impl WindowClass {
    pub fn is_invalid(&self) -> bool {
        self.name.is_null()
    }
}
impl Drop for WindowClass {
    fn drop(&mut self) {
        unsafe {
            let _ = UnregisterClassW(self.name, None);
        }
    }
}
///如果窗口类名长度大于255或小于4（以字节为单位，而不是字符或字素）将失败并返回ERROR_SECRET_TOO_LONG
///如果class_extra和window_extra的值大于4，将失败并返回ERROR_NOT_ENOUGH_MEMORY
impl WindowClass {
    pub fn register(
        class_name: &str,
        style: WindowClassStyle,
        default_menu_resource: Option<ResourceID>,
        icon: Option<Icon>,
        icon_small: Option<Icon>,
        cursor: Option<Cursor>,
        background_brush: Option<ClassBackgroundBrush>,
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
            Some(x) => x.into(),
        };
        let (dmr, _dmr_ptr) = match default_menu_resource {
            None => (PCWSTR::null(), None),
            Some(x) => x.to_pcwstr(),
        };
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
            name: class_name,
            owner: Some(classddd),
        })
    }
    fn get(&self) -> PCWSTR {
        self.name
    }
    pub fn create_window(
        &self,
        name: &str,
        wtype: WindowType,
        pos: Option<Rectangle>,
        msgr: Box<CallBackObj>,
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
        let ptr = Box::into_raw(Box::new(msgr)) as *mut c_void;
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
