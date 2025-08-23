use super::*;
pub use capdows::positioning::FontPoint;
pub use capdows::positioning::FontSize;
pub use capdows::ui::dialog::DialogTempleControl;
pub use capdows::ui::font::FontCharSet;
pub use capdows::ui::help::HelpId;
pub use capdows::ui::style::{ChildWindowStyles, NormalWindowStyles};
pub use capdows::ui::window::WindowID;
use windows_sys::Win32::UI::WindowsAndMessaging::*;
pub struct DialogTempleFont {
    /// 字体名称（最多 30 个字符）。
    ///
    ///None表示系统自动设置
    pub face_name: Option<&'static str>, //None => MS Shell Dlg + DS_SHELLFONT
    /// 字体大小
    pub size: i32,
    /// 字符集。
    pub char_set: FontCharSet,
    /// 是否为斜体。
    pub italic: bool,
    /// 字体粗细（0 ~ 1000）。默认400
    pub weight: i32,
}
impl Default for DialogTempleFont {
    fn default() -> Self {
        Self {
            face_name: None,
            size: 9,
            char_set: FontCharSet::default(),
            italic: false,
            weight: 400,
        }
    }
}
pub struct DialogTemple {
    pub pos: Option<FontPoint>,
    pub size: FontSize,
    pub style: DialogStyles,
    pub dtype: DialogTempleType,
    pub caption: String,
    pub class_name: Option<String>,
    pub font: DialogTempleFont,
    pub menu: Option<ResourceID>,
    pub language: Option<LangID>,
    pub help_id: Option<HelpId>,
    controls: String,
}
impl DialogTemple {
    pub fn new(size: FontSize, dtype: DialogTempleType) -> Self {
        Self {
            pos: None,
            size,
            style: DialogStyles::default(),
            dtype,
            caption: String::new(),
            class_name: None,
            font: DialogTempleFont::default(),
            menu: None,
            language: None,
            help_id: None,
            controls: String::new(),
        }
    }

    #[inline]
    pub fn append_control<C: DialogTempleControl>(
        &mut self,
        control: C,
        pos: FontPoint,
        size: FontSize,
        identifier: WindowID,
    ) {
        self.controls = format!(
            "{}{}\n",
            self.controls,
            control.pre_compile(pos, size, identifier)
        );
    }
    #[inline]
    pub unsafe fn get_controls_raw(&self) -> &str {
        &self.controls
    }
    #[inline]
    pub unsafe fn get_controls_raw_mut(&mut self) -> &mut str {
        &mut self.controls
    }
    pub fn pre_compile(self, id: ResourceID) -> PreCompilePruduct {
        let (mut style, style_ex) = self.dtype.into();
        style |= <DialogStyles as Into<WINDOW_STYLE>>::into(self.style);
        let font_name = match self.font.face_name {
            Some(x) => do_escapes(x),
            None => {
                style |= (DS_SETFONT | DS_FIXEDSYS) as u32;
                "MS Shell Dlg".to_string()
            }
        };
        let pos = self
            .pos
            .unwrap_or(FontPoint::new(CW_USEDEFAULT, CW_USEDEFAULT));
        PreCompilePruduct::from(format!(
            "
{} DIALOGEX {}, {}, {}, {}, {}
STYLE 0x{:04X}
EXSTYLE 0x{:04X}
CAPTION \"{}\"{}{}{}FONT {}, \"{}\", {}, {}, {:04X}
{{
{}
}}",
            pre_compile_resource_id(id).get(),
            pos.x,
            pos.y,
            self.size.width,
            self.size.height,
            match self.help_id {
                None => 0,
                Some(help_id) => help_id.get(),
            },
            style,
            style_ex,
            do_escapes(&self.caption),
            match self.menu {
                Some(StringId(y)) => {
                    let result = y.to_string();
                    check_res_id(&result);
                    format!("\nMENU \"{}\"", do_escapes(&result))
                }
                Some(NumberId(x)) => format!("\nMENU {x}"),
                None => "".to_string(),
            },
            match self.class_name {
                None => "".to_string(),
                Some(x) => format!("\nCLASS  \"{}\"", do_escapes(&x)),
            },
            pre_compile_lang_id(self.language).get(),
            self.font.size,
            font_name,
            self.font.weight,
            self.font.italic as u8,
            self.font.char_set as u8,
            self.controls,
        ))
    }
}
pub enum DialogTempleType {
    Overlapped {
        style: NormalWindowStyles,
        is_layered: bool, //WS_EX_LAYERED
    }, //重叠窗口
    Popup {
        style: NormalWindowStyles,
        is_layered: bool, //WS_EX_LAYERED
    },
    Child {
        style: ChildWindowStyles,
        is_layered: bool, //WS_EX_LAYERED
    },
}
impl Default for DialogTempleType {
    fn default() -> Self {
        Self::Overlapped {
            style: Default::default(),
            is_layered: false,
        }
    }
}
impl From<DialogTempleType> for (WINDOW_STYLE, WINDOW_EX_STYLE) {
    fn from(val: DialogTempleType) -> Self {
        use DialogTempleType::*;
        match val {
            Overlapped { style, is_layered } => {
                let (style, mut style_ex) = style.into();
                if is_layered {
                    style_ex |= WS_EX_LAYERED;
                };
                (style, style_ex)
            }
            Popup { style, is_layered } => {
                let (style, mut style_ex) = style.into();
                if is_layered {
                    style_ex |= WS_EX_LAYERED;
                };
                (style | WS_POPUP, style_ex)
            }
            Child { style, is_layered } => {
                let (style, mut style_ex) = style.into();
                if is_layered {
                    style_ex |= WS_EX_LAYERED;
                };
                (style, style_ex)
            }
        }
    }
}
#[derive(Clone, PartialEq, Copy, Debug, Default)]
#[repr(C, packed)]
/// DS_CONTEXTHELP 请用替代 [`capdows::ui::style::WindowContextBarButton::Help`]
/// 未包含DS_FIXEDSYS、DS_USEPIXELS。
pub struct DialogStyles {
    /// 将对话框在其所有者的监视器工作区中居中。
    ///
    /// 如果没有所有者，则在系统选择的监视器中居中。
    pub center: bool, // DS_CENTER

    /// 对话框创建时将自动其置于前台。
    ///
    ///相当于调用Window上的set_foreground方法。
    pub set_foreground: bool, // DS_SETFOREGROUND

    /// 是否**禁止**对话框创建时自动递归设置子窗口的字体。
    ///
    /// 相当于CommonControl的new函数的最后一个参数
    pub no_set_font: bool, // DS_SETFONT

    /// 允许对话框在鼠标光标位置居中。
    pub center_mouse: bool, // DS_CENTERMOUSE

    /// 创建一个可作为子窗口使用的对话框（如属性页）。
    ///
    /// 允许 Tab 切换、快捷键等。
    pub control_like: bool, // DS_CONTROL

    /// 禁止发送 WM_ENTERIDLE 消息给所有者。
    pub no_idle_msg: bool, // DS_NOIDLEMSG

    /// 即使创建子控件失败也继续创建对话框。
    pub no_fail_create: bool, // DS_NOFAILCREATE

    ///使用模式对话框框架创建一个对话框
    pub modalfame: bool, //DS_MODALFRAME

    /// 坐标为屏幕坐标。
    #[deprecated(note = "Use relative layout instead of hard-coded screen coordinates")]
    pub abs_align: bool, // DS_ABSALIGN
}
impl DialogStyles {
    pub fn set_modalfame(mut self) -> Self {
        self.modalfame = true;
        self
    }
}
impl From<DialogStyles> for WINDOW_STYLE {
    #[allow(deprecated)]
    #[inline]
    fn from(val: DialogStyles) -> Self {
        ((val.center as i32) * DS_CENTER
            + (val.set_foreground as i32) * DS_SETFOREGROUND
            + ((!val.no_set_font) as i32) * DS_SETFONT
            + (val.center_mouse as i32) * DS_CENTERMOUSE
            + (val.control_like as i32) * DS_CONTROL
            + (val.no_idle_msg as i32) * DS_NOIDLEMSG
            + (val.no_fail_create as i32) * DS_NOFAILCREATE
            + (val.modalfame as i32) * DS_MODALFRAME
            + (val.abs_align as i32) * DS_ABSALIGN) as WINDOW_STYLE
    }
}
