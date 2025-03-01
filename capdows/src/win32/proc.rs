use super::*;
pub unsafe extern "system" fn window_proc(
    window_handle: HWND,
    msg: u32,
    param1: WPARAM,
    param2: LPARAM,
) -> LRESULT {
    unsafe {
        let mut window = Window {
            handle: window_handle,
        };
        let user_callback_ptr = match get_proc(&window) {
            Ok(x) => x,
            Err(_) => {
                if msg == WM_NCCREATE {
                    let s = *(param2.0 as *mut CREATESTRUCTW);
                    let mm = set_proc(&mut window, s.lpCreateParams as *mut CallBackObj);
                    s.lpCreateParams as *mut CallBackObj
                } else {
                    return DefWindowProcW(window_handle, msg, param1, param2);
                }
            }
        };
        if user_callback_ptr.is_null() {
            return DefWindowProcW(window_handle, msg, param1, param2);
        }
        let user_callback_s = Box::from_raw(user_callback_ptr);
        // if user_callback_s.0 != PROC_MEMORY_SINGS {
        //     return DefWindowProcW(window_handle, msg, param1, param2);
        // };
        let mut c = user_callback_s;
        pub use MessageReceiverError::*;
        let result = {
            let mut w = window;
            match msg {
                //unimplemented!()
                //----------------------------------------------------------------------------------
                // WM_ACTIVATEAPP => {},
                // WM_CANCELMODE => {},
                // WM_CHILDACTIVATE => {},
                // WM_CLOSE => {
                //     mstch user_callback_s {
                //
                //     }
                // },
                // WM_COMPACTING => {},
                WM_CREATE => {
                    let s = *(param2.0 as *mut CREATESTRUCTW);
                    let wc = w.get_class();
                    match c.create(
                        &mut w,
                        &s.lpszName.to_string().unwrap_or(String::from("")),
                        match wc {
                            Err(_) => WindowClass {
                                name: None,
                                atom: s.lpszClass,
                                handle_instance: None,
                            },
                            Ok(x) => x,
                        },
                        ExecutableFile {
                            name: None,
                            handle: Some(HMODULE(s.hInstance.0)),
                        },
                        ((s.x, s.y), s.cx, s.cy),
                        (
                            WINDOW_STYLE(s.style as u32),
                            s.dwExStyle,
                            if s.hMenu.is_invalid() {
                                None
                            } else {
                                Some(s.hMenu)
                            },
                            if s.hwndParent.is_invalid() {
                                None
                            } else {
                                Some(s.hwndParent)
                            },
                        )
                            .into(),
                    ) {
                        Ok(x) => match x {
                            true => 0isize,
                            false => -1isize,
                        },
                        Err(NoProcessed) => DefWindowProcW(window_handle, msg, param1, param2).0,
                        Err(x) => callback_error(x),
                    }
                }
                WM_DESTROY => {
                    //这里的return不要删，作用是防止回调对象被变成原始指针，销毁窗口时，应该销毁回调对象
                    return LRESULT(match c.destroy(&mut w) {
                        Ok(_) => 0isize,
                        Err(NoProcessed) => DefWindowProcW(window_handle, msg, param1, param2).0,
                        Err(x) => callback_error(x),
                    });
                }
                WM_COMMAND if param2.0 != 0 => {
                    let param2e = param2.0;
                    let param1e = param1.0;
                    let mut nmhdr = NMHDR {
                        hwndFrom: HWND(param2e as *mut c_void),
                        idFrom: (param1e & 0xffff) as usize,
                        code: ((param1e >> 16) & 0xffff) as u32,
                    };
                    let nmhdr_ptr: *mut NMHDR = &mut nmhdr;
                    match c.control_message(&mut w, nmhdr_ptr as usize, nmhdr.idFrom as WindowID) {
                        Ok(x) => x,
                        Err(NoProcessed) => DefWindowProcW(window_handle, msg, param1, param2).0,
                        Err(x) => callback_error(x),
                    }
                }
                WM_NOTIFYFORMAT => {
                    2isize //NFR_UNICODE
                }
                WM_NOTIFY => {
                    let nmhdr_ptr = param2.0 as *mut NMHDR;
                    match c.control_message(
                        &mut w,
                        nmhdr_ptr as usize,
                        (*nmhdr_ptr).idFrom as WindowID,
                    ) {
                        Ok(x) => x,
                        Err(NoProcessed) => DefWindowProcW(window_handle, msg, param1, param2).0,
                        Err(x) => callback_error(x),
                    }
                }
                // WM_DPICHANGED => {},
                // WM_ENABLE => {},
                // WM_ENTERSIZEMOVE => {},
                // WM_EXITSIZEMOVE => {},
                // WM_GETICON => {},
                // WM_GETMINMAXINFO => {},
                // WM_INPUTLANGCHANGE => {},
                // WM_INPUTLANGCHANGEREQUEST => {},
                // WM_MOVE => {},
                // WM_MOVING => {},
                // WM_NCACTIVATE => {},
                // WM_NCCALCSIZE => {},
                // WM_NCCREATE => {},
                // WM_NCDESTROY => {},
                // WM_NULL => {},
                // WM_QUERYDRAGICON => {},
                // WM_QUERYOPEN => {},
                // WM_SHOWWINDOW => {},
                // WM_SIZE => {},
                // WM_SIZING => {},
                // WM_STYLECHANGED => {},
                // WM_STYLECHANGING => {},
                // WM_THEMECHANGED => {},
                // WM_USERCHANGED => {},
                // WM_WINDOWPOSCHANGED => {},
                // WM_WINDOWPOSCHANGING => {},
                //----------------------------------------------------------------------------------
                _ => {
                    //println!("msg:{}", msg);
                    DefWindowProcW(window_handle, msg, param1, param2).0
                }
            }
        };
        let _ = Box::into_raw(c);
        LRESULT(result)
    }
}
fn set_proc(wnd: &mut Window, ptr: *mut CallBackObj) -> Result<()> {
    wnd.set_prop(PROC_KEY_NAME, ptr as usize)
}
fn get_proc(wnd: &Window) -> Result<*mut CallBackObj> {
    match wnd.get_prop(PROC_KEY_NAME) {
        Ok(x) => Ok(x as *mut CallBackObj),
        Err(x) => Err(x),
    }
}
fn callback_error(err:MessageReceiverError) -> isize {
    println!("{:?}", err);
    err.code() as isize
}