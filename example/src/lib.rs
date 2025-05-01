use capdows::win32::allmods::*;
use capdows::win32::control::Control;
use capdows::win32::control::ControlMsgType;
use capdows::win32::mouse::release_mouse;
use capdows_controls::view::*;
use std::sync::LazyLock;
pub struct WindowFinder(ImageTextView);
impl Control for WindowFinder {
    type MsgType = WindowFinderMsg;
    unsafe fn force_from_window(wnd: Window) -> Self {
        unsafe { WindowFinder(ImageTextView::force_from_window(wnd)) }
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
    fn get_class() -> WindowClass {
        ImageTextView::get_class()
    }
}
pub struct WindowsFinderMessageReceiver {
    currect_wnd: Option<Window>,
}
static icon_full: LazyLock<Icon> = LazyLock::new(|| {
    Icon::load_from_module(
        ExecutableFile::from_current_file().unwrap(),
        NumberId(3),
        None,
        true,
    )
    .unwrap()
});
static icon_empty: LazyLock<Icon> = LazyLock::new(|| {
    Icon::load_from_module(
        ExecutableFile::from_current_file().unwrap(),
        NumberId(2),
        None,
        true,
    )
    .unwrap()
});
static cursor: LazyLock<Cursor> = LazyLock::new(|| {
    Cursor::load_from_module(
        ExecutableFile::from_current_file().unwrap(),
        NumberId(4),
        None,
        true,
    )
    .unwrap()
});
impl WindowFinder {
    pub fn new(window: &mut Window, pos: Option<Rectangle>, id: WindowID) -> Result<WindowFinder> {
        let mut view = ImageTextView::new(
            window,
            pos,
            id,
            ImageTextViewStyle::new_icon(icon_full.clone()),
            Default::default(),
            Default::default(),
            true,
            false,
        )?;
        view.get_window_mut().add_msg_receiver(
            10,
            Box::new(WindowsFinderMessageReceiver { currect_wnd: None }),
        );
        Ok(Self(view))
    }
}
pub enum WindowFinderMsgType {
    BeginFind,
    SelChanged(Option<Window>),
    EndFind,
}
use ButtonState::*;
pub use WindowFinderMsgType::*;
pub struct WindowFinderMsg(Window, WindowFinderMsgType);
use capdows::win32::control::ControlMsg;
impl ControlMsg for WindowFinderMsg {
    type ControlMsgDataType = Window;
    fn into_raw_control_msg(self) -> Result<(u32, Option<Self::ControlMsgDataType>)> {
        Ok((
            match self.1 {
                BeginFind => 114,
                EndFind => 514,
                SelChanged(x) => return Ok((114514, x)),
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
            wnd,
            match code {
                114 => BeginFind,
                514 => EndFind,
                114514 => SelChanged(data.map(|a| a.copy_handle())),
                _ => return Err(ERROR_MSG_CODE_NOT_SUPPORT),
            },
        ))
    }
}
impl ControlMsgType for WindowFinderMsg {
    type ControlType = WindowFinder;
    fn get_control(&self) -> &<Self as ControlMsgType>::ControlType {
        todo!()
    }
    fn get_control_mut(&mut self) -> &mut <Self as ControlMsgType>::ControlType {
        todo!()
    }
}
impl MessageReceiver for WindowsFinderMessageReceiver {
    fn mouse_msg(
        &mut self,
        _id: usize,
        window: &mut Window,
        msg: MouseMsg,
    ) -> MessageReceiverResult<()> {
        match msg {
            MouseMsg::Move { mtype, is_nc } => {
                if !is_nc {
                    if let Some(_) = self.currect_wnd {
                        match mtype {
                            MouseMsgMoveType::Move(point) => {
                                let wnd_point =
                                    Window::from_screen_point(point.window_to_screen(window)?);
                                if wnd_point != self.currect_wnd {
                                    erase_window_border(self.currect_wnd);
                                    self.currect_wnd = wnd_point;
                                    window.send_control_nofiy(WindowFinderMsg(
                                        window.copy_handle(),
                                        SelChanged(self.currect_wnd.map(|a| a.copy_handle())),
                                    ))?;
                                    draw_window_border(self.currect_wnd);
                                }
                                Ok(())
                            }
                            _ => Err(NoProcessed),
                        }
                    } else {
                        Err(NoProcessed)
                    }
                } else {
                    Err(NoProcessed)
                }
            }
            MouseMsg::Button {
                button_type,
                state,
                pos,
                is_nc,
            } => {
                if !is_nc {
                    match button_type {
                        Left => match state {
                            Down | DoubleClick => {
                                if window.send_control_msg(WindowFinderMsg(
                                    window.copy_handle(),
                                    BeginFind,
                                ))? < 0
                                {
                                    //返回大于或等于零表示允许继续查找
                                    return Err(NoProcessed);
                                };
                                ImageTextView::from_window(&window.copy_handle())?
                                    .change_content(ViewContent::Icon(*icon_empty));
                                self.currect_wnd = Some(window.copy_handle());
                                draw_window_border(self.currect_wnd);
                                window.capture_mouse();
                                window.send_control_nofiy(WindowFinderMsg(
                                    window.copy_handle(),
                                    SelChanged(Some(window.copy_handle())),
                                ));
                                Ok(())
                            }
                            Up => {
                                if let Some(_) = self.currect_wnd {
                                    {
                                        erase_window_border(self.currect_wnd);
                                        release_mouse();
                                        Cursor::from_system(SystemCursor::NormalSelection)?.apply();
                                        ImageTextView::from_window(window)?
                                            .change_content(ViewContent::Icon(*icon_full));
                                    };
                                    window.send_control_nofiy(WindowFinderMsg(
                                        window.copy_handle(),
                                        EndFind,
                                    ));
                                    self.currect_wnd = None;
                                    Ok(())
                                } else {
                                    Err(NoProcessed)
                                }
                            }
                        },
                        _ => Err(NoProcessed),
                    }
                } else {
                    Err(NoProcessed)
                }
            }
            _ => Err(NoProcessed),
        }
    }
}
