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
        positioning::{DPIAwareDeviceUnit, DeviceUnit, DialogTemplateUnit, Point, Rect, Size},
        strings::{WideString, widestr},
        ui::{
            class::WindowClass,
            control::{Control, ControlMsg, ControlMsgType, DefaultNMHDR, NotifyMessage},
            core::ResourceID,
            font::{ControlFont, Font},
            image::{Bitmap, Cursor, Icon},
            menu::{
                Menu, MenuBar, MenuCheckShow, MenuItem, MenuItemDisabledState, MenuItemID,
                MenuItemPos, MenuItemShow,
            },
            msg::{
                MessageReceiver, MessageReceiverError, MessageReceiverResult, RawMessage, msg_loop,
                stop_msg_loop,
            },
            style::{
                ChildWindowStyles, ClassBackgroundBrush, NormalWindowStyles, WindowBorderType,
                WindowClassStyle, WindowSizeState, WindowType,
            },
            tools::{MessageBoxOptions, MessageBoxResult, msg_box},
            window::{ShowWindowType, Window},
        },
    };
}
