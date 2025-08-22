use capdows::prelude::*;
use capdows::ui::msg::NoProcessed;
use capdows_controls::prelude::*;
// use capdows_example::*; //从lib.rs导入
use euclid::rect;
use std::marker::PhantomData;
#[derive(Default, Debug)]
struct Mycb;
#[derive(Default, Debug)]
struct MyDlgcb;
const BUTTON_01: WindowID = 1u16;
const SPLIT_BUTTON_01: WindowID = 2u16;
const LINK_BUTTON_01: WindowID = 3u16;
const GROUP_BOX_01: WindowID = 4u16;
const CHECK_BOX_01: WindowID = 5u16;
const CHECK_BOX_02: WindowID = 6u16;
const EDIT_01: WindowID = 7u16;
const VIEW_01: WindowID = 8u16;
const COMBO_BOX_01: WindowID = 9u16;
// const FINDER_01: WindowID = 10u16;

const RADIO_BOX_01_01: WindowID = 501u16;
const RADIO_BOX_01_02: WindowID = 502u16;
const RADIO_BOX_02_01: WindowID = 503u16;
const RADIO_BOX_02_02: WindowID = 504u16;
//------------------------
const MENU_ITEM_1: MenuItemID = 1145u16;
impl MessageReceiver<DialogPorc> for MyDlgcb {}
impl DialogMessageReceiver for MyDlgcb {}
impl MessageReceiver for Mycb {
    fn menu_command(window: &mut Window, item: MenuCommandMsgItemPos) -> MessageReceiverResult<()> {
        if let MenuCommandMsgItemPos::CostomId(id) = item {
            if id == MENU_ITEM_1 + 4 {
                window
                    .with_menu(|menu| {
                        menu.clear().unwrap();
                        menu.insert_item(
                            None,
                            MenuItem::Normal(
                                MenuItemStyle::default(),
                                MenuItemShow::String(
                                    MenuCheckIcon::default(),
                                    "点击测试1".to_string(),
                                ),
                                Some(MENU_ITEM_1),
                            ),
                        )
                        .unwrap();
                    })
                    .unwrap();
                msg_box!(
                    &format!("菜单点击, 编号：{:?}", (id + 1 - MENU_ITEM_1)),
                    "提示"
                )
                .unwrap();
                msg_box!("重新开始", "提示").unwrap();
                window.redraw_menu_bar().unwrap();
                return Ok(());
            }
            window
                .with_menu(|menu| {
                    menu.set_item_state(MenuItemPos::CostomId(id), MenuItemDisabledState::Disabled)
                        .unwrap();
                    menu.insert_item(
                        None,
                        MenuItem::Normal(
                            MenuItemStyle::default(),
                            MenuItemShow::String(
                                MenuCheckIcon::default(),
                                "点击测试".to_string() + &((id + 2 - MENU_ITEM_1).to_string()),
                            ),
                            Some(id + 1),
                        ),
                    )
                    .unwrap();
                })
                .unwrap();
            msg_box!(
                &format!("菜单点击, 编号：{:?}", (id + 1 - MENU_ITEM_1)),
                "点击测试"
            )
            .unwrap();
            window.redraw_menu_bar().unwrap();
        };
        Ok(())
    }
    fn create(
        window: &mut Window,
        _name: &str,
        _class: &mut WindowClass,
        _file: &ExecutableFile,
        _pos: Rect,
        _itype: &WindowType<'_>,
        //ex_data: usize,
    ) -> MessageReceiverResult<bool> {
        const FONT: ControlFont = ControlFont::CaptionFont;

        Button::new_then(
            window,
            Some(rect(400, 0, 150, 50)),
            LINK_BUTTON_01,
            ButtonStyle::new(ButtonType::Link, "链接按钮01"),
            Some(FONT),
            |btn: &mut Button| {
                btn.set_note("114514abc中文").unwrap();
            },
        )
        .unwrap();

        Button::new(
            window,
            Some(rect(0, 0, 150, 50)),
            BUTTON_01,
            ButtonStyle::new(ButtonType::Normal, "按钮01"),
            Some(FONT),
        )
        .unwrap();

        Button::new(
            window,
            Some(rect(200, 0, 150, 50)),
            SPLIT_BUTTON_01,
            ButtonStyle::new(ButtonType::Split, "分割按钮01"),
            Some(FONT),
        )
        .unwrap();

        TextView::new(
            window,
            Some(rect(400, 100, 130, 50)),
            VIEW_01,
            TextViewStyle::new("文字"),
            Some(FONT),
        )
        .unwrap();

        GroupBox::new(
            window,
            Some(rect(575, 0, 300, 100)),
            GROUP_BOX_01,
            GroupBoxStyle::new("分组框01"),
            Some(FONT),
        )
        .unwrap();

        RadioBox::new(
            window,
            Some(rect(20 + 575, 20, 100, 20)),
            RADIO_BOX_01_01,
            RadioBoxStyle::new_text("单选按钮a01").group_leader(),
            Some(FONT),
        )
        .unwrap();

        RadioBox::new(
            window,
            Some(rect(150 + 575, 20, 100, 20)),
            RADIO_BOX_01_02,
            RadioBoxStyle::new_text("单选按钮a02"),
            Some(FONT),
        )
        .unwrap();

        RadioBox::new(
            window,
            Some(rect(20 + 575, 70, 100, 20)),
            RADIO_BOX_02_01,
            RadioBoxStyle::new_text("单选按钮b01").group_leader(),
            Some(FONT),
        )
        .unwrap();

        RadioBox::new(
            window,
            Some(rect(150 + 575, 70, 100, 20)),
            RADIO_BOX_02_02,
            RadioBoxStyle::new_text("单选按钮b02"),
            Some(FONT),
        )
        .unwrap();

        CheckBox::new(
            window,
            Some(rect(900, 0, 150, 50)),
            CHECK_BOX_01,
            CheckBoxStyle::new_text("选择框01"),
            Some(FONT),
        )
        .unwrap();

        CheckBox::new(
            window,
            Some(rect(900, 50, 150, 50)),
            CHECK_BOX_02,
            CheckBoxStyle::new_text("选择框02").three_state(),
            Some(FONT),
        )
        .unwrap();

        Edit::new(
            window,
            Some(rect(15, 75, 130, 50)),
            EDIT_01,
            EditStyle::new("编辑框01"),
            Some(FONT),
        )
        .unwrap();

        Edit::new(
            window,
            Some(rect(15, 75, 130, 50)),
            EDIT_01,
            EditStyle::new("编辑框01"),
            Some(FONT),
        )
        .unwrap();

        ComboBox::new(
            window,
            Some(rect(155, 85, 200, 100)),
            COMBO_BOX_01,
            ComboBoxStyle::new("组合框01"),
            Some(FONT),
        )
        .unwrap();

        println!("hello from example");
        Ok(true)
    }
    fn control_message(
        window: &mut Window,
        msg: &mut RawMessage,
        id: WindowID,
    ) -> MessageReceiverResult<isize> {
        // let controls = MY_CONTROLS.get().ok_or(NoProcessed)?;
        match id {
            BUTTON_01 => {
                use ButtonMsgType::*;
                let msg = msg.get_control_msg::<Button>().unwrap();
                let mut iter = [
                    RADIO_BOX_01_01,
                    RADIO_BOX_01_02,
                    RADIO_BOX_02_01,
                    RADIO_BOX_02_02,
                ]
                .into_iter()
                .map(|id| {
                    window
                        .with_child(id, |ctl| {
                            ctl.as_ctl::<RadioBox>().unwrap().is_checked().unwrap()
                        })
                        .unwrap()
                });
                match msg.get_type() {
                    Clicked => {
                        println!(
                            "按钮1点了a1:{} a2:{} b1:{} b2:{}",
                            iter.next().unwrap(),
                            iter.next().unwrap(),
                            iter.next().unwrap(),
                            iter.next().unwrap()
                        );
                        Ok(0)
                    }
                    _ => Err(NoProcessed),
                }
            }
            SPLIT_BUTTON_01 => {
                use ButtonMsgType::*;
                let msg = msg.get_control_msg::<Button>().unwrap();

                match msg.get_type() {
                    Clicked => {
                        let mut iter = [CHECK_BOX_01, CHECK_BOX_02].into_iter().map(|id| {
                            window
                                .with_child(id, |ctl| {
                                    ctl.as_ctl::<CheckBox>().unwrap().is_checked().unwrap()
                                })
                                .unwrap()
                        });
                        println!(
                            "分割按钮1点了box1:{} box2:{}",
                            iter.next().unwrap(),
                            iter.next().unwrap(),
                        );
                        Ok(0)
                    }
                    DropDown(rect) => {
                        Dialog::load(
                            ExecutableFile::from_current_file()?,
                            ResourceID::NumberId(6),
                            PhantomData::<MyDlgcb>,
                            None,
                        )
                        .unwrap();
                        println!("分割按钮1边点了！按钮位置：{rect:?}, 已创建对话框",);

                        Ok(0)
                    }
                    _ => Err(NoProcessed),
                }
            }
            LINK_BUTTON_01 => {
                use ButtonMsgType::*;
                let msg = msg.get_control_msg::<Button>().unwrap();

                match msg.get_type() {
                    Clicked => {
                        println!(
                            "链接按钮1点了，文本：{}",
                            window
                                .with_child(EDIT_01, |ctl| {
                                    ctl.as_ctl::<Edit>().unwrap().get_text().unwrap()
                                })
                                .unwrap()
                        );
                        Ok(0)
                    }
                    _ => Err(NoProcessed),
                }
            }
            _ => Err(NoProcessed),
        }
    }
}
fn main() -> Result<()> {
    let class = WindowClassBuilder::new(Lc!("LibraryTest 中文👅öé English"))
        .default_menu(NumberId(5))
        .icon(
            Icon::load_from_module(
                ExecutableFile::from_current_file().unwrap(),
                NumberId(1),
                None,
                false,
            )
            .unwrap(),
        )
        .small_icon(
            Icon::load_from_module(
                ExecutableFile::from_current_file().unwrap(),
                NumberId(1),
                None,
                false,
            )
            .unwrap(),
        )
        .default_cursor()
        .unwrap()
        .background_brush(Some(ClassBackgroundBrush::BtnFace))
        .build(PhantomData::<Mycb>)
        .unwrap();
    let mut menu_bar = MenuBar::new().unwrap();
    menu_bar
        .insert_item(
            None,
            MenuItem::Normal(
                MenuItemStyle::default(),
                MenuItemShow::String(MenuCheckIcon::default(), "点击测试1".to_string()),
                Some(MENU_ITEM_1),
            ),
        )
        .unwrap();
    class
        .create_window_then(
            "中文😝öé English",
            WindowType::Overlapped {
                style: Default::default(),
                menu: Some(menu_bar),
                owner: None,
                is_layered: false,
            },
            None,
            None,
            |window| {
                window.show(ShowWindowType::Normal);
                // window.redraw_menu_bar().unwrap();
            },
        )
        .unwrap();
    println!("ok");
    capdows::ui::msg::msg_loop()?;
    Ok(())
}
