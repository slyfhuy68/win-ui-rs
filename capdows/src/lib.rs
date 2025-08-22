#[cfg(not(target_os = "windows"))]
compile_error!("This crate only support windows target");
pub mod error;
pub mod i18n;
pub mod positioning;
pub mod strings;
pub mod ui;
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{
        error::WinError as Error,
        error::*,
        i18n::LangID,
        positioning::{DPIAwareDeviceUnit, DeviceUnit, FontUnit, Point, Rect, Size},
        strings::{CWideStr, L, Lc, WideString, widestr},
        ui::{
            class::{WindowClass, WindowClassBuilder},
            control::{Control, ControlMsg, ControlMsgType, DefaultNMHDR, NotifyMessage},
            core::*,
            dialog::Dialog,
            font::{ControlFont, Font},
            image::{Bitmap, Cursor, Icon},
            menu::{
                Menu, MenuBar, MenuCheckIcon, MenuCheckShow, MenuItem, MenuItemDisabledState,
                MenuItemID, MenuItemPos, MenuItemShow, MenuItemStyle,
            },
            module::ExecutableFile,
            msg::{
                DialogMessageReceiver, DialogPorc, MainPorc, MenuCommandMsgItemPos,
                MessageReceiver, MessageReceiverError, MessageReceiverResult, RawMessage,
                StaticMsg, SubPorc, msg_loop, stop_msg_loop,
            },
            style::{
                ChildWindowStyles, ClassBackgroundBrush, NormalWindowStyles, WindowBorderType,
                WindowClassStyle, WindowSizeState, WindowType,
            },
            tools::{MessageBoxOptions, MessageBoxResult, msg_box},
            window::{ShowWindowType, Window, WindowID},
        },
    };
    #[doc(no_inline)]
    pub use std::os::windows::{prelude::*, raw::HANDLE};
}
pub mod utilities {
    #[doc(inline)]
    pub use capdows_utility::{do_escapes, icontain, runtime_fmt, set_istyle, set_style, ucontain};
}
