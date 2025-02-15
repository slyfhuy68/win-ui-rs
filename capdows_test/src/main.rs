use capdows::Win32::allmods::*;
use capdows_controls::button::*;
use capdows_controls::group_box::*;
pub struct Mycb {num:i8}
use crate::WindowClassP::BrushC;
const BUTTON_01: WindowID = 1u16;
const SPLIT_BUTTON_01: WindowID = 2u16;
const LINK_BUTTON_01: WindowID = 3u16;
impl MessageReceiver for Mycb {
    fn create(
        &mut self,
        _window: &mut Window,
        _name: &str,
        _class: WindowClass,
        _file: ExecutableFile,
        _pos: RectangleWH,
        _itype: WindowType,
        //ex_data: usize,
    ) -> MessageReceiverResult<bool> {
        eprintln!("hello from create");
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
                            println!("按钮1点了");
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
                            println!("分割按钮1边边点了！数字：{}按钮位置：{:?}", self.num, rect);
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
        NULLn,
        None,
        None,
        Some(Cursor::from_system(32512)?),
        Some(BrushC::BtnFace),
        0,
        0,
    )?;
    //println!("{}", class);
    let mut window =
        class.create_window("114", Default::default(),None, Box::new(Mycb {num:0}))?;
    window.Fshow(1)?;
    println!("ok");
    let mut style = ChildWindowStyles::null();
    style.visble = true;
    style.tab_stop = false;
    let _ = Button::new(&mut window, "按钮01", Some(((0, 0), 150, 50)), BUTTON_01, Default::default(), style.clone(), Default::default(), true, false);
    let _ = SplitButton::new(&mut window, "分割按钮01", Some(((200, 0), 150, 50)), SPLIT_BUTTON_01, Default::default(), style.clone(), Default::default(), true, false);
    let _ = LinkButton::new(&mut window, "链接按钮01", Some(((400, 0), 150, 50)), LINK_BUTTON_01, Default::default(), style.clone(), Default::default(), true, false);
    let _ = GroupBox::new(&mut window, "分组框01", Some(((600, 0), 150, 50)), 0u16, style.clone(), Default::default(), true, false);
    capdows::Win32::msg::msg_loop();
    Ok(())
}
