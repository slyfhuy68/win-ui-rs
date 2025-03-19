use super::*;
pub struct Edit(HWND);
pub enum EditType {
	Normal, 
	MultiLine, 
	Password(char), 
	//Rich, 
}
pub struct EditStyle {
	//AI
    pub auto_hscroll: bool,  // ES_AUTOHSCROLL
    pub auto_vscroll: bool,  // ES_AUTOVSCROLL
    pub center: bool,        // ES_CENTER
    pub left: bool,          // ES_LEFT
    pub lowercase: bool,     // ES_LOWERCROLL
    pub nohide_sel: bool,    // ES_NOHIDESEL
    pub number: bool,        // ES_NUMBER
    pub oem_convert: bool,   // ES_OEMCONVERT
    pub etype: EditType,        //不是AI
    pub readonly: bool,      // ES_READONLY
    pub right: bool,         // ES_RIGHT
    pub uppercase: bool,     // ES_UPPERCASE
    pub want_return: bool,   // ES_WANTRETURN
}

impl Into<(WINDOW_STYLE, Option<char>)> for EditStyle {
	//AI
    fn into(self) -> WINDOW_STYLE {
        let mut edit_style = WINDOW_STYLE(0u32);
        let mut pass:Option<char>=None;
        if self.auto_hscroll {
            edit_style |= WINDOW_STYLE(ES_AUTOHSCROLL as u32);
        }
        if self.auto_vscroll {
            edit_style |= WINDOW_STYLE(ES_AUTOVSCROLL as u32);
        }
        if self.center {
            edit_style |= WINDOW_STYLE(ES_CENTER as u32);
        }
        if self.left {
            edit_style |= WINDOW_STYLE(ES_LEFT as u32);
        }
        if self.lowercase {
            edit_style |= WINDOW_STYLE(ES_LOWERCASE as u32);
        }
        // if self.multiline {
        //     edit_style |= WINDOW_STYLE(ES_MULTILINE as u32);
        // }
        if self.nohide_sel {
            edit_style |= WINDOW_STYLE(ES_NOHIDESEL as u32);
        }
        if self.number {
            edit_style |= WINDOW_STYLE(ES_NUMBER as u32);
        }
        if self.oem_convert {
            edit_style |= WINDOW_STYLE(ES_OEMCONVERT as u32);
        }
        // if self.password {
        //     edit_style |= WINDOW_STYLE(ES_PASSWORD as u32);
        // }
        if self.readonly {
            edit_style |= WINDOW_STYLE(ES_READONLY as u32);
        }
        if self.right {
            edit_style |= WINDOW_STYLE(ES_RIGHT as u32);
        }
        if self.uppercase {
            edit_style |= WINDOW_STYLE(ES_UPPERCASE as u32);
        }
        if self.want_return {
            edit_style |= WINDOW_STYLE(ES_WANTRETURN as u32);
        }
        use EditType::*;
		match self.etype {     //不是AI
			Normal => (), 
			MultiLine => {edit_style |= WINDOW_STYLE(ES_MULTILINE as u32);}, 
			Password(c) => {edit_style |= WINDOW_STYLE(ES_PASSWORD as u32);pass = Some(c)}, 
		}
        (edit_style, pass)
    }
}
pub struct EditMsg {
    hwnd: HWND,
    pub bm_type: EditMsgType,
}
pub enum EditMsgType {
	///如果系统上安装了双向语言（例如阿拉伯语或希伯来语），则用户可以使用 `CTRL+左SHIFT`（从左到右）和 `Ctrl+右SHIFT`（从右到左）更改Edit控件方向。更改完毕后会收到此消息。
	///true代表改变为从左到右，false代表改变为从右到左。
    DirectionChanged(bool),
    ///使用 `multiline` 样式并通过 [Edit::set_text] 设置文本时，不会收到 `TextChanged` 消息。
    TextChanged,
    ///当滚动条的上下箭头和空白区域被鼠标点击时，将会收到此消息；当键盘事件导致`Edit`控件的视图区域发生更改（例如，按 HOME、END、上下左右箭头）时，也会收到此消息。
    ///true => 垂直方向 false => 水平方向
    Scroll(bool),
    ///当前文本插入超过`Edit`控件的指定字符数时会收到此消息。 文本插入已被截断。
	///当`Edit`控件没有 auto_hscroll 样式且要插入的字符数超过`Edit`控件的宽度时，也会收到此消息。
	///当`Edit`控件没有 auto_vscroll 样式且文本插入产生的总行数会超过`Edit`控件的高度时，也会收到此消息。
    MaxText,
    ///当`Edit`控件失去键盘焦点时，将会收到此消息。
    LoseKeyboardFocus,
    ///当`Edit`控件获得键盘焦点时，将会收到此消息。
    GetKeyboardFocus,
    ///当`Edit`控件无法分配足够的内存来满足特定请求时会收到 `NoEnoughMemory`
    NoEnoughMemory,
    ///当`Edit`即将重新绘制自身时，在显示文本之前，将会收到此消息。 这样就可以根据需要调整编辑`Edit`控件的大小。 
    Update, 
}
impl ControlMsg for EditMsg {
    type ControlType = Edit;
    unsafe fn from_msg(ptr: usize) -> Option<Box<Self>> {
        unsafe {
            let nmhdr = *(ptr as *mut NMHDR);
            let code = nmhdr.code;
            let w = nmhdr.hwndFrom.clone();
            let _ = nmhdr;
            use EditMsgType::*;
            let bmtype = match code {
				EN_ALIGN_LTR_EC => DirectionChanged(true), 
				EN_ALIGN_RTL_EC => DirectionChanged(false), 
				EN_CHANGE => TextChanged, 
				EN_ERRSPACE => NoEnoughMemory, 
				EN_HSCROLL => Scroll(false), 
				EN_VSCROLL => Scroll(true), 
				EN_KILLFOCUS => LoseKeyboardFocus, 
				EN_SETFOCUS => GetKeyboardFocus, 
				EN_MAXTEXT => MaxText, 
				EN_UPDATE => Update, 
                _ => return None,
            };
            Some(Box::new(Self {
                hwnd: w,
                bm_type: bmtype,
            }))
        }
    }
    fn get_control(&self) -> Self::ControlType {
        Edit(self.hwnd)
    }
}
impl Control for Edit {
    type MsgType = EditMsg;
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
    unsafe fn is_self(wnd: &HWND) -> Result<bool> {
        let mut array1 = vec![0u16; 8];
		if unsafe { GetClassNameW(*wnd, &mut array1[..]) } == 0 {
			return Err(Error::from_win32());
		}
		let meunasfe = unsafe { PCWSTR(array1.as_ptr()).to_string()? };
		//println!("{}", meunasfe);
		return Ok(meunasfe == "Edit".to_string());
    }
}

impl Default for EditStyle {
    fn default() -> Self {
        Self {
			  auto_hscroll: false,  // ES_AUTOHSCROLL
			  auto_vscroll: false,  // ES_AUTOVSCROLL
			  center: false,        // ES_CENTER
			  left: false,          // ES_LEFT
			  lowercase: false,     // ES_LOWERCROLL
			  //multiline: false,     // ES_MULTILINE
			  nohide_sel: false,    // ES_NOHIDESEL
			  number: false,        // ES_NUMBER
			  oem_convert: false,   // ES_OEMCONVERT
			  //password: false,      // ES_PASSWORD 不能与multiline组合，未实现限制与multiline组合的功能
			  readonly: false,      // ES_READONLY
			  right: false,         // ES_RIGHT
			  uppercase: false,     // ES_UPPERCASE
			  want_return: false,   // ES_WANTRETURN
			  etype: EditType::Normal,  
        }
    }
}

impl Edit {
    pub fn new(
        wnd: &mut Window,
        name: &str,
        pos: Option<Rectangle>,
        identifier: WindowID,
        control_style: EditStyle,
        style: ChildWindowStyles,
        style_ex: NormalWindowExStyles,
        font: bool,
        no_notify: bool,
    ) -> Result<Self> {
        let (control_style_ms,password) = control_style.into();
        let hwnd = new_control(
			wnd,
			"EDIT",
			name,
			pos,
			identifier,
			style,
			style_ex,
			control_style_ms,
			font,
			no_notify,
		)?;
		let mut result = Edit(hwnd);
		match password {
		    None => (), //不要直接传给set_passwrd_char，表达的含义不一样
		    Some(s) => result.set_passwrd_char(Some(s)) 
		};
        Ok(result)
    }
    // fn can_undo(&self) -> Result<bool>{
    //     if !unsafe { Self::is_self(&self.0) }? {
    //         return Err(Error::new(ERROR_NOT_SUPPORTED.to_hresult(), ""));
    //     };
    //     unsafe { SendMessageW(self.0, EM_CANUNDO, Some(WPARAM(0)), Some(LPARAM(0))).0 }
    //             as usize != 0
    // }
    ///使用AsciiChar::Null禁用密码
    pub fn set_passwrd_char(&mut self, pw_char: Option<char>) -> Result<()>{

       let num = match pw_char {
            Some(x) => {
                if !x.is_ascii() {
                    return Err(Error::new(ERROR_NOT_SUPPORTED.to_hresult(), ""));
                }
                let mut b = [0; 4];
                x.encode_utf8(&mut b);  
                b[0] as usize
            }
            None => 0usize
        };

        if !unsafe { Self::is_self(&self.0) }? {
            return Err(Error::new(ERROR_NOT_SUPPORTED.to_hresult(), ""));
        };
        unsafe { SendMessageW(self.0, EM_SETPASSWORDCHAR, Some(WPARAM(num)), Some(LPARAM(0))).0 };
        Ok(())
    }
    pub fn get_passwrd_char(&mut self, pw_char: Option<char>) -> Result<char>{
        if !unsafe { Self::is_self(&self.0) }? {
            return Err(Error::new(ERROR_NOT_SUPPORTED.to_hresult(), ""));
        };
        match char::from_u32(unsafe { SendMessageW(self.0, EM_GETPASSWORDCHAR, Some(WPARAM(0)), Some(LPARAM(0))).0 } as u32) {
            Some(x) => Ok(x), 
            None => Err(Error::new(ERROR_NO_UNICODE_TRANSLATION.to_hresult(), ""))
        }
    }
    pub fn get_text(&self) -> Result<String> {
        let length =
            unsafe { SendMessageW(self.0, WM_GETTEXTLENGTH, Some(WPARAM(0)), Some(LPARAM(0))).0 }
                as usize;
        if length == 0 {
            if !unsafe { Self::is_self(&self.0) }? {
                return Ok(String::new());
            } else {
                return Err(Error::new(ERROR_NOT_SUPPORTED.to_hresult(), ""));
            };
        };
        let mut buffer: Vec<u16> = vec![0; length + 1];
        unsafe {
            SendMessageW(
                self.0,
                WM_GETTEXT,
                Some(WPARAM(length)),
                Some(LPARAM(buffer.as_mut_ptr() as isize)),
            )
            .0;
        }
        Ok(String::from_utf16_lossy(&buffer[..length]))
    }
    pub fn set_text(&mut self, text: &str) -> Result<()> {
        if !unsafe { Self::is_self(&self.0) }? {
            return Err(Error::new(ERROR_NOT_SUPPORTED.to_hresult(), ""));
        };
        let (text_ptr, _text_u16) = str_to_pcwstr(text);

        if unsafe {
            SendMessageW(
                self.0,
                WM_SETTEXT,
                Some(WPARAM(0)),
                Some(LPARAM(text_ptr.0 as isize)),
            )
        }
        .0 == 0
        {
            return Err(Error::from_win32());
        }
        Ok(())
    }



}
