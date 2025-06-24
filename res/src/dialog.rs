use super::*;
use capdows::ui::{
    core::{Point, Size},
    font::ControlFont,
    help::HelpId,
    window::WindowID,
};
use windows::Win32::UI::WindowsAndMessaging::*;
pub type ControlPreCompilePruduct = PreCompilePruduct;
pub trait DialogTempleControl {
    fn pre_compile(
        &mut self,
        pos: Point,
        size: Size,
        identifier: WindowID,
    ) -> ControlPreCompilePruduct;
}
pub struct DialogTemple {
    pos: Point,
    size: Size,
    dtype: DialogTempleType,
    caption: String,
    class_name: Option<String>,
    font: ControlFont,
    menu: Option<ResourceID>,
    language: Option<LangID>,
    help_id: Option<HelpId>,
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
CAPTION \"{}\"{}{}{}FONT 9, \"SEGOE UI\", FW_NORMAL, FALSE, 0
{{
{}
}}",
            pre_compile_resource_id(id)?.get(),
            self.pos.0,
            self.pos.1,
            self.size.0,
            self.size.1,
            match self.help_id {
                None => 0,
                Some(help_id) => help_id.get(),
            },
            style.0,
            style_ex.0,
            self.caption,
            match self.menu {
                Some(StringId(y)) => {
                    if y.parse::<f32>().is_ok() {
                        return Err(ERROR_INVALID_STRING_ID);
                    };
                    format!("\nMENU \"{}\"", y)
                }
                Some(NumberId(x)) => format!("\nMENU {}", x),
                None => "".to_string(),
            },
            match self.class_name {
                None => "".to_string(),
                Some(x) => format!("\nCLASS  \"{}\"", x),
            },
            pre_compile_lang_id(self.language).get(),
            self.controls
                .into_iter()
                .map(|x| x.get())
                .collect::<Vec<_>>()
                .join("\n")
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
