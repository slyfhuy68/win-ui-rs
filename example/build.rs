extern crate capdows_resource;
// use either::Either;
// use either::Either::*;
use capdows_controls::prelude_build::*;
use capdows_resource::menu::*;
use capdows_resource::string_table::*;
use capdows_resource::*;
use capdows_resource::{image::*, version::*};
use std::collections::HashMap;
use version::LangID as vLangID;
fn main() {
    init_controls();
    let vstr = Version {
        product_internal_version: (0u16, 0u16, 0u16, 1u16),
        file_internal_version: None,
        debug: None,
        pre_release: false,
        pached: false,
        variant: ProductVariant::default(),
        strings: HashMap::from([
            (vLangID::from_hex("0804"), StringInfo::default()),
            // (vLangID::from_hex("0809"), StringInfo::default()),
        ]),
        os: Default::default(),
        ftype: Default::default(),
    }
    .pre_compile();
    let icon1 = Icon(r#".\test_res\ICON1.ico"#.into(), None).pre_compile(NumberId(1));
    let icon2 = Icon(r#".\test_res\ICON2.ico"#.into(), None).pre_compile(NumberId(2));
    let icon3 = Icon(r#".\test_res\ICON3.ico"#.into(), None).pre_compile(NumberId(3));
    let cursor1 = Cursor(r#".\test_res\CURSOR1.cur"#.into(), None).pre_compile(NumberId(4));
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
                        content: "测试3\tCtrl+P".to_string(),
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
    .pre_compile(NumberId(5));
    let st = StringTable {
        language: None,
        strings: HashMap::from([
            (50, "Hello, {1}!".to_string()),
            (51, "Bye, {1}!".to_string()),
            (52, "中文".to_string()),
            (53, "\"𰻞𰻞面\"的{1}字像{2}一样{3}".to_string()),
        ]),
    }
    .pre_compile();
    use dialog::*;
    use euclid::*;
    let my_dialog = DialogTemple {
        pos: point2(0, 0),
        size: size2(250, 100),
        style: DialogStyles::default().set_modalfame(),
        dtype: DialogTempleType::Popup {
            style: Default::default(),
            is_layered: false,
        },
        caption: "abc\t123".to_string(),
        class_name: None,
        font: DialogTempleFont {
            face_name: None,
            size: 9,
            char_set: FontCharSet::default(),
            italic: false,
            weight: Some(400),
        },
        menu: Some(NumberId(5)),
        language: None,
        help_id: None,
        controls: vec![
            PreCompilePruduct::from(
                r#"CONTROL "&OK", 1, BUTTON, 0x50010001, 130, 78, 50, 11"#.to_string(),
            ),
            PreCompilePruduct::from(
                r#"CONTROL "&Cancel", 2, BUTTON, 0x50010001, 187, 78, 50, 11"#.to_string(),
            ),
        ],
    }
    .pre_compile(NumberId(6));
    compile_all!(vstr, icon1, icon2, icon3, cursor1, menu1, st, my_dialog);
}
