use crate::PreCompilePruduct;
use crate::pre_compile_lang_id;
use crate::pre_compile_resource_id;
use capdows::i18n::LangID;
use capdows::win32::Result;
use capdows::win32::core::ResourceID;
pub use capdows::win32::help::HelpId;
pub use capdows::win32::menu::MenuItemID;
pub use capdows::win32::menu::MenuItemStyle;
///菜单模板只能定义OwnDraw与字符串项或分隔符，位图类型的项需要在运行时添加
// menuID MENUEX
// LANGUAGE LANG_NEUTRAL, SUBLANG_NEUTRAL
// {
// 		popupBody
// }
pub struct MenuTemplate {
    pub language: Option<LangID>,
    pub items: Vec<MenuTemplateItem>,
}
// popupBody:
// [
// MENUITEM itemText, id, type, state
// |
// POPUP itemText, 0, type, state, HelpId {
// 		popupBody
// }
// ] ...
pub enum MenuTemplateItem {
    Item {
        content: String,
        id: MenuItemID,
        style: MenuItemStyle,
    },
    Child {
        content: String,
        items: Vec<MenuTemplateItem>,
        style: MenuItemStyle,
        help_id: Option<HelpId>,
    },
    Separator,
}
impl MenuTemplateItem {
    pub fn pre_compile(self) -> Result<PreCompilePruduct> {
        Ok(PreCompilePruduct::from(match self {
            MenuTemplateItem::Item { content, id, style } => {
                let (mtype, state) = style.into();
                format!("MENUITEM \"{}\", {}, {}, {}", content, id, mtype.0, state.0)
            }
            MenuTemplateItem::Separator => String::from("MENUITEM \"\", 0, 0x800, 0"),
            MenuTemplateItem::Child {
                content,
                items,
                style,
                help_id,
            } => {
                let (mtype, state) = style.into();
                format!(
                    "POPUP \"{}\", 0, {}, {}, {} \n{{\n{}\n}}",
                    content,
                    mtype.0,
                    state.0,
                    match help_id {
                        None => 0,
                        Some(help_id) => help_id.get(),
                    },
                    items
                        .into_iter()
                        .map(|i| {
                            Ok(i.pre_compile()?
                                .get()
                                .lines()
                                .map(|line| format!("  {}", line))
                                .collect::<Vec<_>>()
                                .join("\n"))
                        })
                        .collect::<Result<Vec<_>>>()?
                        .join("\n")
                )
            }
        }))
    }
}
impl MenuTemplate {
    pub fn pre_compile(self, id: ResourceID) -> Result<PreCompilePruduct> {
        Ok(PreCompilePruduct::from(format!(
            "{} MENUEX{}{{\n{}\n}}",
            pre_compile_resource_id(id)?.get(),
            pre_compile_lang_id(self.language).get(),
            self.items
                .into_iter()
                .map(|i| {
                    Ok(i.pre_compile()?
                        .get()
                        .lines()
                        .map(|line| format!("  {}", line))
                        .collect::<Vec<_>>()
                        .join("\n"))
                })
                .collect::<Result<Vec<_>>>()?
                .join("\n")
        )))
    }
}
