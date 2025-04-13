use capdows::win32::allmods::*;
use capdows_controls::view::*;
use std::sync::LazyLock;
pub struct WindowFinder(ImageTextView);
pub struct WindowsFinderMessageReceiver {
    state: bool,
    currect_wnd: Option<Window>,
}
static bitmap1: LazyLock<Icon> = LazyLock::new(|| {
    Icon::load_from_module(
        ExecutableFile::from_current_file().unwrap(),
        NumberId(3),
        None,
        true,
    )
    .unwrap()
});
static bitmap2: LazyLock<Icon> = LazyLock::new(|| {
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
        let view = ImageTextView::new(
            window,
            pos,
            id,
            {
                set_state(true);
                ImageTextViewStyle::new_icon(*bitmap1)
            },
            Default::default(),
            Default::default(),
            true,
            false,
        )?;
        view.get_window_mut().add_msg_receiver(
            10,
            Box::new(WindowsFinderMessageReceiver {
                state: false,
                currect_wnd: None,
            }),
        );
        Ok(Self(view))
    }
}
pub enum WindowFinderMsgType<'a> {
    Begin,
    SelChanged(Window),
    End,
}
use ButtonState::*;
pub use WindowFinderMsgType::*;
pub struct WindowFinderMsg<'a>(Window, WindowFinderMsgType<'a>);
impl MessageReceiver for WindowsFinderMessageReceiver {
    fn mouse_msg(
        &mut self,
        _id: usize,
        window: &mut Window,
        msg: MouseMsg,
    ) -> MessageReceiverResult<()> {
        match msg {
            MouseMsg::Move { mtype, is_nc } => {
                if !is_nc && self.state {
                    match mtype {
                        MouseMsgMoveType::Move(point) => {
                            let wnd_point =
                                Window::from_screen_point(point.window_to_screen(window));
                            if Some(wnd_point) != self.currect_wnd {
                                erase_window_border(self.currect_wnd);
                                self.currect_wnd = Some(wnd_point);
                                window.send_to(
                                    window.parent()?,
                                    WindowFinderMsg(window.copy_handle(), SelChanged),
                                );
                                invert_window(self.currect_wnd);
                            }
                            Ok(())
                        }
                        _ => Err(NoProcessed),
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
                                if window.send_to_result(
                                    window.parent()?,
                                    WindowFinderMsg(copy_handle.clone(), Begin),
                                ) < 0
                                //返回大于或等于零表示允许继续查找
                                {
                                    return Err(NoProcessed);
                                };
                                self.state = true;
                                self.from_window(window)
                                    .change_content(ViewContent::Icon(*bitmap2));
                                self.currect_wnd = window;
                                invert_window(self.currect_wnd);
                                window.set_apture();
                                window.send_to(
                                    window.parent()?,
                                    WindowFinderMsg(window.clone(), SelChanged),
                                );
                            }
                            Up => {
                                if self.state {
                                    if let Some(c_wnd) = self.currect_wnd {
                                        self.state = false;
                                        {
                                            window.parent()?;
                                            InvertWindow(hwndCurrent, fShowHidden);
                                            ReleaseCapture();
                                            SetCursor(Cursor::from_system(
                                                SystemCursor::NormalSelection,
                                            ));
                                            self.from_window(window)
                                                .change_content(ViewContent::Icon(*bitmap1));
                                        };
                                        window.send_to(
                                            window.parent()?,
                                            WindowFinderMsg(window.clone(), End),
                                        );
                                    } else {
                                        Err(NoProcessed)
                                    }
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
