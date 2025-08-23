use super::*;
pub enum EditType {
    Normal,
    MultiLine,
    ///默认`●`(新版系统)或`*`(旧版系统)
    Password,
    //Rich,
}
pub enum CaseType {
    DefaultCase,
    Number, // ES_NUMBER
    Lower,  // ES_LOWERCASE
    Upper,  // ES_UPPERCASE
}
pub struct EditStyle {
    //AI
    pub text: String,
    pub style: ChildWindowStyles,
    pub auto_hscroll: bool, // ES_AUTOHSCROLL
    pub auto_vscroll: bool, // ES_AUTOVSCROLL
    pub center: bool,       // ES_CENTER
    pub nohide_sel: bool,   // ES_NOHIDESEL
    pub oem_convert: bool,  // ES_OEMCONVERT
    pub readonly: bool,     // ES_READONLY
    pub right: bool,        // ES_RIGHT
    pub want_return: bool,  // ES_WANTRETURN
    pub case_type: CaseType,
    pub etype: EditType,
}
impl From<EditStyle> for ((WINDOW_STYLE, WINDOW_EX_STYLE), String) {
    fn from(val: EditStyle) -> Self {
        let (mut edit_style, ex) = val.style.into();
        set_style(
            &mut edit_style,
            ES_AUTOHSCROLL as WINDOW_STYLE,
            val.auto_hscroll,
        );
        set_style(
            &mut edit_style,
            ES_AUTOVSCROLL as WINDOW_STYLE,
            val.auto_vscroll,
        );
        set_style(&mut edit_style, ES_CENTER as WINDOW_STYLE, val.center);
        set_style(
            &mut edit_style,
            ES_NOHIDESEL as WINDOW_STYLE,
            val.nohide_sel,
        );
        set_style(
            &mut edit_style,
            ES_OEMCONVERT as WINDOW_STYLE,
            val.oem_convert,
        );
        set_style(&mut edit_style, ES_READONLY as WINDOW_STYLE, val.readonly);
        set_style(&mut edit_style, ES_RIGHT as WINDOW_STYLE, val.right);
        set_style(
            &mut edit_style,
            ES_WANTRETURN as WINDOW_STYLE,
            val.want_return,
        );
        use CaseType::*;
        match val.case_type {
            DefaultCase => (),
            Number => edit_style |= ES_NUMBER as WINDOW_STYLE,
            Lower => edit_style |= ES_LOWERCASE as WINDOW_STYLE,
            Upper => edit_style |= ES_UPPERCASE as WINDOW_STYLE,
        }

        use EditType::*;
        match val.etype {
            Normal => (),
            MultiLine => {
                edit_style |= ES_MULTILINE as WINDOW_STYLE;
            }
            Password => {
                edit_style |= ES_PASSWORD as WINDOW_STYLE;
            } // Rich => {
              //     todo!()
              // },
        }

        ((edit_style, ex), val.text)
    }
}
pub type EditTemple = EditStyle;
impl DialogTempleControl for EditTemple {
    #[inline]
    fn pre_compile(self, pos: FontPoint, size: FontSize, identifier: WindowID) -> String {
        let ((ms_style, ex), ct) = self.into();
        format!(
            "CONTROL \"{}\", {}, \"Edit\", 0x{:04X}, {}, {}, {}, {}, 0x{:04X}",
            ct, identifier, ms_style, pos.x, pos.y, size.width, size.height, ex
        )
    }
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
    ///WM_CTLCOLOREDIT消息
    Colour(usize),
}
define_control! {
    Edit,
    "Edit",
    {
        match code {
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
                WM_CTLCOLOREDIT => {
                    let nmhdr = (*(ptr as *mut NMHDRCOLOR)).DC;
                    Colour(nmhdr as usize)
                }
                _ => return Err(ERROR_MSG_CODE_NOT_SUPPORT),
            }
    },
    {
        is_some_window(wnd, L!("Edit"))
    },
    {
        todo!()
    }
}
impl EditStyle {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            style: ChildWindowStyles {
                style: NormalWindowStyles {
                    edge_type: WindowEdgeType::Sunken,
                    border_type: WindowBorderType::NoBorder,
                    visible: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            auto_hscroll: true, // ES_AUTOHSCROLL
            auto_vscroll: true, // ES_AUTOVSCROLL
            center: false,      // ES_CENTER
            nohide_sel: false,  // ES_NOHIDESEL
            oem_convert: false, // ES_OEMCONVERT
            readonly: false,    // ES_READONLY
            right: false,       // ES_RIGHT
            want_return: true,  // ES_WANTRETURN
            case_type: CaseType::DefaultCase,
            etype: EditType::Normal,
        }
    }
}
impl CommonControl for Edit {
    type Style = EditStyle;
    #[inline]
    fn new_raw(
        wnd: &mut Window,
        pos: Option<Rect>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<HWND> {
        let (a, text) = control_style.into();
        new_control(wnd, w!("Edit"), text, pos, identifier, a, font)
    }
}

impl TextControl for Edit {}
impl Edit {
    pub fn set_passwrd_char(&mut self, pw_char: Option<char>) -> Result<()> {
        let num = match pw_char {
            Some(x) => {
                if !x.is_ascii() {
                    return Err(ERROR_NOT_SUPPORTED);
                }
                let mut b = [0; 4];
                x.encode_utf8(&mut b);
                b[0] as usize
            }
            None => 0usize,
        };
        unsafe {
            SendMessageW(
                self.0.handle(),
                EM_SETPASSWORDCHAR,
                num as WPARAM,
                0 as LPARAM,
            )
        };
        Ok(())
    }
    pub fn get_passwrd_char(&mut self) -> Result<char> {
        match char::from_u32(unsafe {
            SendMessageW(
                self.0.handle(),
                EM_GETPASSWORDCHAR,
                0 as WPARAM,
                0 as LPARAM,
            )
        } as u32)
        {
            Some(x) => Ok(x),
            None => Err(ERROR_NO_UNICODE_TRANSLATION),
        }
    }
}
