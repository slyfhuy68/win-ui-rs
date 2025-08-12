use super::*;
pub struct WindowClassBuilder {
    class_name: &'static CWideStr,
    style: WindowClassStyle,
    executable_file: Option<ExecutableFile>,
    default_menu_resource: Option<ResourceID>,
    icon: Option<Icon>,
    icon_small: Option<Icon>,
    cursor: Option<Cursor>,
    background_brush: Option<ClassBackgroundBrush>,
    class_extra: ExtraMemory,
    window_extra: ExtraMemory,
}
impl WindowClassBuilder {
    #[inline]
    pub fn new(class_name: &'static CWideStr) -> Self {
        WindowClassBuilder {
            class_name,
            style: WindowClassStyle::default(),
            executable_file: None,
            default_menu_resource: None,
            icon: None,
            icon_small: None,
            cursor: None,
            background_brush: Some(ClassBackgroundBrush::default()),
            class_extra: ExtraMemory::default(),
            window_extra: ExtraMemory::default(),
        }
    }
    #[inline]
    pub const fn style(mut self, style: WindowClassStyle) -> Self {
        self.style = style;
        self
    }
    #[inline]
    pub const fn default_menu(mut self, res: ResourceID) -> Self {
        self.default_menu_resource = Some(res);
        self
    }
    #[inline]
    pub const fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }
    #[inline]
    pub const fn small_icon(mut self, icon_small: Icon) -> Self {
        self.icon_small = Some(icon_small);
        self
    }
    #[inline]
    pub const fn cursor(mut self, cursor: Cursor) -> Self {
        self.cursor = Some(cursor);
        self
    }
    #[inline]
    pub const fn background_brush(mut self, brush: Option<ClassBackgroundBrush>) -> Self {
        self.background_brush = brush;
        self
    }
    #[inline]
    pub const fn class_extra(mut self, extra: ExtraMemory) -> Self {
        self.class_extra = extra;
        self
    }
    #[inline]
    pub const fn window_extra(mut self, extra: ExtraMemory) -> Self {
        self.window_extra = extra;
        self
    }
    #[inline]
    pub fn default_cursor(mut self) -> Result<Self> {
        self.cursor = Some(Cursor::from_system(SystemCursor::NormalSelection)?);
        Ok(self)
    }
    #[inline]
    pub const fn executable_file(mut self, e: ExecutableFile) -> Self {
        self.executable_file = Some(e);
        self
    }
    /// 需指定C为消息接收器，一般情况下，使用[`crate::ui::msg::MessageReceiver`]trait来指定消息接收器。
    ///
    /// 所有实现了[`crate::ui::msg::MessageReceiver`]trait的类型都自动实现了[`crate::ui::msg::RawMessageHandler`]trait
    pub fn build<C: RawMessageHandler + Sync + 'static>(
        self,
        _msg_receiver: PhantomData<C>,
    ) -> Result<WindowClass> {
        if self.class_name.len() < 4 || self.class_name.len() >= 255 {
            return Err(ERROR_CLASS_NAME_TOO_LONG);
        }
        let _ = error_from_win32_num!(RegisterClassExW(&WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: self.style.into(),
            //window_proc是一个函数，定义在私有模块cpadows::ui::proc里
            lpfnWndProc: Some(window_proc::<C>),
            cbClsExtra: self.class_extra as i32 * 8,
            cbWndExtra: self.window_extra as i32 * 8,
            hInstance: match self.executable_file {
                Some(x) => x.into(),
                None => ExecutableFile::from_current_file()?.into(),
            },
            hIcon: self.icon.unwrap_or(Icon::null()).into(),
            hCursor: self.cursor.unwrap_or(Cursor::null()).into(),
            hbrBackground: match self.background_brush {
                None => NULL_PTR(),
                Some(x) => x.into(),
            },
            lpszMenuName: match self.default_menu_resource {
                None => 0 as *const u16,
                Some(x) => x.to_pcwstr(),
            },
            lpszClassName: self.class_name.to_pcwstr(),
            hIconSm: self.icon_small.unwrap_or(Icon::null()).into(),
        }) as i32)?;
        Ok(WindowClass {
            name: self.class_name.to_pcwstr(),
        })
    }
}
#[repr(transparent)]
#[derive(Debug)]
pub struct WindowClass {
    pub(crate) name: PCWSTR,
}
unsafe impl Send for WindowClass {}
unsafe impl Sync for WindowClass {}
impl WindowClass {
    pub fn is_invalid(&self) -> bool {
        self.name.is_null()
    }
}
impl Drop for WindowClass {
    fn drop(&mut self) {
        unsafe {
            let _ = UnregisterClassW(self.name, 0 as HINSTANCE);
        }
    }
}
// impl Drop for WindowClass {
//     fn drop(&mut self) {
//         if !(std::thread::panicking() || self.name.is_null()) {
//             println!("debug-class, {:?}", self);
//             println!("Backtrace:\n{}", std::backtrace::Backtrace::capture());
//             println!(
//                 "note: run with `RUST_BACKTRACE=1` or `RUST_BACKTRACE=full` for a verbose backtrace."
//             );
//         }
//     }
// }

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ExtraMemory {
    #[default]
    NoExtraMemory = 0,
    OneExtraMemory = 1,
    DoubleExtraMemory = 2,
    TripleExtraMemory = 3,
    QuadrupleExtraMemory = 4,
}
impl std::convert::TryFrom<u8> for ExtraMemory {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        use ExtraMemory::*;
        Ok(match value {
            0 => NoExtraMemory,
            1 => OneExtraMemory,
            2 => DoubleExtraMemory,
            3 => TripleExtraMemory,
            4 => QuadrupleExtraMemory,
            _ => return Err(ERROR_INVALID_DATA),
        })
    }
}

///如果窗口类名长度大于255或小于4（以字节为单位，而不是字符或字素）将失败并返回ERROR_SECRET_TOO_LONG
///如果class_extra和window_extra的值大于4，将失败并返回ERROR_NOT_ENOUGH_MEMORY
impl WindowClass {
    pub fn from_str(class_name: &'static CWideStr) -> Self {
        Self {
            name: class_name.to_pcwstr(),
        }
    }
    pub fn from_raw(raw: PCWSTR) -> Self {
        Self { name: raw }
    }
    fn get_raw(&self) -> PCWSTR {
        self.name
    }
    pub fn create_window(
        &self,
        name: &str,
        wtype: WindowType<'_>,
        pos: Option<Point>,
        size: Option<Size>,
    ) -> Result<Window> {
        unsafe {
            let (style, ex_style, menu, parent) = wtype.into();
            let (wname, _wnameptr) = str_to_pcwstr(name);
            let cname = self.get_raw();
            let (x, y) = pos
                .unwrap_or(Point::new(CW_USEDEFAULT, CW_USEDEFAULT))
                .to_tuple();
            let (width, height) = size
                .unwrap_or(Size::new(CW_USEDEFAULT, CW_USEDEFAULT))
                .to_tuple();
            let hinstance = error_from_win32!(GetModuleHandleW(0 as *const u16))?;
            let result = Window::from_handle(error_from_win32!(CreateWindowExW(
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
                hinstance,
                0 as *const c_void,
            ))?);

            Ok(result)
        }
    }
}
