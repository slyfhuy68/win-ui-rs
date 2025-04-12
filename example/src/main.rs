use capdows::win32::allmods::*;
use capdows::win32::control::Control;
use capdows_controls::button::*;
use capdows_controls::check_box::*;
use capdows_controls::edit::*;
use capdows_controls::group_box::*;
use capdows_controls::radio::*;
use capdows_controls::view::*;
use either::*;
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
    edit: Edit,
    finder: ImageTextView,
    text: ImageTextView,
}
struct Mycb {
    num: i8,
    controls: Option<MyControls>,
}
static mut ICON_STATE: bool = false;
fn get_state() -> bool {
    unsafe { ICON_STATE }
}
fn set_state(state: bool) {
    unsafe { ICON_STATE = state }
}
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
const EDIT_01: WindowID = 7u16;
const VIEW_01: WindowID = 8u16;
const VIEW_02: WindowID = 8u16;
impl MessageReceiver for Mycb {
    fn error_handler(&mut self, err: MessageReceiverError) -> MessageReceiverResult<isize> {
        println!("{:?}", err);
        Ok(err.code() as isize)
    }
    fn create(
        &mut self,
        _id: usize,
        window: &mut Window,
        _name: &str,
        _class: WindowClass,
        _file: ExecutableFile,
        _pos: Rectangle,
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
            Some(Rectangle::PointSize(Point(400, 0), Size(150, 50))),
            LINK_BUTTON_01,
            Default::default(),
            style.clone(),
            Default::default(),
            true,
            false,
        )
        .unwrap();
        link_button_1.set_note("114514abc中文").unwrap();
        let mut g_b = GroupBox::new(
            window,
            "分组框01",
            Some(Rectangle::PointSize(Point(575, 0), Size(300, 100))),
            GROUP_BOX_01,
            style.clone(),
            Default::default(),
            true,
            false,
        )
        .unwrap()
        .to_window();
        self.controls = Some(MyControls {
            button1: Button::new(
                window,
                "按钮01",
                Some(Rectangle::PointSize(Point(0, 0), Size(150, 50))),
                BUTTON_01,
                Default::default(),
                style.clone(),
                Default::default(),
                true,
                false,
            )
            .unwrap(),
            link_button_1,
            split_button: SplitButton::new(
                window,
                "分割按钮01",
                Some(Rectangle::PointSize(Point(200, 0), Size(150, 50))),
                SPLIT_BUTTON_01,
                Default::default(),
                style.clone(),
                Default::default(),
                true,
                false,
            )
            .unwrap(),
            a2: RadioButton::new(
                &mut g_b,
                "单选按钮a02",
                Some(Rectangle::PointSize(Point(150, 20), Size(100, 20))),
                RADIO_BUTTON_01_02,
                Default::default(),
                style.clone(),
                Default::default(),
                true,
                false,
            )
            .unwrap(),
            b1: RadioButton::new(
                &mut g_b,
                "单选按钮b01",
                Some(Rectangle::PointSize(Point(20, 70), Size(100, 20))),
                RADIO_BUTTON_02_01,
                Default::default(),
                style_group.clone(),
                Default::default(),
                true,
                false,
            )
            .unwrap(),
            b2: RadioButton::new(
                &mut g_b,
                "单选按钮b02",
                Some(Rectangle::PointSize(Point(150, 70), Size(100, 20))),
                RADIO_BUTTON_02_02,
                Default::default(),
                style.clone(),
                Default::default(),
                true,
                false,
            )
            .unwrap(),
            boxed1: CheckBox::new(
                window,
                "选择框01",
                Some(Rectangle::PointSize(Point(900, 0), Size(150, 50))),
                CHECK_BOX_01,
                Default::default(),
                style.clone(),
                Default::default(),
                true,
                false,
            )
            .unwrap(),
            boxed2: CheckBox::new(
                window,
                "选择框02",
                Some(Rectangle::PointSize(Point(900, 50), Size(150, 50))),
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
            )
            .unwrap(),
            a1: RadioButton::new(
                &mut g_b,
                "单选按钮a01",
                Some(Rectangle::PointSize(Point(20, 20), Size(100, 20))),
                RADIO_BUTTON_01_01,
                Default::default(),
                style_group.clone(),
                Default::default(),
                true,
                false,
            )
            .unwrap(),
            g_b: GroupBox::from_window(g_b).unwrap(),
            edit: Edit::new(
                window,
                "编辑框01",
                Some(Rectangle::PointSize(Point(15, 75), Size(130, 50))),
                EDIT_01,
                Default::default(),
                Default::default(),
                {
                    let mut ex_style: NormalWindowExStyles = Default::default();
                    ex_style.clint_edge = true;
                    ex_style
                },
                true,
                false,
            )
            .unwrap(),
            finder: ImageTextView::new(
                window,
                Some(Rectangle::PointSize(Point(200, 100), Size(130, 50))),
                VIEW_01,
                {
                    set_state(true);
                    ImageTextViewStyle::new_icon(
                        Icon::load_from_module(
                            ExecutableFile::from_current_file().unwrap(),
                            Right(3),
                            None,
                            true,
                        )
                        .unwrap(),
                    )
                },
                Default::default(),
                Default::default(),
                true,
                false,
            )
            .unwrap(),
            text: ImageTextView::new(
                window,
                Some(Rectangle::PointSize(Point(400, 100), Size(130, 50))),
                VIEW_02,
                ImageTextViewStyle::new_text("文字"),
                Default::default(),
                Default::default(),
                true,
                true,
            )
            .unwrap(),
        });
        println!("hello from example");
        Ok(true)
    }
    fn control_message(
        &mut self,
        _id: usize,
        _window: &mut Window,
        msg: &mut RawMessage,
        id: WindowID,
    ) -> MessageReceiverResult<isize> {
        let controls = &mut self.controls.as_mut().ok_or(NoProcessed)?;
        match id {
            VIEW_01 => {
                use ImageTextViewMsgType::*;
                let msg = msg.get_control_msg::<ImageTextView>()?;
                match msg.get_type() {
                    Clicked | DoubleClicked => {
                        println!("hi");
                        if get_state() {
                            controls
                                .finder
                                .change_content(ViewContent::Icon({
                                    set_state(false);
                                    Icon::load_from_module(
                                        ExecutableFile::from_current_file().unwrap(),
                                        Right(2),
                                        None,
                                        true,
                                    )
                                    .unwrap()
                                }))
                                .unwrap();
                        } else {
                            controls
                                .finder
                                .change_content(ViewContent::Icon({
                                    set_state(true);
                                    Icon::load_from_module(
                                        ExecutableFile::from_current_file().unwrap(),
                                        Right(3),
                                        None,
                                        true,
                                    )
                                    .unwrap()
                                }))
                                .unwrap();
                        }
                        Ok(0)
                    }
                    _ => Err(NoProcessed),
                }
            }
            BUTTON_01 => {
                use ButtonMsgType::*;
                let msg = msg.get_control_msg::<Button>()?;
                match msg.get_type() {
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
                let msg = msg.get_control_msg::<SplitButton>()?;
                match msg.get_type() {
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
                let msg = msg.get_control_msg::<LinkButton>()?;
                match msg.get_type() {
                    Clicked => {
                        println!("链接按钮1点了，文本：{}", controls.edit.get_text()?);
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
        Some(ClassBackgroundBrush::BtnFace),
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
