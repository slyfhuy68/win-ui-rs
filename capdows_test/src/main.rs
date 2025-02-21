use capdows::Win32::allmods::*;
use capdows_controls::button::*;
use capdows_controls::radio::*;
use capdows_controls::check_box::*;
use capdows::Win32::control::Control;
use capdows_controls::group_box::*;
pub struct Mycb {num:i8, a1:Option<RadioButton>, a2:Option<RadioButton>,b1:Option<RadioButton>,b2:Option<RadioButton>,}
use crate::WindowClassP::BrushC;
const BUTTON_01: WindowID = 1u16;
const SPLIT_BUTTON_01: WindowID = 2u16;
const LINK_BUTTON_01: WindowID = 3u16;
const GROUP_BOX_01: WindowID = 4u16;
    const RADIO_BUTTON_01_01: WindowID = 1u16;
    const RADIO_BUTTON_01_02: WindowID = 2u16;
    const RADIO_BUTTON_02_01: WindowID = 3u16;
    const RADIO_BUTTON_02_02: WindowID = 4u16;
const CHECK_BOX_01: WindowID = 5u16;
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
        eprintln!("hello from create");
            let mut style = ChildWindowStyles::null();
            style.visble = true;
            style.tab_stop = false;
            let mut style_group = style.clone();
            style_group.group = true;
            let _ = Button::new(window, "按钮01", Some(((0, 0), 150, 50)), BUTTON_01, Default::default(), style.clone(), Default::default(), true, false)?;
            let _ = SplitButton::new(window, "分割按钮01", Some(((200, 0), 150, 50)), SPLIT_BUTTON_01, Default::default(), style.clone(), Default::default(), true, false)?;
            let _ = LinkButton::new(window, "链接按钮01", Some(((400, 0), 150, 50)), LINK_BUTTON_01, Default::default(), style.clone(), Default::default(), true, false)?.set_note("114514abc中文")?;
            let mut g_b = GroupBox::new(window, "分组框01", Some(((575, 0), 300, 100)), GROUP_BOX_01, style.clone(), Default::default(), true, false)?.to_window();
            self.a1 = Some(RadioButton::new(&mut g_b, "单选按钮a01", Some(((20, 20), 100, 20)), RADIO_BUTTON_01_01, Default::default(), style_group.clone(), Default::default(), true, false)?);
            self.a2 = Some(RadioButton::new(&mut g_b, "单选按钮a02", Some(((150, 20), 100, 20)), RADIO_BUTTON_01_02, Default::default(), style.clone(), Default::default(), true, false)?);
            self.b1 = Some(RadioButton::new(&mut g_b, "单选按钮b01", Some(((20, 70), 100, 20)), RADIO_BUTTON_02_01, Default::default(), style_group.clone(), Default::default(), true, false)?);
            self.b2 = Some(RadioButton::new(&mut g_b, "单选按钮b02", Some(((150, 70), 100, 20)), RADIO_BUTTON_02_02, Default::default(), style.clone(), Default::default(), true, false)?);
            let _ = CheckBox::new(window, "选择框01", Some(((900, 0), 150, 50)), CHECK_BOX_01, Default::default(), style.clone(), Default::default(), true, false)?;
        Ok(true)
    }
    fn control_message(&mut self, _window: &mut Window, msg: usize, id:WindowID) -> MessageReceiverResult<isize>{
        match id {
            BUTTON_01 => {
                use ButtonMsgType::*;
                let msg = get_contro_msg::<Button>(msg);
                if let Some(msg) = msg {
                    match msg.bm_type{
                        Clicked => {
                            println!("按钮1点了a1:{} a2:{} b1:{} b2:{}", 
                                &self.a1.clone().expect("REASON").is_checked()?, 
                                &self.a2.clone().expect("REASON").is_checked()?, 
                                &self.b1.clone().expect("REASON").is_checked()?, 
                                &self.b2.clone().expect("REASON").is_checked()?
                                );
                            Ok(0)
                        }
                        _ => Err(NoProcessed)
                    }
                } else {
                    Err(NoProcessed)
                }
            }, 
            SPLIT_BUTTON_01 => {
                use SplitButtonMsgType::*;
                let msg = get_contro_msg::<SplitButton>(msg);
                if let Some(msg) = msg {
                    match msg.bm_type{
                        Clicked => {
                            println!("分割按钮1点了");
                            Ok(0)
                        }, 
                        DropDown(rect) => {
                            if self.num == 127 {
                                self.num = -128
                            } else {
                                self.num += 1;
                            }
                            println!("分割按钮1边点了！数字：{}按钮位置：{:?}", self.num, rect);
                            Ok(0)
                        }
                        _ => Err(NoProcessed)
                    }
                } else {
                    Err(NoProcessed)
                }
            }, 
            LINK_BUTTON_01 => {
                use ButtonMsgType::*;
                let msg = get_contro_msg::<LinkButton>(msg);
                if let Some(msg) = msg {
                    match msg.bm_type{
                        Clicked => {
                            println!("链接按钮1点了");
                            Ok(0)
                        }
                        _ => Err(NoProcessed)
                    }
                } else {
                    Err(NoProcessed)
                }
            }, 
            _ => Err(NoProcessed)
        }
    } 
}
fn main() -> Result<()> {
    let class = WindowClass::register(
        "LibraryTest",
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
    let mut window =
        class.create_window("114", Default::default(),None, Box::new(Mycb {num:0, a1:None, a2:None, b1:None, b2:None}))?;
    window.Fshow(1)?;
    println!("ok");
    capdows::Win32::msg::msg_loop();
    Ok(())
}
