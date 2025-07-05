use super::*;
use capdows::ui::font::FontCharSet;
use capdows::ui::help::HelpId;
use capdows::ui::window::WindowID;
use windows::Win32::UI::WindowsAndMessaging::*;
///由DialogTempleControl的pre_compile方法得到
pub type ControlPreCompilePruduct = PreCompilePruduct;
//CONTROL <content>, <id>, "<class>", <style>, <x>, <y>, <w>, <h>, <ex_style>
pub trait DialogTempleControl {
    fn pre_compile(
        self,
        pos: Point,
        size: Size,
        identifier: WindowID,
    ) -> ControlPreCompilePruduct;
}
pub struct DialogTempleFont {
    /// 字体名称（最多 30 个字符）。
    pub face_name: &'static str,
    /// 字体大小
    pub size: i32,
    /// 字符集。
    pub char_set: FontCharSet,
    /// 是否为斜体。
    pub italic: bool,
    /// 字体粗细（0 ~ 1000）。
    pub weight: Option<i32>,
}
pub struct DialogTemple {
    pos: Point,
    size: Size,
    dtype: DialogTempleType,
    caption: String,
    class_name: Option<String>,
    font: DialogTempleFont,
    menu: Option<ResourceID>,
    language: Option<LangID>,
    help_id: Option<HelpId>,
    /// 可以手动编写，也可以使用DialogTempleControl
    controls: Vec<ControlPreCompilePruduct>,
}
impl DialogTemple {
    pub fn pre_compile(self, id: ResourceID) -> Result<PreCompilePruduct> {
        let (style, style_ex) = self.dtype.into();
        Ok(PreCompilePruduct::from(format!(
            "
{} DIALOGEX {}, {}, {}, {}, {}
STYLE 0x{:04X}
EXSTYLE 0x{:04X}
CAPTION \"{}\"{}{}{}FONT {}, \"{}\", {}, {}, {:04X}
{{
{}
}}",
            pre_compile_resource_id(id)?.get(),
            self.pos.x,
            self.pos.y,
            self.size.width,
            self.size.height,
            match self.help_id {
                None => 0,
                Some(help_id) => help_id.get(),
            },
            style.0,
            style_ex.0,
            self.caption,
            match self.menu {
                Some(StringId(y)) => {
                    let result = y.to_string();
                    if result.parse::<f32>().is_ok() {
                        return Err(ERROR_INVALID_STRING_ID);
                    };
                    format!("\nMENU \"{}\"", result)
                }
                Some(NumberId(x)) => format!("\nMENU {}", x),
                None => "".to_string(),
            },
            match self.class_name {
                None => "".to_string(),
                Some(x) => format!("\nCLASS  \"{}\"", x),
            },
            pre_compile_lang_id(self.language).get(),
            self.font.size,
            self.font.face_name,
            self.font.weight.unwrap_or(400),
            self.font.italic as u8,
            self.font.char_set as u8,
            self.controls
                .into_iter()
                .map(|x| x.get())
                .collect::<Vec<_>>()
                .join("\n"),
        )))
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
                (style | WS_CHILD, style_ex)
            }
        }
    }
}
