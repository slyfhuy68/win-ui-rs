use super::*;
pub unsafe extern "system" fn window_proc<C: RawMessageHandler + Sync + 'static>(
    window_handle: HWND,
    msg: u32,
    param1: WPARAM,
    param2: LPARAM,
) -> LRESULT {
    unsafe {
        match C::handle_msg(window_handle, msg, param1, param2) {
            Some(x) => x,
            None => DefWindowProcW(window_handle, msg, param1, param2),
        }
    }
}
pub unsafe extern "system" fn subclass_porc<
    const PORC_ID: usize,
    C: RawMessageHandler<SubPorc<PORC_ID>> + Sync + 'static,
>(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    _: usize, //uidsubclass
    _: usize, //dwrefdata
) -> LRESULT {
    unsafe {
        match C::handle_msg(hwnd, msg, wparam, lparam) {
            Some(x) => x,
            None => DefSubclassProc(hwnd, msg, wparam, lparam),
        }
    }
}
pub unsafe extern "system" fn dialog_porc<C: RawMessageHandler<DialogPorc> + Sync + 'static>(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match C::handle_msg(hwnd, msg, wparam, lparam) {
            Some(x) => x,
            None => DefDialogPorc(hwnd, msg, wparam, lparam),
        }
    }
}
#[allow(non_snake_case)]
pub fn DefDialogPorc(window_handle: HWND, msg: u32, param1: WPARAM, param2: LPARAM) -> LRESULT {
    match msg {
        WM_CHARTOITEM | WM_VKEYTOITEM => -1,
        // WM_COMPAREITEM => 0,
        WM_INITDIALOG => 1,
        // WM_QUERYDRAGICON => 0,
        //这些消息是自定义绘制控件时用到的，与对话框管理器无关，可以直接调用DefWindowProc
        WM_CTLCOLORBTN | WM_CTLCOLORDLG | WM_CTLCOLOREDIT | WM_CTLCOLORLISTBOX
        | WM_CTLCOLORSCROLLBAR | WM_CTLCOLORSTATIC => unsafe {
            DefWindowProcW(window_handle, msg, param1, param2)
        },
        _ => 0,
    }
}
