extern crate capdows_resource;
// use either::Either;
// use either::Either::*;
use capdows_resource::menu::*;
use capdows_resource::*;
use capdows_resource::{image::*, version::*};
use std::collections::HashMap;
use version::LangID as vLangID;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vstr = Version {
        product_internal_version: (0u16, 0u16, 0u16, 1u16),
        file_internal_version: None,
        debug: None,
        pre_release: false,
        pached: false,
        variant: ProductVariant::default(),
        strings: HashMap::from([
            (vLangID::new("0804")?, StringInfo::default()),
            // (vLangID::new("0809")?, StringInfo::default()),
        ]),
        os: Default::default(),
        ftype: Default::default(),
    }
    .pre_compile()?;
    let icon1 = Icon("./res/ICON1.ico".into()).pre_compile(NumberId(1))?;
    let icon2 = Icon("./res/ICON2.ico".into()).pre_compile(NumberId(2))?;
    let icon3 = Icon("./res/ICON3.ico".into()).pre_compile(NumberId(3))?;
    let cursor1 = Cursor("./res/CURSOR1.cur".into()).pre_compile(NumberId(4))?;
    let menu1 = MenuTemplate {
        language: None,
        items: vec![
            MenuTemplateItem::Item {
                content: "测试1".to_string(),
                id: 11,
                style: MenuItemStyle::default(),
            },
            MenuTemplateItem::Child {
                content: "测试2".to_string(),
                style: MenuItemStyle::default(),
                help_id: None,
                items: vec![
                    MenuTemplateItem::Item {
                        content: "测试3".to_string(),
                        id: 12,
                        style: MenuItemStyle::default(),
                    },
                    MenuTemplateItem::Separator,
                    MenuTemplateItem::Child {
                        content: "测试4".to_string(),
                        style: MenuItemStyle::default(),
                        help_id: None,
                        items: vec![
                            MenuTemplateItem::Item {
                                content: "测试5".to_string(),
                                id: 13,
                                style: MenuItemStyle::default(),
                            },
                            MenuTemplateItem::Item {
                                content: "测试6".to_string(),
                                id: 14,
                                style: MenuItemStyle::default(),
                            },
                        ],
                    },
                ],
            },
        ],
    }
    .pre_compile(NumberId(5))?;
    compile_all!(vstr, icon1, icon2, icon3, cursor1, menu1)?;
    Ok(())
}
