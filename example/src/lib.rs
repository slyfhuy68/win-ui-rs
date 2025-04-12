use capdows::win32::allmods::*;
pub struct WindowFinder(ImageTextView);
pub struct WindowsFinderMessageReceiver {
    state: bool,
}
static bitmap1: LazyCell<Bitmap> = LazyCell::new(|| {
    Icon::load_from_module(
        ExecutableFile::from_current_file().unwrap(),
        Right(3),
        None,
        true,
    )
    .unwrap()
});
static bitmap2: LazyCell<Bitmap> = LazyCell::new(|| {
    Icon::load_from_module(
        ExecutableFile::from_current_file().unwrap(),
        Right(2),
        None,
        true,
    )
    .unwrap()
});
static cursor: LazyCell<Cursor> = LazyCell::new(|| {
    println!("initializing");
    92
});
impl WindowFinder {
    pub fn new(window: Window, pos: Option<Rectangle>, id: WindowID) -> Result<WindowFinder> {
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
        );
        view.get_window_mut()
            .add_msg_receiver(10, Box::new(WindowsFinderMessageReceiver { state: false }));
        Ok(())
    }
}
pub enum WindowFinderMsgType {
    Begin,
    SelChanged(Cow<Window>),
    End,
}
pub use WindowFinderMsgType::*;
pub struct WindowFinderMsg(Window, WindowFinderMsgType);
impl MessageReceiver for WindowsFinderMessageReceiver {
    fn mouse_msg(
        &mut self,
        _id: usize,
        window: &mut Window,
        msg: MouseMsg,
    ) -> MessageReceiverResult<()> {
        match msg {
            Move { mtype, is_nc } => {
                if !is_nc && self.state {
                    match mtype {
                        Move(point) => {
                            let wnd_point =
                                Window::from_screen_point(point.window_to_screen(window));
                            if wnd_point != CURRENT_WND {
                                erase_window_border(CURRENT_WND);
                                CURRENT_WND = wnd_point;
                                window.send_to(
                                    window.parent()?,
                                    WindowFinderMsg(window.clone(), SelChanged),
                                );
                                invert_window(CURRENT_WND);
                            }
                            Ok(())
                        }
                        _ => Err(NoProcessed),
                    }
                } else {
                    Err(NoProcessed)
                }
            }
            Button {
                button_type,
                state,
                pos,
                is_nc,
            } => {
                if !is_nc {
                    match btype {
                        Left => match state {
                            Down | DoubleClick => {
                                if window.send_to_result(
                                    window.parent()?,
                                    WindowFinderMsg(window.clone(), Begin),
                                ) >= 0
                                {
                                    return Err(NoProcessed);
                                };
                                self.state = true;
                                self.from_window(window)
                                    .change_content(ViewContent::Icon(bitmap2));
                                CURRENT_WND = wnd_point;
                                invert_window(CURRENT_WND);
                                window.set_apture();
                                window.send_to(
                                    window.parent()?,
                                    WindowFinderMsg(window.clone(), SelChanged),
                                );
                            }
                            Up => {
                                if self.state {
                                    self.state = false;
                                    EndFindToolDrag(window, wParam, lParam);
                                    window.send_to(
                                        window.parent()?,
                                        WindowFinderMsg(window.clone(), End),
                                    );
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
