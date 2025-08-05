#![allow(static_mut_refs)]
use MessageReceiverError::NoProcessed;
use capdows::positioning::Win32Point;
use capdows::prelude::*;
use capdows::ui::image::SystemCursor;
use capdows::ui::mouse::release_mouse;
use capdows::ui::msg::ButtonState::*;
use capdows::ui::msg::MouseButton;
use capdows::ui::msg::MouseMsg;
use capdows::ui::msg::MouseMsgMoveType;
use capdows::ui::option_copy_handle;
use capdows_controls::prelude::*;
use std::sync::LazyLock;
pub struct WindowFinder(pub ImageView);
impl Control for WindowFinder {
    const CLASS_NAME: &'static str = "Static";
    const CLASS_NAME_WIDE: &'static widestr = L!("Static");
    type MsgType = WindowFinderMsg;
    unsafe fn force_from_window(wnd: Window) -> Self {
        unsafe { WindowFinder(ImageView::force_from_window(wnd)) }
    }
    fn to_window(self) -> Window {
        self.0.to_window()
    }
    fn get_window(&self) -> &Window {
        self.0.get_window()
    }
    fn get_window_mut(&mut self) -> &mut Window {
        self.0.get_window_mut()
    }
    fn is_self(_wnd: &Window) -> Result<bool> {
        Ok(true)
    }
    // fn get_class() -> WindowClass {
    //     ImageView::get_class()
    // }
}
static ICON_FULL: LazyLock<Icon> = LazyLock::new(|| {
    Icon::load_from_module(
        ExecutableFile::from_current_file().unwrap(),
        NumberId(3),
        None,
        true,
    )
    .unwrap()
});
static ICON_EMPTY: LazyLock<Icon> = LazyLock::new(|| {
    Icon::load_from_module(
        ExecutableFile::from_current_file().unwrap(),
        NumberId(2),
        None,
        true,
    )
    .unwrap()
});
static FIND_CURSOR: LazyLock<Cursor> = LazyLock::new(|| {
    Cursor::load_from_module(
        ExecutableFile::from_current_file().unwrap(),
        NumberId(4),
        None,
        true,
    )
    .unwrap()
});
impl WindowFinder {
    pub fn new(window: &mut Window, pos: Option<Rect>, id: WindowID) -> Result<WindowFinder> {
        let mut view = ImageView::new(
            window,
            pos,
            id,
            ImageViewStyle::new_icon("WndFiner", ICON_FULL.clone()).enable_notify(),
            Some(ControlFont::CaptionFont),
        )?;
        view.get_window_mut()
            .add_msg_receiver(10, PhantomData::<WindowsFinderMessageReceiver>)?;
        Ok(Self(view))
    }
}
#[derive(Debug)]
pub enum WindowFinderMsgType {
    BeginFind,
    SelChanged(Option<Window>),
    EndFind,
}
pub use WindowFinderMsgType::*;
pub struct WindowFinderMsg(WindowFinder, WindowFinderMsgType);
impl Drop for WindowFinderMsg {
    fn drop(&mut self) {
        self.0.get_window_mut().nullify()
    }
}
impl WindowFinderMsg {
    pub fn get_type(&self) -> &WindowFinderMsgType {
        &self.1
    }
}
impl ControlMsg for WindowFinderMsg {
    type ControlMsgDataType = Window;
    fn into_raw_control_msg(mut self) -> Result<(u32, Option<Self::ControlMsgDataType>)> {
        Ok((
            match std::mem::replace(&mut self.1, WindowFinderMsgType::EndFind) {
                BeginFind => 114,
                EndFind => 514,
                SelChanged(x) => {
                    return Ok((1145, x));
                }
            },
            None,
        ))
    }
    fn from_raw_control_msg(
        code: u32,
        data: Option<&mut Self::ControlMsgDataType>,
        wnd: Window,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self(
            unsafe { WindowFinder::force_from_window(wnd) },
            match code {
                114 => BeginFind,
                514 => EndFind,
                312 => SelChanged(data.map(|a| a.copy_handle())),
                _ => return Err(ERROR_MSG_CODE_NOT_SUPPORT),
            },
        ))
    }
}
impl ControlMsgType for WindowFinderMsg {
    type ControlType = WindowFinder;
    fn get_control(&self) -> &<Self as ControlMsgType>::ControlType {
        &self.0
    }
    fn get_control_mut(&mut self) -> &mut <Self as ControlMsgType>::ControlType {
        &mut self.0
    }
}
#[derive(Default, Debug)]
struct WindowsFinderMessageReceiver;
impl MessageReceiver for WindowsFinderMessageReceiver {
    fn mouse_msg(_id: usize, window: &mut Window, msg: MouseMsg) -> MessageReceiverResult<()> {
        static mut CURRECT_WND: Option<Window> = None;
        match msg {
            MouseMsg::Move { mtype, is_nc } => {
                if is_nc {
                    return Err(NoProcessed);
                };
                if unsafe { CURRECT_WND.is_none() } {
                    return Err(NoProcessed);
                }
                let mut point = if let MouseMsgMoveType::Move(point) = mtype {
                    point
                } else {
                    return Err(NoProcessed);
                };
                point.window_to_screen(window)?;
                let wndpoint = Window::from_screen_point(point);
                let wnd_point = if let Some(wnd_point) = wndpoint {
                    wnd_point
                } else {
                    unsafe {
                        CURRECT_WND = None;
                        draw_window_border(&mut CURRECT_WND)?;
                    }
                    return Ok(());
                };
                if unsafe { wnd_point.handle_eq(CURRECT_WND.as_ref().unwrap()) } {
                    return Ok(());
                }
                unsafe {
                    erase_window_border(&mut CURRECT_WND)?;
                    CURRECT_WND = Some(wnd_point);
                }
                window.send_control_nofiy(WindowFinderMsg(
                    unsafe { WindowFinder::force_from_window(window.copy_handle()) },
                    SelChanged(unsafe { option_copy_handle(&CURRECT_WND) }),
                ))?;
                unsafe {
                    draw_window_border(&mut CURRECT_WND)?;
                }
                Ok(())
            }
            MouseMsg::Button {
                button_type,
                state,
                is_nc,
                ..
            } => {
                if !is_nc {
                    return Err(NoProcessed);
                }
                if MouseButton::Left != button_type {
                    return Err(NoProcessed);
                }
                match state {
                    Down | DoubleClick => {
                        if window.send_control_msg(WindowFinderMsg(
                            unsafe { WindowFinder::force_from_window(window.copy_handle()) },
                            BeginFind,
                        ))? < 0
                        {
                            //返回大于或等于零表示允许继续查找
                            return Err(NoProcessed);
                        };
                        // ImageView::from_window(window.copy_handle())?
                        //     .change_content(ViewContent::Icon((*ICON_EMPTY).copy_handle()))?;
                        unsafe {
                            CURRECT_WND = Some(window.copy_handle());
                            draw_window_border(&mut CURRECT_WND)?;
                        }
                        window.capture_mouse();
                        window.send_control_nofiy(WindowFinderMsg(
                            unsafe { WindowFinder::force_from_window(window.copy_handle()) },
                            SelChanged(Some(window.copy_handle())),
                        ))?;
                        Ok(())
                    }
                    Up => {
                        if unsafe { CURRECT_WND.is_none() } {
                            return Err(NoProcessed);
                        }
                        unsafe {
                            erase_window_border(&mut CURRECT_WND)?;
                        }
                        release_mouse()?;
                        Cursor::from_system(SystemCursor::NormalSelection)?.apply();
                        // ImageView::from_window(window.copy_handle())?
                        //     .change_content(ViewContent::Icon((*ICON_FULL).copy_handle()))?;
                        window.send_control_nofiy(WindowFinderMsg(
                            unsafe { WindowFinder::force_from_window(window.copy_handle()) },
                            EndFind,
                        ))?;
                        unsafe {
                            CURRECT_WND = None;
                        }
                        Ok(())
                    }
                }
            }
            _ => Err(NoProcessed),
        }
    }
}
fn draw_window_border(_wnd: &mut Option<Window>) -> Result<()> {
    Ok(())
}
fn erase_window_border(_wnd: &mut Option<Window>) -> Result<()> {
    Ok(())
}
