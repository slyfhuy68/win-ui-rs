use super::*;
pub use capdows::ui::font::FontCharSet;
use capdows::ui::help::HelpId;
pub use capdows::ui::style::{ChildWindowStyles, NormalWindowStyles};
use capdows::ui::window::WindowID;
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
    pub pos: FontPoint,
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
use crate::ui::font::FontCharSet;
impl DialogTemple {
    pub fn new(pos: FontPoint, size: FontSize, dtype: DialogTempleType) -> Self {
        Self {
            pos,
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
        control: DialogTempleControl,
        pos: FontPoint,
        size: FontSize,
        identifier: WindowID,
    ) {
        self.controls
            .push_str(&control.pre_compile(pos, size, identifier).get().push("\n"));
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
            self.pos.x,
            self.pos.y,
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
                Some(NumberId(x)) => format!("\nMENU {}", x),
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
            self.controls.join("\n"),
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
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE)> for DialogTempleType {
    fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE) {
        use DialogTempleType::*;
        match self {
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
#[repr(packed)]
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
    /// 相当于[`capdows_controls::traits::CommonControl`]的new函数的最后一个参数
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
impl Into<WINDOW_STYLE> for DialogStyles {
    #[allow(deprecated)]
    #[inline]
    fn into(self) -> WINDOW_STYLE {
        ((self.center as i32) * DS_CENTER
            + (self.set_foreground as i32) * DS_SETFOREGROUND
            + ((!self.no_set_font) as i32) * DS_SETFONT
            + (self.center_mouse as i32) * DS_CENTERMOUSE
            + (self.control_like as i32) * DS_CONTROL
            + (self.no_idle_msg as i32) * DS_NOIDLEMSG
            + (self.no_fail_create as i32) * DS_NOFAILCREATE
            + (self.modalfame as i32) * DS_MODALFRAME
            + (self.abs_align as i32) * DS_ABSALIGN) as WINDOW_STYLE
    }
}
