use capdows::Lc;
use capdows::msg_box;
use capdows::prelude::*;
use capdows::ui::msg::NoProcessed;
use capdows_controls::prelude::*;
use euclid::rect;
use std::marker::PhantomData;
// use capdows_example::*;
#[derive(Debug)]
struct MyControls {
    a1: RadioBox,
    a2: RadioBox,
    b1: RadioBox,
    b2: RadioBox,
    boxed1: CheckBox,
    boxed2: CheckBox,
    edit: Edit,
}
static MY_CONTROLS: OnceLock<MyControls> = OnceLock::new();
use std::sync::OnceLock;
#[derive(Default, Debug)]
struct Mycb;
const BUTTON_01: WindowID = 1u16;
const SPLIT_BUTTON_01: WindowID = 2u16;
const LINK_BUTTON_01: WindowID = 3u16;
const GROUP_BOX_01: WindowID = 4u16;
const RADIO_BOX_01_01: WindowID = 1u16;
const RADIO_BOX_01_02: WindowID = 2u16;
const RADIO_BOX_02_01: WindowID = 3u16;
const RADIO_BOX_02_02: WindowID = 4u16;
const CHECK_BOX_01: WindowID = 5u16;
const CHECK_BOX_02: WindowID = 6u16;
const EDIT_01: WindowID = 7u16;
const VIEW_01: WindowID = 8u16;
// const VIEW_02: WindowID = 9u16;
//------------------------
const MENU_ITEM_1: MenuItemID = 1145u16;
impl MessageReceiver for Mycb {
    fn menu_command(
        _id: usize,
        window: &mut Window,
        item: MenuCommandMsgItemPos,
    ) -> MessageReceiverResult<()> {
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
    // fn error_handler(err: MessageReceiverError) -> MessageReceiverResult<isize> {
    //     println!("发生错误: {:?}", err);
    //     Ok(err.code() as isize)
    // }
    fn create(
        _id: usize,
        window: &mut Window,
        _name: &str,
        _class: &mut WindowClass,
        _file: &ExecutableFile,
        _pos: Rect,
        _itype: &mut WindowType,
        //ex_data: usize,
    ) -> MessageReceiverResult<bool> {
        const FONT: ControlFont = ControlFont::CaptionFont;
        let mut link_button_1 = Button::new(
            window,
            Some(rect(400, 0, 150, 50)),
            LINK_BUTTON_01,
            ButtonStyle::new(ButtonType::Link, "链接按钮01"),
            Some(FONT),
        )
        .unwrap();
        link_button_1.set_note("114514abc中文").unwrap();
        link_button_1.neednot();
        Button::new(
            window,
            Some(rect(0, 0, 150, 50)),
            BUTTON_01,
            ButtonStyle::new(ButtonType::Normal, "按钮01"),
            Some(FONT),
        )
        .unwrap()
        .neednot();
        Button::new(
            window,
            Some(rect(200, 0, 150, 50)),
            SPLIT_BUTTON_01,
            ButtonStyle::new(ButtonType::Split, "分割按钮01"),
            Some(FONT),
        )
        .unwrap()
        .neednot();
        TextView::new(
            window,
            Some(rect(400, 100, 130, 50)),
            VIEW_01,
            TextViewStyle::new("文字"),
            Some(FONT),
        )
        .unwrap()
        .neednot();
        let mut g_b = GroupBox::new(
            window,
            Some(rect(575, 0, 300, 100)),
            GROUP_BOX_01,
            GroupBoxStyle::new("分组框01"),
            Some(FONT),
        )
        .unwrap();
        let useful_controls = MyControls {
            a1: RadioBox::new(
                g_b.get_window_mut(),
                Some(rect(20, 20, 100, 20)),
                RADIO_BOX_01_01,
                RadioBoxStyle::new_text("单选按钮a01").group_leader(),
                Some(FONT),
            )
            .unwrap(),
            a2: RadioBox::new(
                g_b.get_window_mut(),
                Some(rect(150, 20, 100, 20)),
                RADIO_BOX_01_02,
                RadioBoxStyle::new_text("单选按钮a02"),
                Some(FONT),
            )
            .unwrap(),
            b1: RadioBox::new(
                g_b.get_window_mut(),
                Some(rect(20, 70, 100, 20)),
                RADIO_BOX_02_01,
                RadioBoxStyle::new_text("单选按钮b01").group_leader(),
                Some(FONT),
            )
            .unwrap(),
            b2: RadioBox::new(
                g_b.get_window_mut(),
                Some(rect(150, 70, 100, 20)),
                RADIO_BOX_02_02,
                RadioBoxStyle::new_text("单选按钮b02"),
                Some(FONT),
            )
            .unwrap(),
            boxed1: CheckBox::new(
                window,
                Some(rect(900, 0, 150, 50)),
                CHECK_BOX_01,
                CheckBoxStyle::new_text("选择框01"),
                Some(FONT),
            )
            .unwrap(),
            boxed2: CheckBox::new(
                window,
                Some(rect(900, 50, 150, 50)),
                CHECK_BOX_02,
                CheckBoxStyle::new_text("选择框02").three_state(),
                Some(FONT),
            )
            .unwrap(),
            edit: Edit::new(
                window,
                Some(rect(15, 75, 130, 50)),
                EDIT_01,
                EditStyle::new("编辑框01"),
                Some(FONT),
            )
            .unwrap(),
        };
        g_b.neednot();
        MY_CONTROLS.set(useful_controls).unwrap();
        println!("hello from example");
        Ok(true)
    }
    fn control_message(
        _id: usize,
        _window: &mut Window,
        msg: &mut RawMessage,
        id: WindowID,
    ) -> MessageReceiverResult<isize> {
        let controls = MY_CONTROLS.get().ok_or(NoProcessed)?;
        match id {
            BUTTON_01 => {
                use ButtonMsgType::*;
                let msg = msg.get_control_msg::<Button>().unwrap();
                match msg.get_type() {
                    Clicked => {
                        println!(
                            "按钮1点了a1:{} a2:{} b1:{} b2:{}",
                            controls.a1.is_checked().unwrap(),
                            controls.a2.is_checked().unwrap(),
                            controls.b1.is_checked().unwrap(),
                            controls.b2.is_checked().unwrap()
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
                        println!(
                            "分割按钮1点了box1:{} box2:{}",
                            controls.boxed1.is_checked().unwrap(),
                            controls.boxed2.is_checked().unwrap(),
                        );
                        Ok(0)
                    }
                    DropDown(rect) => {
                        println!("分割按钮1边点了！按钮位置：{:?}", rect);
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
                        println!("链接按钮1点了，文本：{}", controls.edit.get_text().unwrap());
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
    let mut window = class
        .create_window(
            "中文😝öé English",
            WindowType::Overlapped {
                style: Default::default(),
                menu: Some(menu_bar),
                owner: None,
                is_layered: false,
            },
            None,
            None,
        )
        .unwrap();
    let _ = window.show(ShowWindowType::Normal);
    // window.redraw_menu_bar().unwrap();
    println!("ok");
    capdows::ui::msg::msg_loop();
    Ok(())
}
