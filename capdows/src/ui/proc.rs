use super::*;
pub unsafe extern "system" fn window_proc<C: RawMessageHandler + Sync + 'static>(
    window_handle: HWND,
    msg: u32,
    param1: WPARAM,
    param2: LPARAM,
) -> LRESULT {
    unsafe {
        match C::msg_handler(window_handle, msg, param1, param2) {
            Some(x) => x,
            None => DefWindowProcW(window_handle, msg, param1, param2),
        }
    }
}
pub unsafe extern "system" fn subclass_porc<C: RawMessageHandler + Sync + 'static>(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    uidsubclass: usize,
    dwrefdata: usize,
) -> LRESULT {
    unsafe {
        match C::msg_handler(
            hwnd,
            msg,
            wparam,
            lparam,
            uidsubclass,
            dwrefdata as *mut c_void,
        ) {
            Some(x) => x,
            None => DefSubclassProc(hwnd, msg, wparam, lparam),
        }
    }
}
