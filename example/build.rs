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
    let mut my_dialog = DialogTemple::new(
        point2(0, 0),
        size2(250, 100),
        DialogTempleType::Popup {
            style: NormalWindowStyles {
                visible: true,
                ..Default::default()
            },
            is_layered: false,
        },
    );
    my_dialog.style = DialogStyles::default().set_modalfame();
    my_dialog.caption = "abc\t123".to_string();
    my_dialog.font = DialogTempleFont {
        face_name: None,
        size: 9,
        char_set: FontCharSet::default(),
        italic: false,
        weight: 400,
    };
    my_dialog.menu = Some(NumberId(5));
    my_dialog.append_control(
        ButtonTemple::new(ButtonType::Normal, "&OK"),
        point2(100, 10),
        size2(50, 10),
        1,
    );
    my_dialog.append_control(
        ButtonTemple::new(ButtonType::Normal, "&Cancel"),
        point2(200, 10),
        size2(50, 10),
        2,
    );

    compile_all!(
        vstr,
        icon1,
        icon2,
        icon3,
        cursor1,
        menu1,
        st,
        my_dialog.pre_compile(NumberId(6))
    );
}
