use capdows::Win32::allmods::*;
use capdows_controls::Button::*;
pub struct Mycb {}
use crate::WindowClassP::BrushC;
const Button1: WindowID = 1u16;
const Button2: WindowID = 2u16;
impl MessageReceiver for Mycb {
    fn create(
        &mut self,
        window: &mut Window,
        name: &str,
        class: WindowClass,
        file: ExecutableFile,
        pos: RectangleWH,
        itype: WindowType,
        //ex_data: usize,
    ) -> MessageReceiverResult<bool> {
        eprintln!("hello from create");
        Ok(true)
    }
    fn control_message(&mut self, window: &mut Window, msg: usize, id:WindowID) -> MessageReceiverResult<isize>{
        match id {
            Button1 => {
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
            Button2 => {
                use ButtonMsgType::*;
                let msg = get_contro_msg::<Button>(msg);
                if let Some(msg) = msg {
                    match msg.bm_type{
                        Clicked => {
                            println!("按钮2点了");
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
        None,
        Some(BrushC::BtnFace),
        0,
        0,
    )?;
    //println!("{}", class);
    let mut window =
        class.create_window("114", Default::default(),None, Box::new(Mycb {}))?;
    window.Fshow(1)?;
    println!("ok");
    let mut style = ChildWindowStyles::null();
    style.visble = true;
    style.tab_stop = true;
    Button::new(&mut window, "button好", Some(((0, 0), 150, 150)), Button1, Default::default(), style.clone(), Default::default(), true, false);
    Button::new(&mut window, "button好2", Some(((200, 0), 150, 150)), Button2, Default::default(), style, Default::default(), true, false);
    capdows::Win32::msg::msg_loop();
    Ok(())
}
