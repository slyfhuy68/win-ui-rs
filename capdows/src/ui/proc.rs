use super::*;
#[repr(C)]
#[allow(non_snake_case)]
struct NMHDRSTATIC {
    #[allow(non_snake_case)]
    nmhdr: NMHDR,
    #[allow(non_snake_case)]
    DC: HANDLE,
}
pub unsafe extern "system" fn window_proc(
    window_handle: HWND,
    msg: u32,
    param1: WPARAM,
    param2: LPARAM,
) -> LRESULT {
    unsafe {
        let user_callback_ptr = match get_proc(window_handle) {
            Ok(x) => x,
            Err(_) => {
                if msg == WM_NCCREATE {
                    let s = *(param2.0 as *mut CREATESTRUCTW);
                    let _ = set_proc(window_handle, s.lpCreateParams as *mut Box<CallBackObj>);
                    s.lpCreateParams as *mut Box<CallBackObj>
                } else {
                    return DefWindowProcW(window_handle, msg, param1, param2);
                }
            }
        };
        if user_callback_ptr.is_null() {
            return DefWindowProcW(window_handle, msg, param1, param2);
        }
        let user_callback_s = &mut *user_callback_ptr;
        let rusult = LRESULT(msg_handler(
            user_callback_s,
            window_handle,
            msg,
            param1,
            param2,
            0,
            windows_porc_default_handler,
        ));
        if msg == WM_DESTROY {
            let _ = remove_proc(window_handle);
            let _ = Box::from_raw(user_callback_ptr);
        };
        rusult
    }
}
#[inline]
fn remove_proc(wnd: HWND) -> Result<()> {
    match unsafe { RemovePropW(wnd, w!("MalibUserCallback"))?.0 } as usize {
        0 => Err(ERROR_NOT_PRESENT),
        _ => Ok(()),
    }
}
#[inline]
pub fn get_proc(wnd: HWND) -> Result<*mut Box<CallBackObj>> {
    match unsafe { GetPropW(wnd, w!("MalibUserCallback")).0 } as usize {
        0 => Err(ERROR_NOT_PRESENT),
        x => Ok(x as *mut Box<CallBackObj>),
    }
}
#[inline]
fn set_proc(wnd: HWND, value: *mut Box<CallBackObj>) -> Result<()> {
    unsafe {
        Ok(SetPropW(
            wnd,
            w!("MalibUserCallback"),
            Some(wHANDLE(value as *mut c_void)),
        )?)
    }
}
#[inline]
fn windows_porc_default_handler(p1: HWND, p2: u32, p3: usize, p4: isize) -> isize {
    unsafe { DefWindowProcW(p1, p2, WPARAM(p3), LPARAM(p4)).0 }
}
unsafe fn msg_handler(
    c: &mut Box<CallBackObj>,
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    callback_id: usize,
    default_handler: unsafe fn(HWND, u32, usize, isize) -> isize,
) -> isize {
    unsafe {
        let mut w = Window::from_handle(hwnd);
        let WPARAM(param1) = wparam;
        let LPARAM(param2) = lparam;
        use MessageReceiverError::*;
        let result = match msg {
            WM_CREATE => {
                let s = *(param2 as *mut CREATESTRUCTW);
                let (wc, _buffer) = {
                    let mut buffer = [0u16; 256];
                    let len = GetClassNameW(hwnd, &mut buffer) as usize;
                    if len == 0 {
                        (WindowClass::from_raw(s.lpszClass), None)
                    } else {
                        let mut vec = buffer[..len].to_vec();
                        vec.push(0);
                        (WindowClass::from_raw(PCWSTR(vec.as_ptr())), Some(vec))
                    }
                };
                let mut wtype = WindowType::from_data(
                    WINDOW_STYLE(s.style as u32),
                    s.dwExStyle,
                    s.hMenu,
                    s.hwndParent,
                );
                let result = match c.create(
                    callback_id,
                    &mut w,
                    &s.lpszName.to_string().unwrap_or(String::from("")),
                    wc,
                    HMODULE(s.hInstance.0).into(),
                    rect(s.x, s.y, s.cx, s.cy),
                    &mut wtype,
                ) {
                    Ok(x) => match x {
                        true => 0isize,
                        false => -1isize,
                    },
                    Err(NoProcessed) => default_handler(hwnd, msg, param1, param2),
                    Err(x) => {
                        callback_error(c, x);
                        -1isize
                    }
                };
                wtype.nullify_menu();
                result
            }
            WM_DESTROY => match c.destroy(callback_id, &mut w) {
                Ok(_) => 0isize,
                Err(NoProcessed) => default_handler(hwnd, msg, param1, param2),
                Err(x) => callback_error(c, x),
            },
            WM_COMMAND => {
                if param2 != 0 {
                    let param2e = param2;
                    let param1e = param1;
                    match c.control_message(
                        callback_id,
                        &mut w,
                        &mut RawMessage(WM_COMMAND, param1e, param2e),
                        (param1e & 0xffff) as WindowID,
                    ) {
                        Ok(x) => x,
                        Err(NoProcessed) => 0isize,
                        Err(x) => callback_error(c, x),
                    }
                } else {
                    let high = ((param1 >> 16) & 0xffff) as u8;
                    let low = (param1 & 0xffff) as u16;
                    match high {
                        0 => {
                            match c.menu_command(
                                callback_id,
                                &mut w,
                                MenuCommandMsgItemPos::CostomId(low as MenuItemID),
                            ) {
                                Ok(_) => 0,
                                Err(NoProcessed) => default_handler(hwnd, msg, param1, param2),
                                Err(x) => callback_error(c, x),
                            }
                        }
                        // 1 => ,//加速器
                        _ => default_handler(hwnd, msg, param1, param2),
                    }
                }
            }
            WM_NOTIFYFORMAT => {
                2isize //NFR_UNICODE
            }
            WM_MENUCOMMAND => {
                let mut hmenu = HMENU(param2 as *mut c_void);
                match c.menu_command(
                    callback_id,
                    &mut w,
                    MenuCommandMsgItemPos::Position(Menu::from_mut_ref(&mut hmenu), param1 as u16),
                ) {
                    Ok(_) => 0,
                    Err(NoProcessed) => default_handler(hwnd, msg, param1, param2),
                    Err(x) => callback_error(c, x),
                }
            }
            WM_NOTIFY => {
                let nmhdr_ptr = param2 as *mut NMHDR;
                match c.control_message(
                    callback_id,
                    &mut w,
                    &mut RawMessage(WM_NOTIFY, 0, nmhdr_ptr as isize),
                    (*nmhdr_ptr).idFrom as WindowID,
                ) {
                    Ok(x) => x,
                    Err(NoProcessed) => default_handler(hwnd, msg, param1, param2),
                    Err(x) => callback_error(c, x),
                }
            }
            WM_CTLCOLORSTATIC => {
                let mut nmhdr = NMHDRSTATIC {
                    nmhdr: NMHDR {
                        hwndFrom: HWND(param2 as *mut c_void),
                        idFrom: GetWindowLongW(HWND(param2 as *mut c_void), GWL_ID) as usize,
                        code: WM_CTLCOLORSTATIC,
                    },
                    DC: param1 as *mut c_void,
                };
                let nmhdr_ptr: *mut NMHDRSTATIC = &mut nmhdr;
                match c.control_message(
                    callback_id,
                    &mut w,
                    &mut RawMessage(WM_NOTIFY, 0, nmhdr_ptr as isize),
                    nmhdr.nmhdr.idFrom as WindowID,
                ) {
                    Ok(x) => x,
                    Err(NoProcessed) => default_handler(hwnd, msg, param1, param2),
                    Err(x) => callback_error(c, x),
                }
            }
            WM_NULL => match c.null_msg(callback_id, &mut w) {
                Ok(_) => 0,
                Err(NoProcessed) => default_handler(hwnd, msg, param1, param2),
                Err(x) => callback_error(c, x),
            },
            WM_LBUTTONDOWN => {
                match c.mouse_msg(
                    callback_id,
                    &mut w,
                    MouseMsg::Button {
                        button_type: MouseButton::Left,
                        state: ButtonState::Down,
                        is_nc: false,
                        pos: point2(
                            (param2 & 0xFFFF) as u16 as i32,
                            (param2 >> 16) as u16 as i32,
                        ),
                    },
                ) {
                    Ok(_) => 0,
                    Err(NoProcessed) => default_handler(hwnd, msg, param1, param2),
                    Err(x) => callback_error(c, x),
                }
            }
            WM_LBUTTONUP => {
                match c.mouse_msg(
                    callback_id,
                    &mut w,
                    MouseMsg::Button {
                        button_type: MouseButton::Left,
                        state: ButtonState::Up,
                        is_nc: false,
                        pos: point2(
                            (param2 & 0xFFFF) as u16 as i32,
                            (param2 >> 16) as u16 as i32,
                        ),
                    },
                ) {
                    Ok(_) => 0,
                    Err(NoProcessed) => default_handler(hwnd, msg, param1, param2),
                    Err(x) => callback_error(c, x),
                }
            }
            _ => default_handler(hwnd, msg, param1, param2),
        };
        w.nullify();
        result
    }
}
#[inline]
fn callback_error(cb: &mut Box<CallBackObj>, err: MessageReceiverError) -> isize {
    match cb.error_handler(err) {
        Ok(x) => x,
        Err(err) => err.code() as isize,
    }
}
pub unsafe extern "system" fn subclass_porc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    uidsubclass: usize,
    dwrefdata: usize,
) -> LRESULT {
    unsafe {
        let c = &mut *(dwrefdata as *mut Box<CallBackObj>);
        let rusult = LRESULT(msg_handler(
            c,
            hwnd,
            msg,
            wparam,
            lparam,
            uidsubclass,
            subclass_porc_default_handler,
        ));
        if msg == WM_DESTROY {
            let _ = Box::from_raw(dwrefdata as *mut Box<CallBackObj>);
            //删除子类化时也需要销毁
            //这里是窗口关闭时销毁
        };
        rusult
    }
}
#[inline]
fn subclass_porc_default_handler(p1: HWND, p2: u32, p3: usize, p4: isize) -> isize {
    unsafe { DefSubclassProc(p1, p2, WPARAM(p3), LPARAM(p4)).0 }
}
