use super::*;
pub enum EditType {
    Normal,
    MultiLine,
    ///None对应`●`(新版系统)或`*`(旧版系统)
    Password(Option<char>),
    //Rich,
}
pub enum EditTempleType {
    Normal,
    MultiLine,
    ///如需指定自定义字符，需要在运行时手动调用set_passwrd_char指定
    Password,
    //Rich,
}
pub enum CaseType {
    Normal,
    Number, // ES_NUMBER
    Lower,  // ES_LOWERCASE
    Upper,  // ES_UPPERCASE
}
pub struct EditOption<T> {
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
    pub etype: T,
}
pub type EditStyle = EditOption<EditType>;
impl<T> EditOption<T> {
    fn p_into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE, T, String) {
        let (mut edit_style, ex) = self.style.into();
        set_style(
            &mut edit_style,
            ES_AUTOHSCROLL as WINDOW_STYLE,
            self.auto_hscroll,
        );
        set_style(
            &mut edit_style,
            ES_AUTOVSCROLL as WINDOW_STYLE,
            self.auto_vscroll,
        );
        set_style(&mut edit_style, ES_CENTER as WINDOW_STYLE, self.center);
        set_style(
            &mut edit_style,
            ES_NOHIDESEL as WINDOW_STYLE,
            self.nohide_sel,
        );
        set_style(
            &mut edit_style,
            ES_OEMCONVERT as WINDOW_STYLE,
            self.oem_convert,
        );
        set_style(&mut edit_style, ES_READONLY as WINDOW_STYLE, self.readonly);
        set_style(&mut edit_style, ES_RIGHT as WINDOW_STYLE, self.right);
        set_style(
            &mut edit_style,
            ES_WANTRETURN as WINDOW_STYLE,
            self.want_return,
        );
        use CaseType::*;
        match self.case_type {
            Normal => (),
            Number => edit_style |= ES_NUMBER as WINDOW_STYLE,
            Lower => edit_style |= ES_LOWERCASE as WINDOW_STYLE,
            Upper => edit_style |= ES_UPPERCASE as WINDOW_STYLE,
        }
        (edit_style, ex, self.etype, self.text)
    }
}
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE, Option<char>, String)> for EditStyle {
    fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE, Option<char>, String) {
        let mut pass: Option<char> = None;
        let (mut edit_style, ex, etype, text) = self.p_into();
        use EditType::*;
        match etype {
            Normal => (),
            MultiLine => {
                edit_style |= ES_MULTILINE as WINDOW_STYLE;
            }
            Password(c) => {
                edit_style |= ES_PASSWORD as WINDOW_STYLE;
                pass = c
            } // Rich => {
              //     todo!()
              // },
        }

        (edit_style, ex, pass, text)
    }
}
pub type EditTemple = EditOption<EditTempleType>;
impl DialogTempleControl for EditTemple {
    fn pre_compile(self, pos: Point, size: Size, identifier: WindowID) -> ControlPreCompilePruduct {
        let (mut ms_style, ex, etype, ct) = self.p_into();
        use EditTempleType::*;
        match etype {
            Normal => (),
            MultiLine => {
                ms_style |= ES_MULTILINE as WINDOW_STYLE;
            }
            Password => {
                ms_style |= ES_PASSWORD as WINDOW_STYLE;
            } // Rich => {
              //     todo!()
              // },
        };
        ControlPreCompilePruduct::from(format!(
            "CONTROL \"{}\", {}, \"Edit\", 0x{:04X}, {}, {}, {}, {}, 0x{:04X}",
            ct, identifier, ms_style, pos.x, pos.y, size.width, size.height, ex
        ))
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
            case_type: CaseType::Normal,
            etype: EditType::Normal,
        }
    }
}
impl CommonControl for Edit {
    type Style = EditStyle;
    fn new(
        wnd: &mut Window,
        pos: Option<Rect>,
        identifier: WindowID,
        control_style: Self::Style,
        font: Option<ControlFont>,
    ) -> Result<Self> {
        let (a, b, pass, text) = control_style.into();
        let mut result = Self(new_control(
            wnd,
            w!("Edit"),
            text,
            pos,
            identifier,
            a,
            b,
            font,
        )?);
        match pass {
            None => (), //不要直接传给set_passwrd_char，表达的含义不一样
            Some(s) => result.set_passwrd_char(Some(s))?,
        };
        Ok(result)
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
