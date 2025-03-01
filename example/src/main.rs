use capdows::win32::allmods::*;
use capdows::win32::control::Control;
use capdows_controls::button::*;
use capdows_controls::check_box::*;
use capdows_controls::group_box::*;
use capdows_controls::radio::*;
use capdows_controls::text_view::*;
struct MyControls {
    a1: RadioButton,
    a2: RadioButton,
    b1: RadioButton,
    b2: RadioButton,
    boxed1: CheckBox,
    boxed2: CheckBox,
    button1: Button,
    link_button_1: LinkButton,
    split_button: SplitButton,
    g_b: GroupBox,
    text_view: TextView,
}
struct Mycb {
    num: i8,
    controls: Option<MyControls>,
}
use crate::BrushC;
const BUTTON_01: WindowID = 1u16;
const SPLIT_BUTTON_01: WindowID = 2u16;
const LINK_BUTTON_01: WindowID = 3u16;
const GROUP_BOX_01: WindowID = 4u16;
const RADIO_BUTTON_01_01: WindowID = 1u16;
const RADIO_BUTTON_01_02: WindowID = 2u16;
const RADIO_BUTTON_02_01: WindowID = 3u16;
const RADIO_BUTTON_02_02: WindowID = 4u16;
const CHECK_BOX_01: WindowID = 5u16;
const CHECK_BOX_02: WindowID = 6u16;
const TEXT_VIEW_01: WindowID = 6u16;
impl MessageReceiver for Mycb {
    fn create(
        &mut self,
        window: &mut Window,
        _name: &str,
        _class: WindowClass,
        _file: ExecutableFile,
        _pos: RectangleWH,
        _itype: WindowType,
        //ex_data: usize,
    ) -> MessageReceiverResult<bool> {
        let mut style = ChildWindowStyles::null();
        style.visble = true;
        style.tab_stop = false;
        let mut style_group = style.clone();
        style_group.group = true;
        let mut link_button_1 = LinkButton::new(
            window,
            "链接按钮01",
            Some(((400, 0), 150, 50)),
            LINK_BUTTON_01,
            Default::default(),
            style.clone(),
            Default::default(),
            true,
            false,
        )?;
        link_button_1.set_note("114514abc中文")?;
        let mut g_b = GroupBox::new(
            window,
            "分组框01",
            Some(((575, 0), 300, 100)),
            GROUP_BOX_01,
            style.clone(),
            Default::default(),
            true,
            false,
        )?
        .to_window();
        self.controls = Some(MyControls {
            button1: Button::new(
                window,
                "按钮01",
                Some(((0, 0), 150, 50)),
                BUTTON_01,
                Default::default(),
                style.clone(),
                Default::default(),
                true,
                false,
            )?,
            link_button_1,
            split_button: SplitButton::new(
                window,
                "分割按钮01",
                Some(((200, 0), 150, 50)),
                SPLIT_BUTTON_01,
                Default::default(),
                style.clone(),
                Default::default(),
                true,
                false,
            )?,
            a2: RadioButton::new(
                &mut g_b,
                "单选按钮a02",
                Some(((150, 20), 100, 20)),
                RADIO_BUTTON_01_02,
                Default::default(),
                style.clone(),
                Default::default(),
                true,
                false,
            )?,
            b1: RadioButton::new(
                &mut g_b,
                "单选按钮b01",
                Some(((20, 70), 100, 20)),
                RADIO_BUTTON_02_01,
                Default::default(),
                style_group.clone(),
                Default::default(),
                true,
                false,
            )?,
            b2: RadioButton::new(
                &mut g_b,
                "单选按钮b02",
                Some(((150, 70), 100, 20)),
                RADIO_BUTTON_02_02,
                Default::default(),
                style.clone(),
                Default::default(),
                true,
                false,
            )?,
            boxed1: CheckBox::new(
                window,
                "选择框01",
                Some(((900, 0), 150, 50)),
                CHECK_BOX_01,
                Default::default(),
                style.clone(),
                Default::default(),
                true,
                false,
            )?,
            boxed2: CheckBox::new(
                window,
                "选择框02",
                Some(((900, 50), 150, 50)),
                CHECK_BOX_02,
                CheckBoxDrawType(ButtonAutoDrawType::TextOnly(false), {
                    let mut state: CheckBoxStyle = Default::default();
                    state.three_state = true;
                    state
                }),
                style.clone(),
                Default::default(),
                true,
                false,
            )?,
            a1: RadioButton::new(
                &mut g_b,
                "单选按钮a01",
                Some(((20, 20), 100, 20)),
                RADIO_BUTTON_01_01,
                Default::default(),
                style_group.clone(),
                Default::default(),
                true,
                false,
            )?,
            g_b: *GroupBox::from_window(g_b)?,
            text_view: TextView::new(
                window,
                "文本11111111112",
                Some(((15, 75), 130, 50)),
                TEXT_VIEW_01,
                Default::default(),
                Default::default(),
                Default::default(),
                true,
                true,
            )?,
        });
        println!("hello from create");
        Ok(true)
    }
    fn control_message(
        &mut self,
        _window: &mut Window,
        msg: usize,
        id: WindowID,
    ) -> MessageReceiverResult<isize> {
        let controls = &mut self.controls.as_mut().unwrap();
        match id {
            BUTTON_01 => {
                use ButtonMsgType::*;
                let msg = get_control_msg::<Button>(msg)?;
                match msg.bm_type {
                    Clicked => {
                        println!(
                            "按钮1点了a1:{} a2:{} b1:{} b2:{}",
                            controls.a1.is_checked()?,
                            controls.a2.is_checked()?,
                            controls.b1.is_checked()?,
                            controls.b2.is_checked()?
                        );
                        Ok(0)
                    }
                    _ => Err(NoProcessed),
                }
            }
            SPLIT_BUTTON_01 => {
                use SplitButtonMsgType::*;
                let msg = get_control_msg::<SplitButton>(msg)?;
                match msg.bm_type {
                    Clicked => {
                        println!(
                            "分割按钮1点了box1:{} box2:{}",
                            controls.boxed1.is_checked()?,
                            controls.boxed2.is_checked()?,
                        );
                        Ok(0)
                    }
                    DropDown(rect) => {
                        if self.num == 127 {
                            self.num = -128
                        } else {
                            self.num += 1;
                        }
                        println!("分割按钮1边点了！数字：{}按钮位置：{:?}", self.num, rect);
                        Ok(0)
                    }
                    _ => Err(NoProcessed),
                }
            }
            LINK_BUTTON_01 => {
                use ButtonMsgType::*;
                let msg = get_control_msg::<LinkButton>(msg)?;
                match msg.bm_type {
                    Clicked => {
                        println!("链接按钮1点了");
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
    let class = WindowClass::register(
        "LibraryTest 中文👅öé English", //日常使用时不建议使用非ANSI字符
        Default::default(),
        None,
        None,
        None,
        Some(Cursor::from_system(32512)?),
        Some(BrushC::BtnFace),
        0,
        0,
    )?;
    //println!("{}", class);
    let mut window = class.create_window(
        "中文😝öé English",
        Default::default(),
        None,
        Box::new(Mycb {
            num: 0,
            controls: None,
        }),
    )?;
    window.show(ShowWindowType::Normal)?;
    println!("ok");
    capdows::win32::msg::msg_loop();
    Ok(())
}
